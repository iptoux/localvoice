use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use uuid::Uuid;

use crate::db::models::Session;
use crate::db::repositories::{
    ambiguous_terms_repo, dictionary_repo, filler_words_repo, models_repo, sessions_repo,
    settings_repo,
};
use crate::dictionary::service as dict_service;
use crate::errors::CmdResult;
use crate::os::{clipboard, foreground_window, text_insertion};
use crate::state::app_state::emit_recording_state;
use crate::state::recording_state::RecordingState;
use crate::state::AppState;
use crate::transcription::engine::{
    ModelRuntime, ENGINE_NEMO, ENGINE_PARAKEET_CPP, ENGINE_WHISPER_CPP, FORMAT_GGML_BIN,
    FORMAT_GGUF, FORMAT_NEMO, RUNTIME_BUNDLED_SIDECAR, RUNTIME_EXTERNAL_PATH,
    RUNTIME_OPTIONAL_NEMO,
};
use crate::transcription::types::{OutputResult, TranscriptionResult};
use crate::transcription::{
    language, nemo_worker, parakeet_parser, parakeet_sidecar, parser, pipeline, whisper_sidecar,
};

struct EngineTranscript {
    raw_text: String,
    segments: Vec<crate::transcription::types::TranscriptSegment>,
    words: Vec<crate::transcription::types::TranscriptWord>,
}

/// Runs the full transcription pipeline synchronously.
///
/// The model override may be either an installed model key or a direct model
/// path. The final output path stays unchanged for all engines: raw engine text
/// is post-processed, persisted, and emitted through the existing pipeline.
pub fn transcribe(
    app: &AppHandle,
    wav_path: &str,
    language_override: Option<&str>,
    model_override: Option<&str>,
) -> CmdResult<TranscriptionResult> {
    let state = app.state::<AppState>();
    let settings = settings_repo::get_all(&state.db).unwrap_or_default();

    let lang_code = language_override
        .map(|s| s.to_string())
        .or_else(|| settings.get("transcription.default_language").cloned())
        .unwrap_or_else(|| "auto".to_string());

    let model_runtime = resolve_model_runtime(app, &lang_code, model_override, &settings)?;

    let wav_path_buf = std::path::PathBuf::from(wav_path);
    let output_prefix = wav_path_buf
        .parent()
        .unwrap_or(std::path::Path::new(
            std::env::temp_dir().to_str().unwrap_or("."),
        ))
        .join(format!("localvoice_out_{}", uuid::Uuid::new_v4()));

    let start = Instant::now();
    let engine_output = match model_runtime.engine.as_str() {
        ENGINE_WHISPER_CPP => run_whisper(
            app,
            &model_runtime.local_path,
            wav_path,
            &language::to_whisper_lang(&lang_code),
            &output_prefix,
        )?,
        ENGINE_PARAKEET_CPP => run_parakeet_cpp(
            app,
            &model_runtime.local_path,
            wav_path,
            &language::to_parakeet_locale(&lang_code),
            settings
                .get("transcription.parakeet.device")
                .map(String::as_str),
        )?,
        ENGINE_NEMO => run_nemo(
            app,
            &model_runtime.local_path,
            wav_path,
            &language::to_parakeet_locale(&lang_code),
            settings
                .get("transcription.nemo.python_path")
                .map(String::as_str),
        )?,
        other => return Err(format!("Unsupported transcription engine: {other}").into()),
    };
    let duration_ms = start.elapsed().as_millis() as u64;

    let raw_text = engine_output.raw_text;
    let active_rules =
        dictionary_repo::list_active_rules(&state.db, Some(&lang_code)).unwrap_or_default();
    let filler_words =
        filler_words_repo::list_words_for_language(&state.db, &lang_code).unwrap_or_default();

    let (cleaned_text, cleaned_segments, fired_rule_ids, removed_fillers) = pipeline::run(
        &raw_text,
        engine_output.segments,
        &settings,
        &active_rules,
        &lang_code,
        &filler_words,
    );

    if !fired_rule_ids.is_empty() {
        if let Err(e) = dict_service::record_rule_usage(&state.db, &fired_rule_ids) {
            log::warn!("Failed to record rule usage: {e}");
        }
    }

    Ok(TranscriptionResult {
        raw_text,
        cleaned_text,
        segments: cleaned_segments,
        words: engine_output.words,
        language: lang_code,
        model_id: model_runtime.model_key,
        engine: model_runtime.engine,
        artifact_format: model_runtime.artifact_format,
        runtime: model_runtime.runtime,
        duration_ms,
        output: None,
        removed_fillers,
    })
}

