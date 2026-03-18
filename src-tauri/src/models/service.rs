use std::path::PathBuf;

use serde::Serialize;
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::db::repositories::models_repo;
use crate::errors::AppError;
use crate::logging::push_log;
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
    /// All languages for which this model is the default (from model_language_defaults table).
    pub default_for_languages: Vec<String>,
    pub local_path: Option<String>,
    pub installed_at: Option<String>,
    // Extended metadata from registry
    pub description: String,
    pub speed: String,
    pub accuracy: String,
    pub category: String,
    pub recommended_for: String,
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
    let all_defaults = models_repo::get_all_defaults(&state.db).unwrap_or_default();

    let models = registry::REGISTRY
        .iter()
        .map(|def| {
            let inst = installed.iter().find(|i| i.model_key == def.key);
            let default_for_languages: Vec<String> = all_defaults
                .iter()
                .filter(|(_, k)| k == def.key)
                .map(|(lang, _)| lang.clone())
                .collect();
            ModelInfo {
                key: def.key.to_string(),
                display_name: def.display_name.to_string(),
                language_scope: def.language_scope.to_string(),
                file_size_bytes: def.file_size_bytes,
                installed: inst.is_some(),
                is_default_for_de: inst.map(|i| i.is_default_for_de).unwrap_or(false),
                is_default_for_en: inst.map(|i| i.is_default_for_en).unwrap_or(false),
                default_for_languages,
                local_path: inst.map(|i| i.local_path.clone()),
                installed_at: inst.and_then(|i| i.installed_at.clone()),
                description: def.description.to_string(),
                speed: def.speed.to_string(),
                accuracy: def.accuracy.to_string(),
                category: def.category.to_string(),
                recommended_for: def.recommended_for.to_string(),
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

    // Perform the download — retry up to 3 times on failure.
    const MAX_ATTEMPTS: u32 = 3;
    let mut last_err = None;
    for attempt in 1..=MAX_ATTEMPTS {
        match downloader::download(&app, &key, def.download_url, &dest_path, def.file_size_bytes).await {
            Ok(_) => { last_err = None; break; }
            Err(e) => {
                downloader::cleanup_tmp(&dest_path);
                push_log("warn", "models::download", &format!(
                    "Attempt {attempt}/{MAX_ATTEMPTS} failed for {}: {e}", def.display_name
                ));
                last_err = Some(e);
                if attempt < MAX_ATTEMPTS {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        }
    }
    if let Some(e) = last_err {
        let msg = format!("Failed to download {} after {MAX_ATTEMPTS} attempts: {e}", def.display_name);
        push_log("error", "models::download", &msg);
        let _ = app.notification()
            .builder()
            .title("Download failed")
            .body(&msg)
            .show();
        return Err(e);
    }

    // Verify checksum (skipped if None).
    if let Err(e) = verify::verify_checksum(&dest_path, def.sha256_checksum) {
        let _ = std::fs::remove_file(&dest_path);
        let msg = format!("Checksum verification failed for {}: {e}", def.display_name);
        push_log("error", "models::verify", &msg);
        let _ = app.notification()
            .builder()
            .title("Download failed")
            .body(&msg)
            .show();
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

    let msg = format!("Model \"{}\" downloaded and ready.", def.display_name);
    push_log("info", "models::download", &msg);
    let _ = app.notification()
        .builder()
        .title("Model ready")
        .body(&msg)
        .show();

    Ok(())
}

/// Deletes the model file from disk and clears the DB install record.
pub fn delete_model(app: &AppHandle, key: &str) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    let display_name = registry::find(key)
        .map(|d| d.display_name)
        .unwrap_or(key);

    if let Some(record) = models_repo::get(&state.db, key)? {
        let path = PathBuf::from(&record.local_path);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| AppError(format!("Cannot delete model file: {e}")))?;
        }
    }

    models_repo::mark_uninstalled(&state.db, key)?;
    push_log("info", "models::delete", &format!("Model \"{display_name}\" deleted."));
    Ok(())
}

/// Sets the default model for any language.
pub fn set_default_model(app: &AppHandle, language: &str, key: &str) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    models_repo::set_default_for_language(&state.db, language, key)?;
    Ok(())
}
