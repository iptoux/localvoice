use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter, Manager};

use crate::db::repositories::settings_repo;
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

    let stored_model_path = settings.get("transcription.model_path").cloned();
    let effective_model_override = model_path_override
        .map(|s| s.to_string())
        .or(stored_model_path);

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

    let (cleaned_text, cleaned_segments) = pipeline::run(&raw_text, segments, &settings);

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

            result.output = Some(output_result);

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
