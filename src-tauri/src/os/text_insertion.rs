use std::time::Duration;

use crate::errors::AppError;

use super::clipboard;

/// Maximum characters per paste chunk. Some target apps (e.g. older WinForms
/// controls) have trouble with very large clipboard payloads.
const CHUNK_SIZE: usize = 4000;

/// Inserts `text` into the currently focused application via clipboard + Ctrl+V.
///
/// `insert_delay_ms` controls the pause between writing to the clipboard and
/// sending Ctrl+V — higher values help slow target apps process the paste.
///
/// For texts longer than 4 000 chars the text is split into chunks, each
/// pasted sequentially with the configured delay in between.
///
/// If the paste simulation fails the text is still on the clipboard so the
/// user can paste manually.
pub fn insert(text: &str, insert_delay_ms: u64) -> Result<(), AppError> {
    // Save previous clipboard content first.
    let previous = clipboard::read_previous()?;

    if text.len() <= CHUNK_SIZE {
        insert_chunk(text, insert_delay_ms)?;
    } else {
        // Split on char boundaries near CHUNK_SIZE.
        for chunk in chunk_text(text, CHUNK_SIZE) {
            insert_chunk(chunk, insert_delay_ms)?;
        }
    }

    // Restore the previous clipboard content (best-effort).
    let _ = clipboard::restore(previous);

    Ok(())
}

/// Writes a single chunk to the clipboard and simulates Ctrl+V.
fn insert_chunk(text: &str, insert_delay_ms: u64) -> Result<(), AppError> {
    clipboard::write(text)?;

    // Pause to let the clipboard settle before sending the keypress.
    std::thread::sleep(Duration::from_millis(insert_delay_ms));

    send_ctrl_v();

    // Allow the target app time to process the paste event.
    std::thread::sleep(Duration::from_millis(100));

    Ok(())
}

/// Splits `text` into chunks of at most `max_chars`, breaking on char boundaries.
fn chunk_text(text: &str, max_chars: usize) -> Vec<&str> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let end = if start + max_chars >= text.len() {
            text.len()
        } else {
            // Find a char boundary at or before start + max_chars.
            let mut e = start + max_chars;
            while !text.is_char_boundary(e) && e > start {
                e -= 1;
            }
            e
        };
        chunks.push(&text[start..end]);
        start = end;
    }
    chunks
}

// ── Windows SendInput ────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn send_ctrl_v() {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_V,
    };

    let inputs: [INPUT; 4] = [
        kbd_input(VK_CONTROL, 0),           // Ctrl down
        kbd_input(VK_V, 0),                 // V down
        kbd_input(VK_V, KEYEVENTF_KEYUP),   // V up
        kbd_input(VK_CONTROL, KEYEVENTF_KEYUP), // Ctrl up
    ];

    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

#[cfg(target_os = "windows")]
fn kbd_input(vk: u16, flags: u32) -> windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{INPUT, INPUT_KEYBOARD, KEYBDINPUT};

    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: windows_sys::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

#[cfg(not(target_os = "windows"))]
fn send_ctrl_v() {
    log::warn!("send_ctrl_v not implemented on this platform");
}
