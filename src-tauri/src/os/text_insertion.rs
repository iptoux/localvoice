use std::time::Duration;

use crate::errors::AppError;

use super::clipboard;

/// Maximum characters per paste chunk. Some target apps (e.g. older WinForms
/// controls) have trouble with very large clipboard payloads.
const CHUNK_SIZE: usize = 4000;

/// Inserts `text` into the currently focused application via clipboard + paste key.
///
/// `insert_delay_ms` controls the pause between writing to the clipboard and
/// sending the paste key — higher values help slow target apps process the paste.
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

/// Writes a single chunk to the clipboard and simulates the paste key.
fn insert_chunk(text: &str, insert_delay_ms: u64) -> Result<(), AppError> {
    clipboard::write(text)?;

    // Pause to let the clipboard settle before sending the keypress.
    std::thread::sleep(Duration::from_millis(insert_delay_ms));

    send_paste_key();

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

// ── Windows — SendInput (Ctrl+V) ──────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn send_paste_key() {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_V,
    };

    let inputs: [INPUT; 4] = [
        kbd_input(VK_CONTROL, 0),               // Ctrl down
        kbd_input(VK_V, 0),                     // V down
        kbd_input(VK_V, KEYEVENTF_KEYUP),       // V up
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

// ── macOS — osascript Cmd+V ───────────────────────────────────────────────────
//
// Requires the app (or Terminal during dev) to have Accessibility permission in
// System Preferences → Privacy & Security → Accessibility.

#[cfg(target_os = "macos")]
fn send_paste_key() {
    use std::process::Command;
    let result = Command::new("osascript")
        .args([
            "-e",
            r#"tell application "System Events" to keystroke "v" using command down"#,
        ])
        .status();
    if let Err(e) = result {
        log::warn!("osascript Cmd+V failed: {e}. Grant Accessibility permission to LocalVoice in System Preferences → Privacy & Security → Accessibility.");
    }
}

// ── Linux — xdotool (X11) or wtype (Wayland) ─────────────────────────────────
//
// X11:     requires `xdotool`  (apt install xdotool / pacman -S xdotool)
// Wayland: requires `wtype`    (apt install wtype  / pacman -S wtype)
//          or      `ydotool`   as an alternative (needs ydotoold daemon)

#[cfg(target_os = "linux")]
fn send_paste_key() {
    use std::process::Command;

    let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok()
        || std::env::var("XDG_SESSION_TYPE")
            .map(|v| v.eq_ignore_ascii_case("wayland"))
            .unwrap_or(false);

    if is_wayland {
        // Try wtype first, fall back to ydotool.
        let result = Command::new("wtype").args(["-k", "ctrl+v"]).status();
        if let Err(e) = result {
            log::warn!(
                "wtype Ctrl+V failed: {e}. Install wtype (`apt install wtype`) for Wayland paste support."
            );
        }
    } else {
        let result = Command::new("xdotool").args(["key", "ctrl+v"]).status();
        if let Err(e) = result {
            log::warn!(
                "xdotool Ctrl+V failed: {e}. Install xdotool (`apt install xdotool`) for X11 paste support."
            );
        }
    }
}

// ── Fallback for other platforms ──────────────────────────────────────────────

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn send_paste_key() {
    log::warn!("Automatic paste not implemented on this platform. Text has been copied to clipboard.");
}
