use tauri::AppHandle;

use crate::errors::CmdResult;
use crate::models::service::{self, ModelInfo};

/// Returns all models from the registry merged with their current install state.
#[tauri::command]
pub fn list_available_models(app: AppHandle) -> CmdResult<Vec<ModelInfo>> {
    service::list_available(&app).map_err(Into::into)
}

/// Downloads, verifies, and installs a model by key.
/// Emits `model-download-progress` events during the transfer.
#[tauri::command]
pub async fn download_model(app: AppHandle, key: String) -> CmdResult<()> {
    service::download_model(app, key).await.map_err(Into::into)
}

/// Deletes the model file from disk and clears its install record.
#[tauri::command]
pub fn delete_model(app: AppHandle, key: String) -> CmdResult<()> {
    service::delete_model(&app, &key).map_err(Into::into)
}

/// Sets the default model for the given language ("de" or "en").
#[tauri::command]
pub fn set_default_model(app: AppHandle, language: String, key: String) -> CmdResult<()> {
    service::set_default_model(&app, &language, &key).map_err(Into::into)
}
