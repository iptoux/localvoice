use std::time::Instant;

use tauri::{AppHandle, Emitter, Manager};

use crate::db::repositories::settings_repo;
use crate::errors::CmdResult;
use crate::state::app_state::emit_recording_state;
use crate::state::recording_state::RecordingState;
use crate::state::AppState;
use crate::transcription::{language, parser, pipeline, whisper_sidecar};
use crate::transcription::types::TranscriptionResult;

/// Runs the full transcription pipeline synchronously.
///
/// - Reads language and model path from `AppState` settings.
/// - `language_override` / `model_path_override` let callers bypass settings.
/// - Returns a complete `TranscriptionResult`.
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
        .join(format!(
            "localvoice_out_{}",
            uuid::Uuid::new_v4()
        ));

    let start = Instant::now();

    let output =
        whisper_sidecar::invoke(&binary, &model, std::path::Path::new(wav_path), &whisper_lang, &output_prefix)?;

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
    })
}

/// Entry point for the background transcription task.
///
/// Called from a `tokio::task::spawn_blocking` closure after `stop_recording`.
/// Transitions pill to Success or Error and emits `transcription-completed`.
pub fn transcribe_and_emit(app: AppHandle, wav_path: String) {
    match transcribe(&app, &wav_path, None, None) {
        Ok(result) => {
            // Store for `get_last_transcription` command.
            let state = app.state::<AppState>();
            *state.last_transcription.lock().unwrap() = Some(result.clone());

            emit_recording_state(&app, RecordingState::Success, None);

            if let Err(e) = app.emit("transcription-completed", &result) {
                log::error!("Failed to emit transcription-completed: {e}");
            }

            log::info!(
                "Transcription done in {}ms — {} words",
                result.duration_ms,
                result.cleaned_text.split_whitespace().count()
            );
        }
        Err(e) => {
            log::error!("Transcription failed: {e}");
            emit_recording_state(&app, RecordingState::Error, Some(e.to_string()));
        }
    }
}
