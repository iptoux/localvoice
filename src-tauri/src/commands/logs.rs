use rusqlite::params;

use crate::errors::{AppError, CmdResult};
use crate::logging::{get_db, set_enabled, LogEntry};

/// Returns the most recent log entries from the persistent `app_logs` table,
/// optionally filtered by level.
///
/// `level_filter`: `"warn"`, `"error"`, `"info"`, or `None` for all.
/// `limit`: max rows to return (default 500).
#[tauri::command]
pub fn list_logs(
    level_filter: Option<String>,
    limit: Option<usize>,
) -> CmdResult<Vec<LogEntry>> {
    let db = get_db().ok_or_else(|| AppError("Log database not initialized".into()))?;
    let conn = db
        .lock()
        .map_err(|_| AppError("Database lock poisoned".into()))?;
    let limit = limit.unwrap_or(500) as i64;

    let entries = if let Some(level) = &level_filter {
        let mut stmt = conn
            .prepare(
                "SELECT id, level, area, message, created_at
                 FROM app_logs
                 WHERE level = ?1
                 ORDER BY created_at DESC
                 LIMIT ?2",
            )
            .map_err(AppError::from)?;
        let rows = stmt
            .query_map(params![level, limit], row_to_entry)
            .map_err(AppError::from)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(AppError::from)?;
        rows
    } else {
        let mut stmt = conn
            .prepare(
                "SELECT id, level, area, message, created_at
                 FROM app_logs
                 ORDER BY created_at DESC
                 LIMIT ?1",
            )
            .map_err(AppError::from)?;
        let rows = stmt
            .query_map(params![limit], row_to_entry)
            .map_err(AppError::from)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(AppError::from)?;
        rows
    };

    Ok(entries)
}

/// Opens a native save-file dialog and writes all log entries as JSON.
#[tauri::command]
pub fn export_logs() -> CmdResult<()> {
    let db = get_db().ok_or_else(|| AppError("Log database not initialized".into()))?;
    let entries: Vec<LogEntry> = {
        let conn = db
            .lock()
            .map_err(|_| AppError("Database lock poisoned".into()))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, level, area, message, created_at
                 FROM app_logs
                 ORDER BY created_at DESC",
            )
            .map_err(AppError::from)?;
        let rows = stmt
            .query_map([], row_to_entry)
            .map_err(AppError::from)?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(AppError::from)?;
        rows
    };

    let json =
        serde_json::to_string_pretty(&entries).map_err(|e| AppError(e.to_string()))?;

    let path = rfd::FileDialog::new()
        .set_title("Export Logs")
        .add_filter("JSON", &["json"])
        .set_file_name("localvoice-logs.json")
        .save_file()
        .ok_or_else(|| AppError("Export cancelled".into()))?;

    std::fs::write(&path, json.as_bytes()).map_err(|e| AppError(e.to_string()))
}

/// Deletes all entries from the `app_logs` table.
#[tauri::command]
pub fn clear_logs() -> CmdResult<()> {
    let db = get_db().ok_or_else(|| AppError("Log database not initialized".into()))?;
    let conn = db
        .lock()
        .map_err(|_| AppError("Database lock poisoned".into()))?;
    conn.execute("DELETE FROM app_logs", [])
        .map_err(AppError::from)?;
    Ok(())
}

/// Enables or disables in-app log buffering at runtime.
#[tauri::command]
pub fn set_logging_enabled(enabled: bool) -> CmdResult<()> {
    set_enabled(enabled);
    Ok(())
}

// ── Row mapper ────────────────────────────────────────────────────────────────

fn row_to_entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<LogEntry> {
    Ok(LogEntry {
        id: row.get(0)?,
        level: row.get(1)?,
        area: row.get(2)?,
        message: row.get(3)?,
        created_at: row.get(4)?,
    })
}