/// Runs the full transcription pipeline synchronously.
///
/// - Reads language and model path from `AppState` settings.
/// - `language_override` / `model_path_override` let callers bypass settings.
/// - Returns a complete `TranscriptionResult` with `output` set to `None`;
///   the output step is performed by [`transcribe_and_emit`].
#[allow(dead_code)]
fn transcribe_legacy_whisper(
    app: &AppHandle,
    wav_path: &str,
    language_override: Option<&str>,
    model_path_override: Option<&str>,
) -> CmdResult<TranscriptionResult> {
    let state = app.state::<AppState>();

    // Read relevant settings from the DB.
    let settings = settings_repo::get_all(&state.db).unwrap_or_default();

    let lang_code = language_override
        .map(|s| s.to_string())
        .or_else(|| settings.get("transcription.default_language").cloned())
        .unwrap_or_else(|| "auto".to_string());
    let whisper_lang = language::to_whisper_lang(&lang_code).to_string();

    // Resolve model path: explicit override → DB default for language → auto fallback → settings path → auto-scan.
    let effective_model_override = if let Some(p) = model_path_override {
        Some(p.to_string())
    } else {
        // Try the DB-registered default for this language first.
        let db_default = models_repo::get_default_path(&state.db, &lang_code).unwrap_or(None);
        if db_default.is_some() {
            db_default
        } else if lang_code == "auto" {
            // For "auto" language, fall back to any configured default model.
            models_repo::get_any_default_path(&state.db)
                .unwrap_or(None)
                .or_else(|| settings.get("transcription.model_path").cloned())
        } else {
            // Fall back to the legacy settings key.
            settings.get("transcription.model_path").cloned()
        }
    };

    let binary = whisper_sidecar::resolve_binary(app)?;
    let model = whisper_sidecar::resolve_model(app, effective_model_override.as_deref())?;

    let model_id = model
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Output JSON prefix — same directory as the WAV file, unique name.
    let wav_path_buf = std::path::PathBuf::from(wav_path);
    let output_prefix = wav_path_buf
        .parent()
        .unwrap_or(std::path::Path::new(
            std::env::temp_dir().to_str().unwrap_or("."),
        ))
        .join(format!("localvoice_out_{}", uuid::Uuid::new_v4()));

    let start = Instant::now();

    let output = whisper_sidecar::invoke(
        &binary,
        &model,
        std::path::Path::new(wav_path),
        &whisper_lang,
        &output_prefix,
    )?;

    let duration_ms = start.elapsed().as_millis() as u64;

    // Parse segments — prefer JSON (has confidence scores), fall back to stdout.
    let segments = output
        .json_path
        .as_deref()
        .and_then(parser::parse_json_file)
        .unwrap_or_else(|| parser::parse_stdout(&output.stdout));

    // Clean up the temporary JSON file if it was written.
    if let Some(json_path) = &output.json_path {
        let _ = std::fs::remove_file(json_path);
    }
    // NOTE: WAV cleanup / persistence is handled by the caller (transcribe_and_emit).

    let raw_text = parser::segments_to_text(&segments);

    // Load active correction rules for the current language.
    let active_rules =
        dictionary_repo::list_active_rules(&state.db, Some(&lang_code)).unwrap_or_default();

    // Load filler words for the current language from DB.
    let filler_words =
        filler_words_repo::list_words_for_language(&state.db, &lang_code).unwrap_or_default();

    let (cleaned_text, cleaned_segments, fired_rule_ids, removed_fillers) = pipeline::run(
        &raw_text,
        segments,
        &settings,
        &active_rules,
        &lang_code,
        &filler_words,
    );

    // Increment usage counters for rules that fired.
    if !fired_rule_ids.is_empty() {
        if let Err(e) = dict_service::record_rule_usage(&state.db, &fired_rule_ids) {
            log::warn!("Failed to record rule usage: {e}");
        }
    }

    Ok(TranscriptionResult {
        raw_text,
        cleaned_text,
        segments: cleaned_segments,
        words: Vec::new(),
        language: lang_code,
        model_id,
        engine: ENGINE_WHISPER_CPP.to_string(),
        artifact_format: FORMAT_GGML_BIN.to_string(),
        runtime: RUNTIME_BUNDLED_SIDECAR.to_string(),
        duration_ms,
        output: None,
        removed_fillers,
    })
}

