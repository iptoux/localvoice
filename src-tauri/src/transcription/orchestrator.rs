use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter, Manager};
use uuid::Uuid;

use crate::db::models::Session;
use crate::db::repositories::{ambiguous_terms_repo, dictionary_repo, models_repo, sessions_repo, settings_repo};
use crate::dictionary::service as dict_service;
use crate::errors::CmdResult;
use crate::os::{clipboard, text_insertion};
use crate::state::app_state::emit_recording_state;
use crate::state::recording_state::RecordingState;
use crate::state::AppState;
use crate::transcription::types::{OutputResult, TranscriptionResult};
use crate::transcription::{language, parser, pipeline, whisper_sidecar};

/// Runs the full transcription pipeline synchronously.
///
/// - Reads language and model path from `AppState` settings.
/// - `language_override` / `model_path_override` let callers bypass settings.
/// - Returns a complete `TranscriptionResult` with `output` set to `None`;
///   the output step is performed by [`transcribe_and_emit`].
pub fn transcribe(
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

    // Resolve model path: explicit override → DB default for language → settings path → auto-scan.
    let effective_model_override = if let Some(p) = model_path_override {
        Some(p.to_string())
    } else {
        // Try the DB-registered default for this language first.
        let db_default = models_repo::get_default_path(&state.db, &lang_code)
            .unwrap_or(None);
        if db_default.is_some() {
            db_default
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
        .unwrap_or(std::path::Path::new(std::env::temp_dir().to_str().unwrap_or(".")))
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
    // Also remove the WAV temp file now that we have the transcription.
    let _ = std::fs::remove_file(wav_path);

    let raw_text = parser::segments_to_text(&segments);

    // Load active correction rules for the current language.
    let active_rules = dictionary_repo::list_active_rules(&state.db, Some(&lang_code))
        .unwrap_or_default();

    let (cleaned_text, cleaned_segments, fired_rule_ids) =
        pipeline::run(&raw_text, segments, &settings, &active_rules);

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
        language: lang_code,
        model_id,
        duration_ms,
        output: None,
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
    match transcribe(&app, &wav_path, None, None) {
        Ok(mut result) => {
            // ── Output step ──────────────────────────────────────────────────
            let state = app.state::<AppState>();
            let settings = settings_repo::get_all(&state.db).unwrap_or_default();
            let output_mode = settings
                .get("output.mode")
                .cloned()
                .unwrap_or_else(|| "clipboard".to_string());

            let output_result = perform_output(&result.cleaned_text, &output_mode);

            // Emit dedicated output-result event (TASK-057).
            if let Err(e) = app.emit("output-result", &output_result) {
                log::error!("Failed to emit output-result: {e}");
            }

            result.output = Some(output_result.clone());

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
                trigger_type: "shortcut".to_string(),
                input_device_id: settings.get("recording.device_id").cloned(),
                raw_text: result.raw_text.clone(),
                cleaned_text: result.cleaned_text.clone(),
                word_count,
                char_count,
                avg_confidence,
                estimated_wpm,
                output_mode: output_mode.clone(),
                output_target_app: None,
                inserted_successfully: output_result.success,
                error_message: output_result.error.clone(),
                created_at: now.to_rfc3339(),
            };
            if let Err(e) = sessions_repo::insert_session(&state.db, &session) {
                log::error!("Failed to persist session: {e}");
            } else if let Err(e) =
                sessions_repo::insert_segments(&state.db, &session.id, &result.segments)
            {
                log::error!("Failed to persist session segments: {e}");
            } else {
                log::info!("Session {} persisted ({} segments)", session.id, result.segments.len());
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

            if let Err(e) = app.emit("transcription-completed", &result) {
                log::error!("Failed to emit transcription-completed: {e}");
            }

            log::info!(
                "Transcription done in {}ms — {} words (output: {} {})",
                result.duration_ms,
                result.cleaned_text.split_whitespace().count(),
                result.output.as_ref().map(|o| o.mode.as_str()).unwrap_or("?"),
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
            log::error!("Transcription failed: {e}");
            emit_recording_state(&app, RecordingState::Error, Some(e.to_string()));

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
fn perform_output(text: &str, mode: &str) -> OutputResult {
    match mode {
        "insert" => match text_insertion::insert(text) {
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
                        error: Some(format!("Insert failed, used clipboard: {e}")),
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

/// Spawns an async task that transitions the recording state back to `Idle`
/// after `delay`. Used to auto-dismiss the Success / Error pill state.
fn schedule_idle_reset(app: AppHandle, delay: Duration) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(delay).await;
        emit_recording_state(&app, RecordingState::Idle, None);
    });
}
