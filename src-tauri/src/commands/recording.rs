use std::sync::atomic::Ordering;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, State};

use crate::audio::capture::{self, SilenceConfig};
use crate::audio::devices;
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

    // Read settings.
    let settings =
        crate::db::repositories::settings_repo::get_all(&state.db).unwrap_or_default();

    // Preferred audio device.
    let device_id: Option<String> = settings
        .get("recording.device_id")
        .cloned()
        .filter(|s| !s.is_empty());

    // Silence detection configuration.
    let silence_cfg = SilenceConfig {
        enabled: settings
            .get("recording.silence_timeout_ms")
            .and_then(|v| v.parse::<u64>().ok())
            .map(|ms| ms > 0)
            .unwrap_or(false),
        threshold: settings
            .get("recording.silence_threshold")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.01),
        timeout_ms: settings
            .get("recording.silence_timeout_ms")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1500),
    };

    let device = devices::get_input_device(device_id.as_deref())?;
    let recording = capture::start_capture(&device, app, silence_cfg)?;

    // Grab the silence flag before moving the recording into state.
    let silence_flag = recording.silence_triggered.clone();

    *state.active_recording.lock().unwrap() = Some(recording);
    *state.recording_started_at.lock().unwrap() = Some(chrono::Utc::now());
    emit_recording_state(app, RecordingState::Listening, None);

    // Spawn a background thread that polls the silence flag every 200ms.
    let app_watcher = app.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(200)).await;

            // Check if we are still listening.
            let state = app_watcher.state::<AppState>();
            let current = state.recording_state.lock().unwrap().clone();
            if current != RecordingState::Listening {
                break;
            }

            if silence_flag.load(Ordering::Relaxed) {
                log::info!("Silence timeout reached — auto-stopping recording");
                let _ = app_watcher.emit("silence-detected", ());
                // Must run stop on a blocking thread because it does file I/O.
                let app_for_stop = app_watcher.clone();
                let _ = tauri::async_runtime::spawn_blocking(move || {
                    let state = app_for_stop.state::<AppState>();
                    if let Err(e) = stop_recording_internal(&app_for_stop, &state) {
                        log::error!("Silence auto-stop failed: {e}");
                    }
                }).await;
                break;
            }
        }
    });

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