fn resolve_model_runtime(
    app: &AppHandle,
    lang_code: &str,
    model_override: Option<&str>,
    settings: &std::collections::HashMap<String, String>,
) -> CmdResult<ModelRuntime> {
    let state = app.state::<AppState>();

    if let Some(override_value) = model_override.filter(|s| !s.is_empty()) {
        if let Some(model) = models_repo::get(&state.db, override_value)? {
            return Ok(ModelRuntime::from_registry(
                &model.model_key,
                model.local_path.clone(),
            ));
        }

        let path = std::path::PathBuf::from(override_value);
        let model_key = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("external-model")
            .to_string();
        return Ok(runtime_from_path(&model_key, override_value));
    }

    let default_model = if lang_code == "auto" {
        models_repo::get_any_default_model(&state.db)?
    } else {
        models_repo::get_default_model(&state.db, lang_code)?
    };
    if let Some(model) = default_model {
        return Ok(ModelRuntime::from_registry(
            &model.model_key,
            model.local_path.clone(),
        ));
    }

    if let Some(path) = settings
        .get("transcription.model_path")
        .filter(|path| !path.is_empty())
    {
        let model_key = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("legacy-model")
            .to_string();
        return Ok(runtime_from_path(&model_key, path));
    }

    Err("No transcription model configured. Download a model and set it as default.".into())
}

fn runtime_from_path(model_key: &str, path: &str) -> ModelRuntime {
    let lower = path.to_lowercase();
    if lower.ends_with(".gguf") {
        ModelRuntime {
            model_key: model_key.to_string(),
            display_name: model_key.to_string(),
            local_path: path.to_string(),
            engine: ENGINE_PARAKEET_CPP.to_string(),
            artifact_format: FORMAT_GGUF.to_string(),
            runtime: RUNTIME_EXTERNAL_PATH.to_string(),
            supports_streaming: true,
            supports_word_timestamps: true,
            supports_confidence: true,
        }
    } else if lower.ends_with(".nemo") {
        ModelRuntime {
            model_key: model_key.to_string(),
            display_name: model_key.to_string(),
            local_path: path.to_string(),
            engine: ENGINE_NEMO.to_string(),
            artifact_format: FORMAT_NEMO.to_string(),
            runtime: RUNTIME_OPTIONAL_NEMO.to_string(),
            supports_streaming: true,
            supports_word_timestamps: true,
            supports_confidence: true,
        }
    } else {
        ModelRuntime::legacy_whisper(model_key, path.to_string())
    }
}

