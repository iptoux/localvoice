use tauri::{AppHandle, Emitter, State};

use crate::audio::{capture, devices};
use crate::errors::CmdResult;
use crate::state::recording_state::{RecordingState, RecordingStatePayload};
use crate::state::AppState;

// ── Internal helpers (shared with hotkeys) ────────────────────────────────────

/// Transitions state, persists it in AppState, and emits the event to all windows.
pub(crate) fn emit_state(
    app: &AppHandle,
    state: &State<AppState>,
    new_state: RecordingState,
    error: Option<String>,
) {
    *state.recording_state.lock().unwrap() = new_state.clone();
    let payload = RecordingStatePayload {
        state: new_state,
        error,
    };
    if let Err(e) = app.emit("recording-state-changed", &payload) {
        log::error!("Failed to emit recording-state-changed: {e}");
    }
}

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
        let id = settings
            .get("recording.device_id")
            .cloned()
            .filter(|s| !s.is_empty());
        id
    };

    let device = devices::get_input_device(device_id.as_deref())?;

    let recording = capture::start_capture(&device, app)?;

    *state.active_recording.lock().unwrap() = Some(recording);
    emit_state(app, state, RecordingState::Listening, None);

    log::info!("Recording started");
    Ok(())
}

/// Core stop logic — returns the WAV file path.
pub fn stop_recording_internal(app: &AppHandle, state: &State<AppState>) -> CmdResult<String> {
    let recording = state
        .active_recording
        .lock()
        .unwrap()
        .take()
        .ok_or("No active recording")?;

    emit_state(app, state, RecordingState::Processing, None);

    match capture::stop_capture(recording) {
        Ok(path) => {
            log::info!("Recording saved to {path}");
            Ok(path)
        }
        Err(e) => {
            emit_state(
                app,
                state,
                RecordingState::Error,
                Some(e.to_string()),
            );
            Err(e)
        }
    }
}

/// Core cancel logic — discards the buffer, returns to Idle.
pub fn cancel_recording_internal(app: &AppHandle, state: &State<AppState>) {
    if let Some(recording) = state.active_recording.lock().unwrap().take() {
        capture::cancel_capture(recording);
    }
    emit_state(app, state, RecordingState::Idle, None);
    log::info!("Recording cancelled");
}

// ── Tauri commands ────────────────────────────────────────────────────────────

/// Starts a new recording session.
#[tauri::command]
pub fn start_recording(app: AppHandle, state: State<AppState>) -> CmdResult<()> {
    start_recording_internal(&app, &state)
}

/// Stops the current recording, writes a WAV file, and returns its path.
/// Transitions the pill to Processing and then stays there until
/// the transcription step (MS-03) sets Success or Error.
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
