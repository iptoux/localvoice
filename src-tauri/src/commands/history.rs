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

/// Deletes multiple sessions in a single transaction. Returns the number of deleted rows.
#[tauri::command]
pub fn bulk_delete_sessions(state: State<AppState>, session_ids: Vec<String>) -> CmdResult<usize> {
    if session_ids.is_empty() {
        return Err(AppError("No sessions selected".to_string()));
    }
    sessions_repo::bulk_delete_sessions(&state.db, &session_ids)
}

/// Returns the absolute path to the audio file for a session.
/// Frontend should use `convertFileSrc()` from `@tauri-apps/api` to get a playable URL.
#[tauri::command]
pub fn get_audio_file_path(
    state: State<AppState>,
    session_id: String,
) -> CmdResult<Option<String>> {
    let detail = sessions_repo::get_session(&state.db, &session_id)?;
    Ok(detail.session.audio_path)
}

/// Exports the requested sessions to a user-chosen file.
///
/// - `session_ids`: list of session ids to export; if empty, exports nothing.
/// - `format`: `"json"`, `"csv"`, or anything else for plain text.
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

    let (content, ext) = match format.as_str() {
        "json" => (export::to_json(&sessions)?, "json"),
        "csv" => (export::to_csv(&sessions), "csv"),
        _ => (export::to_text(&sessions), "txt"),
    };

    let path = rfd::FileDialog::new()
        .set_title("Export Sessions")
        .add_filter("Text file", &["txt"])
        .add_filter("JSON", &["json"])
        .add_filter("CSV", &["csv"])
        .set_file_name(&format!("localvoice-export.{ext}"))
        .save_file()
        .ok_or_else(|| AppError("Export cancelled".to_string()))?;

    std::fs::write(&path, content)
        .map_err(|e| AppError(format!("Failed to write export file: {e}")))?;

    Ok(path.to_string_lossy().to_string())
}

/// Exports multiple sessions at once (bulk). Identical to `export_sessions` but
/// named explicitly for bulk use from the multi-select UI.
#[tauri::command]
pub fn bulk_export_sessions(
    state: State<AppState>,
    session_ids: Vec<String>,
    format: String,
) -> CmdResult<String> {
    export_sessions(state, session_ids, format)
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
        reprocess::reprocess_session(&app_clone, &sid, language.as_deref(), model_id.as_deref())
    })
    .await
    .map_err(|e| AppError(format!("Task join error: {e}")))??;

    // Re-read the updated session to return it.
    let state = app.state::<AppState>();
    let detail = sessions_repo::get_session(&state.db, &session_id)?;

    let _ = app.emit("session-reprocessed", &detail.session.id);

    Ok(detail)
}