fn run_whisper(
    app: &AppHandle,
    model_path: &str,
    wav_path: &str,
    language: &str,
    output_prefix: &std::path::Path,
) -> CmdResult<EngineTranscript> {
    let binary = whisper_sidecar::resolve_binary(app)?;
    let model = whisper_sidecar::resolve_model(app, Some(model_path))?;
    let output = whisper_sidecar::invoke(
        &binary,
        &model,
        std::path::Path::new(wav_path),
        language,
        output_prefix,
    )?;
    let segments = output
        .json_path
        .as_deref()
        .and_then(parser::parse_json_file)
        .unwrap_or_else(|| parser::parse_stdout(&output.stdout));

    if let Some(json_path) = &output.json_path {
        let _ = std::fs::remove_file(json_path);
    }

    let raw_text = parser::segments_to_text(&segments);
    Ok(EngineTranscript {
        raw_text,
        segments,
        words: Vec::new(),
    })
}

fn run_parakeet_cpp(
    app: &AppHandle,
    model_path: &str,
    wav_path: &str,
    language: &str,
    device: Option<&str>,
) -> CmdResult<EngineTranscript> {
    let binary = parakeet_sidecar::resolve_binary(app)?;
    let model = parakeet_sidecar::resolve_model(Some(model_path))?;
    let output = parakeet_sidecar::invoke(
        &binary,
        &model,
        std::path::Path::new(wav_path),
        language,
        device,
    )?;

    if let Some(parsed) = parakeet_parser::parse_stdout(&output.stdout) {
        return Ok(EngineTranscript {
            raw_text: parsed.text,
            segments: parsed.segments,
            words: parsed.words,
        });
    }

    let text = output.stdout.trim().to_string();
    Ok(EngineTranscript {
        raw_text: text.clone(),
        segments: vec![crate::transcription::types::TranscriptSegment {
            start_ms: 0,
            end_ms: 0,
            text,
            confidence: None,
        }],
        words: Vec::new(),
    })
}

fn run_nemo(
    app: &AppHandle,
    model_path: &str,
    wav_path: &str,
    language: &str,
    python_path: Option<&str>,
) -> CmdResult<EngineTranscript> {
    let transcript = nemo_worker::transcribe_file(
        app,
        std::path::Path::new(model_path),
        std::path::Path::new(wav_path),
        language,
        python_path,
    )?;
    Ok(EngineTranscript {
        raw_text: transcript.text,
        segments: transcript.segments,
        words: transcript.words,
    })
}

