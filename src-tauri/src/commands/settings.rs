use crate::db::repositories::settings_repo;
use crate::errors::CmdResult;
use crate::os::modifier_shortcuts;
use crate::state::AppState;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

/// Returns all settings as a flat key→value map.
#[tauri::command]
pub fn get_settings(state: State<AppState>) -> CmdResult<HashMap<String, String>> {
    settings_repo::get_all(&state.db).map_err(Into::into)
}

/// Upserts a single setting key.
#[tauri::command]
pub fn update_setting(key: String, value: String, state: State<AppState>) -> CmdResult<()> {
    settings_repo::upsert(&state.db, &key, &value).map_err(Into::into)
}

/// Resets all settings to factory defaults.
#[tauri::command]
pub fn reset_settings(state: State<AppState>) -> CmdResult<()> {
    settings_repo::reset_to_defaults(&state.db).map_err(|e| crate::errors::AppError(e.to_string()))
}

/// Updates the global recording shortcut: validates format, unregisters the old
/// plugin shortcut, persists the new value, and registers or enables it.
#[tauri::command]
pub fn update_shortcut(shortcut: String, app: AppHandle) -> CmdResult<()> {
    // Translate to keyboard-types format for validation.
    let normalized = shortcut
        .replace("CommandOrControl", "Ctrl")
        .replace("CmdOrCtrl", "Ctrl");

    let modifier_shortcut = modifier_shortcuts::parse(&shortcut);

    if let Some(modifier_shortcut) = modifier_shortcut {
        modifier_shortcuts::validate_supported(modifier_shortcut)?;
    } else {
        // Validate by parsing before touching the DB.
        normalized
            .parse::<tauri_plugin_global_shortcut::Shortcut>()
            .map_err(|e| format!("Invalid shortcut '{shortcut}': {e}"))?;
    }

    // Unregister all current shortcuts.
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| format!("Failed to unregister shortcuts: {e}"))?;

    // Persist the new shortcut (in Electron-style format for UI display).
    let state = app.state::<AppState>();
    settings_repo::upsert(&state.db, "recording.shortcut", &shortcut)?;

    if let Some(modifier_shortcut) = modifier_shortcut {
        modifier_shortcuts::configure(&app, Some(modifier_shortcut))?;
        log::info!("Global modifier-only shortcut updated to: {shortcut}");
        return Ok(());
    }

    modifier_shortcuts::configure(&app, None)?;

    // Register the new shortcut.
    app.global_shortcut()
        .register(normalized.as_str())
        .map_err(|e| format!("Failed to register shortcut '{normalized}': {e}"))?;

    log::info!("Global shortcut updated to: {normalized}");
    Ok(())
}
