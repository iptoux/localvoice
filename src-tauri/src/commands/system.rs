use tauri::State;

use crate::db::repositories::models_repo;
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
