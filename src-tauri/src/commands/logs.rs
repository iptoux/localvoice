use crate::errors::{AppError, CmdResult};
use crate::logging::{get_buffer, LogEntry};

/// Returns the most recent log entries, optionally filtered by level.
///
/// `level_filter`: `"warn"`, `"error"`, or `None` for all.
/// `limit`: max rows to return (default 500).
#[tauri::command]
pub fn list_logs(
    level_filter: Option<String>,
    limit: Option<usize>,
) -> CmdResult<Vec<LogEntry>> {
    let buf = get_buffer().ok_or_else(|| AppError("Log buffer not initialized".into()))?;
    let lock = buf.read().map_err(|_| AppError("Log buffer lock poisoned".into()))?;
    let limit = limit.unwrap_or(500);
    let entries: Vec<LogEntry> = lock
        .iter()
        .filter(|e| {
            level_filter
                .as_deref()
                .map(|f| e.level == f)
                .unwrap_or(true)
        })
        .rev() // newest first
        .take(limit)
        .cloned()
        .collect();
    Ok(entries)
}

/// Opens a native save-file dialog and writes log entries as JSON.
#[tauri::command]
pub fn export_logs() -> CmdResult<()> {
    let buf = get_buffer().ok_or_else(|| AppError("Log buffer not initialized".into()))?;
    let entries: Vec<LogEntry> = {
        let lock = buf.read().map_err(|_| AppError("Log buffer lock poisoned".into()))?;
        lock.iter().rev().cloned().collect()
    };

    let json = serde_json::to_string_pretty(&entries)
        .map_err(|e| AppError(e.to_string()))?;

    let path = rfd::FileDialog::new()
        .set_title("Export Logs")
        .add_filter("JSON", &["json"])
        .set_file_name("localvoice-logs.json")
        .save_file()
        .ok_or_else(|| AppError("Export cancelled".into()))?;

    std::fs::write(&path, json.as_bytes()).map_err(|e| AppError(e.to_string()))
}

/// Clears all buffered log entries.
#[tauri::command]
pub fn clear_logs() -> CmdResult<()> {
    let buf = get_buffer().ok_or_else(|| AppError("Log buffer not initialized".into()))?;
    let mut lock = buf.write().map_err(|_| AppError("Log buffer lock poisoned".into()))?;
    lock.clear();
    Ok(())
}
