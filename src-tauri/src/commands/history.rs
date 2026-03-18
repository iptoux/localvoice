use tauri::{AppHandle, Emitter, Manager, State};

use crate::db::models::{Session, SessionFilter, SessionWithSegments};
use crate::db::repositories::sessions_repo;
use crate::errors::{AppError, CmdResult};
use crate::history::{export, reprocess};
use crate::state::AppState;

/// Returns a filtered, paginated list of sessions (newest first).
#[tauri::command]
pub fn list_sessions(state: State<AppState>, filter: SessionFilter) -> CmdResult<Vec<Session>> {
    sessions_repo::list_sessions(&state.db, &filter)
}

/// Returns a single session together with its time-stamped segments.
#[tauri::command]
pub fn get_session(state: State<AppState>, session_id: String) -> CmdResult<SessionWithSegments> {
    sessions_repo::get_session(&state.db, &session_id)
}

/// Permanently deletes a session and all its segments.
#[tauri::command]
pub fn delete_session(state: State<AppState>, session_id: String) -> CmdResult<()> {
    sessions_repo::delete_session(&state.db, &session_id)
}

/// Exports the requested sessions to a user-chosen file.
///
/// - `session_ids`: list of session ids to export; if empty, exports nothing.
/// - `format`: `"json"` for JSON array, anything else for plain text.
///
/// Opens a native save-file dialog. Returns the chosen path on success,
/// or an error if the dialog is cancelled or the write fails.
#[tauri::command]
pub fn export_sessions(
    state: State<AppState>,
    session_ids: Vec<String>,
    format: String,
) -> CmdResult<String> {
    if session_ids.is_empty() {
        return Err(AppError("No sessions selected for export".to_string()));
    }

    let sessions = sessions_repo::get_sessions_by_ids(&state.db, &session_ids)?;

    let (content, ext) = if format == "json" {
        (export::to_json(&sessions)?, "json")
    } else {
        (export::to_text(&sessions), "txt")
    };

    let path = rfd::FileDialog::new()
        .set_title("Export Sessions")
        .add_filter("Text file", &["txt"])
        .add_filter("JSON", &["json"])
        .set_file_name(&format!("localvoice-export.{ext}"))
        .save_file()
        .ok_or_else(|| AppError("Export cancelled".to_string()))?;

    std::fs::write(&path, content)
        .map_err(|e| AppError(format!("Failed to write export file: {e}")))?;

    Ok(path.to_string_lossy().to_string())
}

/// Re-transcribes a session using its stored audio file.
///
/// Optionally overrides the language and/or model. Emits `session-reprocessed`
/// on success so the frontend can refresh the detail view.
#[tauri::command]
pub async fn reprocess_session(
    app: AppHandle,
    session_id: String,
    language: Option<String>,
    model_id: Option<String>,
) -> CmdResult<SessionWithSegments> {
    let app_clone = app.clone();
    let sid = session_id.clone();

    tauri::async_runtime::spawn_blocking(move || {
        reprocess::reprocess_session(
            &app_clone,
            &sid,
            language.as_deref(),
            model_id.as_deref(),
        )
    })
    .await
    .map_err(|e| AppError(format!("Task join error: {e}")))??;

    // Re-read the updated session to return it.
    let state = app.state::<AppState>();
    let detail = sessions_repo::get_session(&state.db, &session_id)?;

    let _ = app.emit("session-reprocessed", &detail.session.id);

    Ok(detail)
}
