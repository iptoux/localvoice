use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::db::repositories::models_repo;
use crate::errors::AppError;
use crate::models::{downloader, registry, verify};
use crate::state::AppState;

/// Merged view of registry metadata + install state — returned by `list_available_models`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub key: String,
    pub display_name: String,
    pub language_scope: String,
    pub file_size_bytes: u64,
    pub installed: bool,
    pub is_default_for_de: bool,
    pub is_default_for_en: bool,
    pub local_path: Option<String>,
    pub installed_at: Option<String>,
}

/// Returns the models storage directory: `{app_data_dir}/models/`.
pub fn models_dir(app: &AppHandle) -> Result<PathBuf, AppError> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError(format!("Cannot resolve app data dir: {e}")))?;
    Ok(data_dir.join("models"))
}

/// Returns the list of all known models merged with their current install state.
pub fn list_available(app: &AppHandle) -> Result<Vec<ModelInfo>, AppError> {
    let state = app.state::<AppState>();
    let installed = models_repo::list_installed(&state.db)?;

    let models = registry::REGISTRY
        .iter()
        .map(|def| {
            let inst = installed.iter().find(|i| i.model_key == def.key);
            ModelInfo {
                key: def.key.to_string(),
                display_name: def.display_name.to_string(),
                language_scope: def.language_scope.to_string(),
                file_size_bytes: def.file_size_bytes,
                installed: inst.is_some(),
                is_default_for_de: inst.map(|i| i.is_default_for_de).unwrap_or(false),
                is_default_for_en: inst.map(|i| i.is_default_for_en).unwrap_or(false),
                local_path: inst.map(|i| i.local_path.clone()),
                installed_at: inst.and_then(|i| i.installed_at.clone()),
            }
        })
        .collect();

    Ok(models)
}

/// Downloads a model, verifies its checksum, and records it as installed in the DB.
///
/// Emits `model-download-progress { key, percent, bytesDownloaded, totalBytes }`
/// events during the download.
pub async fn download_model(app: AppHandle, key: String) -> Result<(), AppError> {
    let def = registry::find(&key)
        .ok_or_else(|| AppError(format!("Unknown model key: {key}")))?;

    let dest_dir = models_dir(&app)?;
    let dest_path = dest_dir.join(format!("{}.bin", key));

    // Perform the download.
    if let Err(e) = downloader::download(
        &app,
        &key,
        def.download_url,
        &dest_path,
        def.file_size_bytes,
    )
    .await
    {
        downloader::cleanup_tmp(&dest_path);
        return Err(e);
    }

    // Verify checksum (skipped if None).
    if let Err(e) = verify::verify_checksum(&dest_path, def.sha256_checksum) {
        let _ = std::fs::remove_file(&dest_path);
        return Err(e);
    }

    // Record installation in DB.
    let state = app.state::<AppState>();
    models_repo::upsert(
        &state.db,
        def.key,
        def.display_name,
        def.language_scope,
        &dest_path.to_string_lossy(),
        Some(def.file_size_bytes),
        def.sha256_checksum,
        true,
    )?;

    Ok(())
}

/// Deletes the model file from disk and clears the DB install record.
pub fn delete_model(app: &AppHandle, key: &str) -> Result<(), AppError> {
    let state = app.state::<AppState>();

    if let Some(record) = models_repo::get(&state.db, key)? {
        let path = PathBuf::from(&record.local_path);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| AppError(format!("Cannot delete model file: {e}")))?;
        }
    }

    models_repo::mark_uninstalled(&state.db, key)?;
    Ok(())
}

/// Sets the default model for `language` ("de" | "en").
pub fn set_default_model(app: &AppHandle, language: &str, key: &str) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    models_repo::set_default_for_language(&state.db, language, key)?;
    Ok(())
}
