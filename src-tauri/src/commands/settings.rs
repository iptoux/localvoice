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

/// Resets all settings to defaults by deleting all rows and re-seeding via migration defaults.
/// For now this is a no-op stub — full reset will run the seed SQL again in MS-10.
#[tauri::command]
pub fn reset_settings(_state: State<AppState>) -> CmdResult<()> {
    Ok(())
}
