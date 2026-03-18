use tauri::State;

use crate::db::repositories::{models_repo, settings_repo};
use crate::errors::CmdResult;
use crate::os::autostart;
use crate::state::AppState;

/// Returns `true` when no model is installed — i.e. the user needs onboarding.
#[tauri::command]
pub fn check_first_run(state: State<AppState>) -> bool {
    models_repo::list_installed(&state.db)
        .map(|v| v.is_empty())
        .unwrap_or(true)
}

/// Returns `true` when a default model is configured for the current language setting.
#[tauri::command]
pub fn has_default_model(state: State<AppState>) -> bool {
    let settings = settings_repo::get_all(&state.db).unwrap_or_default();
    let language = settings
        .get("transcription.default_language")
        .cloned()
        .unwrap_or_else(|| "auto".to_string());

    if language == "auto" {
        models_repo::get_default_path(&state.db, "de")
            .map(|p| p.is_some())
            .unwrap_or(false)
            || models_repo::get_default_path(&state.db, "en")
                .map(|p| p.is_some())
                .unwrap_or(false)
    } else {
        models_repo::get_default_path(&state.db, &language)
            .map(|p| p.is_some())
            .unwrap_or(false)
    }
}

/// Enables or disables launching LocalVoice on OS login.
#[tauri::command]
pub fn set_autostart(enabled: bool) -> CmdResult<()> {
    autostart::set_autostart(enabled).map_err(Into::into)
}

/// Returns the current autostart state.
#[tauri::command]
pub fn get_autostart() -> bool {
    autostart::get_autostart()
}
