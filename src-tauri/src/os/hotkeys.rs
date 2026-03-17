use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};

use crate::commands::recording::{
    cancel_recording_internal, start_recording_internal, stop_recording_internal,
};
use crate::state::recording_state::RecordingState;
use crate::state::AppState;

/// Called from `tauri_plugin_global_shortcut::Builder::with_handler` in lib.rs
/// for every shortcut event. Dispatches to the appropriate recording action.
pub fn handle(app: &AppHandle, _shortcut: &Shortcut, event: ShortcutEvent) {
    if event.state() != ShortcutState::Pressed {
        return;
    }

    let state = app.state::<AppState>();
    let current = state.recording_state.lock().unwrap().clone();

    match current {
        RecordingState::Idle => {
            if let Err(e) = start_recording_internal(app, &state) {
                log::error!("Hotkey: start_recording failed: {e}");
            }
        }
        RecordingState::Listening => {
            if let Err(e) = stop_recording_internal(app, &state) {
                log::error!("Hotkey: stop_recording failed: {e}");
            }
        }
        RecordingState::Processing => {
            // Pressing again during processing cancels the session.
            cancel_recording_internal(app, &state);
        }
        RecordingState::Success | RecordingState::Error => {
            // Ignore until the UI transitions back to Idle.
        }
    }
}

/// Registers the global shortcut from the `recording.shortcut` setting.
///
/// Must be called after `AppState` has been managed (i.e. inside `setup`).
/// Default: `Ctrl+Shift+Space` (translates `CommandOrControl` → `Ctrl` on Windows).
pub fn setup(app: &AppHandle) -> Result<(), String> {
    let shortcut_str = {
        let state = app.state::<AppState>();
        let settings =
            crate::db::repositories::settings_repo::get_all(&state.db).unwrap_or_default();
        let raw = settings
            .get("recording.shortcut")
            .cloned()
            .unwrap_or_else(|| "CommandOrControl+Shift+Space".to_string());
        // Translate Electron-style modifiers to keyboard-types format.
        raw.replace("CommandOrControl", "Ctrl")
            .replace("CmdOrCtrl", "Ctrl")
    };

    app.global_shortcut()
        .register(shortcut_str.as_str())
        .map_err(|e| format!("Failed to register global shortcut '{shortcut_str}': {e}"))?;

    log::info!("Global shortcut registered: {shortcut_str}");
    Ok(())
}
