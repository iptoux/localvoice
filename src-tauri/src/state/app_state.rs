use crate::db::DbConn;
use std::sync::Mutex;

/// Central application state injected via `tauri::Builder::manage()`.
/// Access in commands via `tauri::State<AppState>`.
pub struct AppState {
    /// Shared SQLite connection.
    pub db: DbConn,

    /// Currently active recording session id (None when idle).
    pub active_session_id: Mutex<Option<String>>,

    /// Recording handle placeholder — will be replaced with a real cpal handle in MS-02.
    pub is_recording: Mutex<bool>,
}

impl AppState {
    pub fn new(db: DbConn) -> Self {
        Self {
            db,
            active_session_id: Mutex::new(None),
            is_recording: Mutex::new(false),
        }
    }
}
