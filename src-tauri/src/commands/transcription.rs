use tauri::{AppHandle, State};

use crate::errors::CmdResult;
use crate::state::AppState;
use crate::transcription::orchestrator;
use crate::transcription::types::TranscriptionResult;

/// Re-transcribes the most recently recorded WAV file.
///
/// `language` defaults to the `transcription.default_language` setting.
/// `model_id` is currently a no-op placeholder — MS-07 will wire it up.
#[tauri::command]
pub fn transcribe_last_recording(
    app: AppHandle,
    state: State<AppState>,
    language: Option<String>,
    _model_id: Option<String>,
) -> CmdResult<TranscriptionResult> {
    let wav_path = state
        .last_wav_path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No recording available to transcribe")?;

    orchestrator::transcribe(&app, &wav_path, language.as_deref(), None)
}

/// Returns the most recently completed transcription result, if any.
#[tauri::command]
pub fn get_last_transcription(state: State<AppState>) -> CmdResult<Option<TranscriptionResult>> {
    Ok(state.last_transcription.lock().unwrap().clone())
}
