use tauri::{AppHandle, State};

use crate::audio::{capture, devices};
use crate::errors::CmdResult;
use crate::state::app_state::emit_recording_state;
use crate::state::recording_state::RecordingState;
use crate::state::AppState;

// ── Internal helpers (shared with hotkeys) ────────────────────────────────────

/// Core start logic — called from both the Tauri command and the hotkey handler.
pub fn start_recording_internal(app: &AppHandle, state: &State<AppState>) -> CmdResult<()> {
    {
        let current = state.recording_state.lock().unwrap();
        if *current != RecordingState::Idle {
            return Err("Recording already in progress".into());
        }
    }

    // Read the preferred device from settings (falls back to default).
    let device_id: Option<String> = {
        let settings =
            crate::db::repositories::settings_repo::get_all(&state.db).unwrap_or_default();
        settings
            .get("recording.device_id")
            .cloned()
            .filter(|s| !s.is_empty())
    };

    let device = devices::get_input_device(device_id.as_deref())?;
    let recording = capture::start_capture(&device, app)?;

    *state.active_recording.lock().unwrap() = Some(recording);
    *state.recording_started_at.lock().unwrap() = Some(chrono::Utc::now());
    emit_recording_state(app, RecordingState::Listening, None);

    log::info!("Recording started");
    Ok(())
}

/// Core stop logic — stops audio capture, writes WAV, then fires the transcription
/// pipeline in a background task. Returns the WAV file path for the caller.
pub fn stop_recording_internal(app: &AppHandle, state: &State<AppState>) -> CmdResult<String> {
    let recording = state
        .active_recording
        .lock()
        .unwrap()
        .take()
        .ok_or("No active recording")?;

    emit_recording_state(app, RecordingState::Processing, None);

    let wav_path = match capture::stop_capture(recording) {
        Ok(path) => path,
        Err(e) => {
            emit_recording_state(app, RecordingState::Error, Some(e.to_string()));
            return Err(e);
        }
    };

    // Persist WAV path so transcribe_last_recording can find it.
    *state.last_wav_path.lock().unwrap() = Some(wav_path.clone());

    log::info!("Recording saved to {wav_path}");

    // Kick off transcription in a background thread so the command returns immediately.
    // tauri::async_runtime::spawn works from any thread (including the hotkey message-loop
    // thread) because it uses Tauri's managed runtime rather than requiring an ambient one.
    let app_for_task = app.clone();
    let wav_for_task = wav_path.clone();
    tauri::async_runtime::spawn(async move {
        tauri::async_runtime::spawn_blocking(move || {
            crate::transcription::orchestrator::transcribe_and_emit(app_for_task, wav_for_task);
        })
        .await
        .unwrap_or_else(|e| log::error!("Transcription task panicked: {e}"));
    });

    Ok(wav_path)
}

/// Core cancel logic — discards the buffer, returns to Idle.
pub fn cancel_recording_internal(app: &AppHandle, state: &State<AppState>) {
    if let Some(recording) = state.active_recording.lock().unwrap().take() {
        capture::cancel_capture(recording);
    }
    *state.recording_started_at.lock().unwrap() = None;
    emit_recording_state(app, RecordingState::Idle, None);
    log::info!("Recording cancelled");
}

// ── Tauri commands ────────────────────────────────────────────────────────────

/// Starts a new recording session.
#[tauri::command]
pub fn start_recording(app: AppHandle, state: State<AppState>) -> CmdResult<()> {
    start_recording_internal(&app, &state)
}

/// Stops the current recording and triggers background transcription.
/// The pill transitions to Processing; Success/Error is emitted by the orchestrator.
#[tauri::command]
pub fn stop_recording(app: AppHandle, state: State<AppState>) -> CmdResult<String> {
    stop_recording_internal(&app, &state)
}

/// Cancels the current recording without saving anything.
#[tauri::command]
pub fn cancel_recording(app: AppHandle, state: State<AppState>) -> CmdResult<()> {
    cancel_recording_internal(&app, &state);
    Ok(())
}

/// Returns the current recording state.
#[tauri::command]
pub fn get_recording_state(state: State<AppState>) -> CmdResult<RecordingState> {
    Ok(state.recording_state.lock().unwrap().clone())
}

/// Lists all available audio input devices.
#[tauri::command]
pub fn list_input_devices() -> CmdResult<Vec<crate::state::DeviceInfo>> {
    devices::list_input_devices()
}