/// Entry point for the background transcription task.
///
/// Called from a `tokio::task::spawn_blocking` closure after `stop_recording`.
/// After transcription:
/// 1. Runs the output step (clipboard write or auto-insert).
/// 2. Emits `output-result` and `transcription-completed` events.
/// 3. Transitions pill to Success (or Error on failure) then auto-resets to
///    Idle after 2 s (success) / 3 s (error).
pub fn transcribe_and_emit(app: AppHandle, wav_path: String) {
    // ── Pre-flight: verify a model is explicitly set for the selected language ─
    {
        let state = app.state::<AppState>();
        let settings = settings_repo::get_all(&state.db).unwrap_or_default();
        let lang_code = settings
            .get("transcription.default_language")
            .cloned()
            .unwrap_or_else(|| "auto".to_string());

        // For "auto" we still need *any* installed model — check via resolve_model.
        // For a specific language, a model must be explicitly assigned to that language.
        let model_ok = if lang_code == "auto" {
            // auto: just needs any model to exist (resolve_model will find it or error)
            // We let the normal transcribe() path handle this case.
            true
        } else {
            models_repo::get_default_path(&state.db, &lang_code)
                .unwrap_or(None)
                .is_some()
                || settings
                    .get("transcription.model_path")
                    .map(|path| !path.is_empty())
                    .unwrap_or(false)
        };

        if !model_ok {
            let _ = std::fs::remove_file(&wav_path);
            let msg = format!(
                "No model is set for language \"{lang_code}\". \
                 Go to Settings → Models and assign a model for \"{lang_code}\", \
                 or switch the language to Auto-detect."
            );
            log::warn!("{msg}");
            let _ = app
                .notification()
                .builder()
                .title("LocalVoice — No Model Set")
                .body(&msg)
                .show();
            emit_recording_state(
                &app,
                RecordingState::Error,
                Some(format!(
                    "No model for \"{lang_code}\". Go to Settings → Models."
                )),
            );
            schedule_idle_reset(app, Duration::from_millis(4000));
            return;
        }

        // Secondary check: if the assigned model is en-only but language is not en/auto.
        if lang_code != "auto" && lang_code != "en" {
            if let Ok(Some(path)) = models_repo::get_default_path(&state.db, &lang_code) {
                let model_key = std::path::Path::new(&path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let is_en_only = models_repo::list_installed(&state.db)
                    .unwrap_or_default()
                    .iter()
                    .any(|m| m.model_key == model_key && m.language_scope == "en-only");
                if is_en_only {
                    let _ = std::fs::remove_file(&wav_path);
                    let msg = format!(
                        "The model assigned to \"{lang_code}\" only supports English. \
                         Please assign a multilingual model in Settings → Models."
                    );
                    log::warn!("{msg}");
                    let _ = app
                        .notification()
                        .builder()
                        .title("LocalVoice — Wrong Model")
                        .body(&msg)
                        .show();
                    emit_recording_state(
                        &app,
                        RecordingState::Error,
                        Some(format!(
                            "Model for \"{lang_code}\" is English-only. Check Settings → Models."
                        )),
                    );
                    schedule_idle_reset(app, Duration::from_millis(4000));
                    return;
                }
            }
        }
    }

    match transcribe(&app, &wav_path, None, None) {
        Ok(mut result) => {
            // ── Output step ──────────────────────────────────────────────────
            let state = app.state::<AppState>();
            let settings = settings_repo::get_all(&state.db).unwrap_or_default();
            let output_mode = settings
                .get("output.mode")
                .cloned()
                .unwrap_or_else(|| "clipboard".to_string());

            let insert_delay_ms: u64 = settings
                .get("output.insert_delay_ms")
                .and_then(|v| v.parse().ok())
                .unwrap_or(100);

            // Detect the foreground window before pasting into it.
            let target_app = if output_mode == "insert" {
                foreground_window::get_foreground_window_title()
            } else {
                None
            };

            let output_result = perform_output(&result.cleaned_text, &output_mode, insert_delay_ms);

            // Emit dedicated output-result event (TASK-057).
            if let Err(e) = app.emit("output-result", &output_result) {
                log::error!("Failed to emit output-result: {e}");
            }

            result.output = Some(output_result.clone());

            // ── Optionally keep audio for reprocessing (TASK-204) ─────────────
            let persisted_audio_path = {
                let keep = settings
                    .get("recording.keep_audio")
                    .map(|v| v == "true")
                    .unwrap_or(false);
                if keep {
                    match persist_audio_file(&app, &wav_path) {
                        Ok(p) => Some(p),
                        Err(e) => {
                            log::warn!("Failed to persist audio file: {e}");
                            let _ = std::fs::remove_file(&wav_path);
                            None
                        }
                    }
                } else {
                    let _ = std::fs::remove_file(&wav_path);
                    None
                }
            };

            // ── Persist session to DB ─────────────────────────────────────────
            let now = chrono::Utc::now();
            let started_at = state
                .recording_started_at
                .lock()
                .unwrap()
                .take()
                .unwrap_or(now);
            let duration_ms = (now - started_at).num_milliseconds();
            let word_count = result.cleaned_text.split_whitespace().count() as i64;
            let char_count = result.cleaned_text.chars().count() as i64;
            let avg_confidence = {
                let conf_vals: Vec<f64> = result
                    .segments
                    .iter()
                    .filter_map(|s| s.confidence.map(|c| c as f64))
                    .collect();
                if conf_vals.is_empty() {
                    None
                } else {
                    Some(conf_vals.iter().sum::<f64>() / conf_vals.len() as f64)
                }
            };
            let estimated_wpm = if duration_ms > 0 {
                Some(word_count as f64 / (duration_ms as f64 / 60_000.0))
            } else {
                None
            };
            let session = Session {
                id: Uuid::new_v4().to_string(),
                started_at: started_at.to_rfc3339(),
                ended_at: now.to_rfc3339(),
                duration_ms,
                language: result.language.clone(),
                model_id: Some(result.model_id.clone()),
                engine: result.engine.clone(),
                model_artifact_format: result.artifact_format.clone(),
                runtime: result.runtime.clone(),
                trigger_type: "shortcut".to_string(),
                input_device_id: settings.get("recording.device_id").cloned(),
                raw_text: result.raw_text.clone(),
                cleaned_text: result.cleaned_text.clone(),
                word_count,
                char_count,
                avg_confidence,
                estimated_wpm,
                output_mode: output_mode.clone(),
                output_target_app: target_app,
                inserted_successfully: output_result.success,
                error_message: output_result.error.clone(),
                created_at: now.to_rfc3339(),
                audio_path: persisted_audio_path,
                original_raw_text: None,
                reprocessed_count: 0,
                original_model_id: None,
                original_language: None,
                original_avg_confidence: None,
            };
            if let Err(e) = sessions_repo::insert_session(&state.db, &session) {
                log::error!("Failed to persist session: {e}");
            } else if let Err(e) =
                sessions_repo::insert_segments(&state.db, &session.id, &result.segments)
            {
                log::error!("Failed to persist session segments: {e}");
            } else if let Err(e) =
                sessions_repo::insert_words(&state.db, &session.id, &result.words)
            {
                log::error!("Failed to persist session words: {e}");
            } else {
                log::info!(
                    "Session {} persisted ({} segments, {} words)",
                    session.id,
                    result.segments.len(),
                    result.words.len()
                );
            }

            // Log filler word removals for stats tracking.
            if !result.removed_fillers.is_empty() {
                if let Err(e) = filler_words_repo::log_removals(
                    &state.db,
                    Some(&session.id),
                    &result.removed_fillers,
                    &result.language,
                ) {
                    log::warn!("Failed to log filler removals: {e}");
                }
            }

            // ── Ambiguity detection ───────────────────────────────────────────
            let conf_threshold: f32 = settings
                .get("ambiguity.confidence_threshold")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.6);
            let candidates =
                crate::postprocess::ambiguity::detect(&result.segments, conf_threshold);
            for c in &candidates {
                if let Err(e) = ambiguous_terms_repo::upsert(
                    &state.db,
                    &c.phrase,
                    Some(&result.language),
                    c.confidence,
                ) {
                    log::warn!("Failed to upsert ambiguous term '{}': {e}", c.phrase);
                }
            }
            if !candidates.is_empty() {
                if let Err(e) = crate::dictionary::suggestions::apply_suggestions(&state.db) {
                    log::warn!("Failed to apply ambiguity suggestions: {e}");
                }
            }

            // Store for `get_last_transcription` command.
            *state.last_transcription.lock().unwrap() = Some(result.clone());

            emit_recording_state(&app, RecordingState::Success, None);

            // Optional success notification.
            let notify_success = settings
                .get("notifications.on_success")
                .map(|v| v == "true")
                .unwrap_or(false);
            if notify_success {
                let word_count = result.cleaned_text.split_whitespace().count();
                let preview: String = result.cleaned_text.chars().take(80).collect();
                let body = format!(
                    "{} word{} — {}{}",
                    word_count,
                    if word_count == 1 { "" } else { "s" },
                    preview,
                    if result.cleaned_text.len() > 80 {
                        "…"
                    } else {
                        ""
                    }
                );
                let _ = app
                    .notification()
                    .builder()
                    .title("LocalVoice")
                    .body(&body)
                    .show();
            }

            if let Err(e) = app.emit("transcription-completed", &result) {
                log::error!("Failed to emit transcription-completed: {e}");
            }

            log::info!(
                "Transcription done in {}ms — {} words (output: {} {})",
                result.duration_ms,
                result.cleaned_text.split_whitespace().count(),
                result
                    .output
                    .as_ref()
                    .map(|o| o.mode.as_str())
                    .unwrap_or("?"),
                if result.output.as_ref().map(|o| o.success).unwrap_or(false) {
                    "ok"
                } else {
                    "failed"
                }
            );

            // Auto-reset pill to Idle after 2 s so the next hotkey press works.
            schedule_idle_reset(app, Duration::from_millis(2000));
        }
        Err(e) => {
            // Clean up temp WAV on error.
            let _ = std::fs::remove_file(&wav_path);
            log::error!("Transcription failed: {e}");
            let friendly = crate::errors::user_friendly_message(&e.to_string());

            // Error notification (opt-out via settings).
            let state = app.state::<AppState>();
            let settings =
                crate::db::repositories::settings_repo::get_all(&state.db).unwrap_or_default();
            let notify_error = settings
                .get("notifications.on_error")
                .map(|v| v == "true")
                .unwrap_or(true);
            if notify_error {
                let _ = app
                    .notification()
                    .builder()
                    .title("LocalVoice — Error")
                    .body(&friendly)
                    .show();
            }

            emit_recording_state(&app, RecordingState::Error, Some(friendly));

            // Auto-reset pill to Idle after 3 s.
            schedule_idle_reset(app, Duration::from_millis(3000));
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Performs the output step based on `mode`:
/// - `"insert"` → write to clipboard then simulate Ctrl+V; falls back to
///   clipboard-only if the paste simulation fails.
/// - anything else → write to clipboard only.
fn perform_output(text: &str, mode: &str, insert_delay_ms: u64) -> OutputResult {
    match mode {
        "insert" => match text_insertion::insert(text, insert_delay_ms) {
            Ok(()) => OutputResult {
                mode: "insert".to_string(),
                success: true,
                error: None,
            },
            Err(e) => {
                log::warn!("Insert mode failed ({e}); falling back to clipboard");
                match clipboard::write(text) {
                    Ok(_) => OutputResult {
                        mode: "clipboard".to_string(),
                        success: true,
                        error: Some("Text copied — paste manually".to_string()),
                    },
                    Err(e2) => OutputResult {
                        mode: "insert".to_string(),
                        success: false,
                        error: Some(e2.to_string()),
                    },
                }
            }
        },
        _ => match clipboard::write(text) {
            Ok(_) => OutputResult {
                mode: "clipboard".to_string(),
                success: true,
                error: None,
            },
            Err(e) => OutputResult {
                mode: "clipboard".to_string(),
                success: false,
                error: Some(e.to_string()),
            },
        },
    }
}

/// Copies the temporary WAV file into the app data directory for later reprocessing.
/// Returns the persisted file path as a string.
fn persist_audio_file(app: &AppHandle, wav_path: &str) -> CmdResult<String> {
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data dir: {e}"))?;
    let audio_dir = app_dir.join("audio");
    std::fs::create_dir_all(&audio_dir).map_err(|e| format!("Failed to create audio dir: {e}"))?;

    let filename = format!("{}.wav", Uuid::new_v4());
    let dest = audio_dir.join(&filename);
    std::fs::copy(wav_path, &dest).map_err(|e| format!("Failed to copy audio file: {e}"))?;
    // Remove the original temp file after copying.
    let _ = std::fs::remove_file(wav_path);
    Ok(dest.to_string_lossy().into_owned())
}

/// Spawns an async task that transitions the recording state back to `Idle`
/// after `delay`. Used to auto-dismiss the Success / Error pill state.
pub fn schedule_idle_reset(app: AppHandle, delay: Duration) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(delay).await;
        emit_recording_state(&app, RecordingState::Idle, None);
    });
}
