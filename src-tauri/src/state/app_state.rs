use crate::db::DbConn;
use crate::state::recording_state::{ActiveRecording, RecordingState, RecordingStatePayload};
use crate::transcription::types::TranscriptionResult;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};

/// Central application state injected via `tauri::Builder::manage()`.
/// Access in commands via `tauri::State<AppState>`.
pub struct AppState {
    /// Shared SQLite connection.
    pub db: DbConn,

    /// Currently active recording session id (None when idle).
    /// Populated in MS-03 when a transcription session row is created.
    #[allow(dead_code)]
    pub active_session_id: Mutex<Option<String>>,

    /// Current recording state (Idle / Listening / Processing / Success / Error).
    pub recording_state: Mutex<RecordingState>,

    /// Live audio capture — Some while recording, None otherwise.
    /// Dropping the inner value stops the cpal stream automatically.
    pub active_recording: Mutex<Option<ActiveRecording>>,

    /// Path of the last WAV file written; set after stop_recording for transcription.
    pub last_wav_path: Mutex<Option<String>>,

    /// Most recently completed transcription result; set by the orchestrator.
    pub last_transcription: Mutex<Option<TranscriptionResult>>,

    /// Timestamp captured when recording starts; used to compute session duration.
    pub recording_started_at: Mutex<Option<chrono::DateTime<chrono::Utc>>>,
}

impl AppState {
    pub fn new(db: DbConn) -> Self {
        Self {
            db,
            active_session_id: Mutex::new(None),
            recording_state: Mutex::new(RecordingState::Idle),
            active_recording: Mutex::new(None),
            last_wav_path: Mutex::new(None),
            last_transcription: Mutex::new(None),
            recording_started_at: Mutex::new(None),
        }
    }
}

/// Updates `recording_state` in `AppState` and broadcasts `recording-state-changed`
/// to all windows. Shared by `commands/recording.rs` and `transcription/orchestrator.rs`.
pub fn emit_recording_state(app: &AppHandle, new_state: RecordingState, error: Option<String>) {
    let state = app.state::<AppState>();
    *state.recording_state.lock().unwrap() = new_state.clone();
    let payload = RecordingStatePayload {
        state: new_state,
        error,
    };
    if let Err(e) = app.emit("recording-state-changed", &payload) {
        log::error!("Failed to emit recording-state-changed: {e}");
    }
}
