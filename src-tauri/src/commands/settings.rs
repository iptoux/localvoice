use crate::db::repositories::settings_repo;
use crate::errors::CmdResult;
use crate::state::AppState;
use std::collections::HashMap;
use tauri::State;

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
