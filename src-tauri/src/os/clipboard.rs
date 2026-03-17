use arboard::Clipboard;

use crate::errors::AppError;

/// Writes `text` to the system clipboard.
///
/// Returns the previous clipboard text so the caller can optionally restore it
/// after a paste operation.
pub fn write(text: &str) -> Result<Option<String>, AppError> {
    let mut board =
        Clipboard::new().map_err(|e| AppError(format!("Clipboard open failed: {e}")))?;
    let previous = board.get_text().ok();
    board
        .set_text(text)
        .map_err(|e| AppError(format!("Clipboard write failed: {e}")))?;
    Ok(previous)
}

/// Restores the clipboard to `previous`. No-op when `previous` is `None`.
pub fn restore(previous: Option<String>) -> Result<(), AppError> {
    let Some(prev) = previous else {
        return Ok(());
    };
    let mut board = Clipboard::new()
        .map_err(|e| AppError(format!("Clipboard open for restore failed: {e}")))?;
    board
        .set_text(&prev)
        .map_err(|e| AppError(format!("Clipboard restore failed: {e}")))?;
    Ok(())
}
