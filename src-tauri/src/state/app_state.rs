use crate::db::DbConn;
use crate::state::recording_state::{ActiveRecording, RecordingState};
use std::sync::Mutex;

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
}

impl AppState {
    pub fn new(db: DbConn) -> Self {
        Self {
            db,
            active_session_id: Mutex::new(None),
            recording_state: Mutex::new(RecordingState::Idle),
            active_recording: Mutex::new(None),
        }
    }
}
