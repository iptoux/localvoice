use std::time::Duration;

use enigo::{Enigo, Key, KeyboardControllable};

use crate::errors::AppError;

use super::clipboard;

/// Inserts `text` into the currently focused application via clipboard + Ctrl+V.
///
/// Steps:
/// 1. Save current clipboard content.
/// 2. Write `text` to clipboard.
/// 3. Simulate `Ctrl+V` via enigo (`SendInput` on Windows).
/// 4. Wait 150 ms for the target app to process the paste event.
/// 5. Restore the previous clipboard content (best-effort).
///
/// If the paste simulation fails the error is returned but `text` is still
/// written to the clipboard so the user can paste manually.
pub fn insert(text: &str) -> Result<(), AppError> {
    // Save previous clipboard content and write the transcription text.
    let previous = clipboard::write(text)?;

    // Brief pause to let the clipboard settle before sending the keypress.
    std::thread::sleep(Duration::from_millis(50));

    // Simulate Ctrl+V in the focused application.
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);

    // Allow the target application time to receive and process the paste event.
    std::thread::sleep(Duration::from_millis(150));

    // Restore the previous clipboard content (best-effort — ignore errors).
    let _ = clipboard::restore(previous);

    Ok(())
}
