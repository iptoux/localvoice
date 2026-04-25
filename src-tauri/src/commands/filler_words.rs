use tauri::AppHandle;

use crate::db::repositories::filler_words_repo::{self, FillerStat, FillerWord};
use crate::errors::CmdResult;
use crate::state::AppState;
use tauri::Manager;

#[tauri::command]
pub async fn list_filler_words(
    app: AppHandle,
    language: Option<String>,
) -> CmdResult<Vec<FillerWord>> {
    let state = app.state::<AppState>();
    filler_words_repo::list(&state.db, language.as_deref()).map_err(Into::into)
}

#[tauri::command]
pub async fn add_filler_word(
    app: AppHandle,
    word: String,
    language: String,
) -> CmdResult<FillerWord> {
    let state = app.state::<AppState>();
    filler_words_repo::add(&state.db, &word, &language).map_err(Into::into)
}

#[tauri::command]
pub async fn delete_filler_word(app: AppHandle, id: String) -> CmdResult<()> {
    let state = app.state::<AppState>();
    filler_words_repo::delete(&state.db, &id).map_err(Into::into)
}

#[tauri::command]
pub async fn reset_filler_words(app: AppHandle, language: String) -> CmdResult<Vec<FillerWord>> {
    let state = app.state::<AppState>();
    filler_words_repo::reset_to_defaults(&state.db, &language)?;
    filler_words_repo::list(&state.db, Some(&language)).map_err(Into::into)
}

#[tauri::command]
pub async fn get_filler_stats(
    app: AppHandle,
    language: Option<String>,
) -> CmdResult<Vec<FillerStat>> {
    let state = app.state::<AppState>();
    filler_words_repo::get_stats(&state.db, language.as_deref()).map_err(Into::into)
}

#[tauri::command]
pub async fn get_filler_total_count(app: AppHandle) -> CmdResult<i64> {
    let state = app.state::<AppState>();
    filler_words_repo::get_total_count(&state.db).map_err(Into::into)
}
