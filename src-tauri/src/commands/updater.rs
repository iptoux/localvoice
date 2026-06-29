use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_updater::{Update, UpdaterExt};

use crate::db::repositories::settings_repo;
use crate::errors::{AppError, CmdResult};
use crate::logging::push_log;
use crate::state::AppState;

const UPDATE_AVAILABLE_EVENT: &str = "update-available";
const UPDATE_PROGRESS_EVENT: &str = "update-download-progress";
const UPDATE_ERROR_EVENT: &str = "update-error";

#[derive(Clone, Default)]
pub struct PendingUpdate {
    inner: Arc<PendingUpdateInner>,
}

#[derive(Default)]
struct PendingUpdateInner {
    update: Mutex<Option<Update>>,
    status: Mutex<UpdateStatus>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub version: String,
    pub current_version: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub percent: Option<u8>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatus {
    pub phase: String,
    pub available: Option<UpdateInfo>,
    pub progress: Option<UpdateDownloadProgress>,
    pub last_error: Option<String>,
}

impl Default for UpdateStatus {
    fn default() -> Self {
        Self {
            phase: "idle".to_string(),
            available: None,
            progress: None,
            last_error: None,
        }
    }
}

impl PendingUpdate {
    pub fn status(&self) -> UpdateStatus {
        self.inner.status.lock().unwrap().clone()
    }

    fn set_phase(&self, phase: &str) {
        self.update_status(|status| {
            status.phase = phase.to_string();
            status.last_error = None;
        });
    }

    fn set_error(&self, message: String) {
        self.update_status(|status| {
            status.phase = "error".to_string();
            status.last_error = Some(message);
            status.progress = None;
        });
    }

    fn set_progress(&self, progress: UpdateDownloadProgress) {
        self.update_status(|status| {
            status.phase = "downloading".to_string();
            status.progress = Some(progress);
            status.last_error = None;
        });
    }

    fn set_available(&self, update: Update, info: UpdateInfo) {
        *self.inner.update.lock().unwrap() = Some(update);
        self.update_status(|status| {
            status.phase = "available".to_string();
            status.available = Some(info);
            status.progress = None;
            status.last_error = None;
        });
    }

    fn clear_update(&self, phase: &str) {
        *self.inner.update.lock().unwrap() = None;
        self.update_status(|status| {
            status.phase = phase.to_string();
            status.available = None;
            status.progress = None;
            status.last_error = None;
        });
    }

    fn take_update(&self) -> Option<Update> {
        self.inner.update.lock().unwrap().take()
    }

    fn update_status(&self, f: impl FnOnce(&mut UpdateStatus)) {
        let mut status = self.inner.status.lock().unwrap();
        f(&mut status);
    }
}

#[tauri::command]
pub fn get_update_status(pending_update: State<PendingUpdate>) -> CmdResult<UpdateStatus> {
    Ok(pending_update.status())
}

#[tauri::command]
pub async fn check_for_update(
    app: AppHandle,
    pending_update: State<'_, PendingUpdate>,
    manual: bool,
) -> CmdResult<Option<UpdateInfo>> {
    check_for_update_inner(app, pending_update.inner().clone(), manual).await
}

#[tauri::command]
pub async fn install_pending_update(
    app: AppHandle,
    pending_update: State<'_, PendingUpdate>,
) -> CmdResult<()> {
    let update = match pending_update.take_update() {
        Some(update) => update,
        None => {
            let message = "No pending update to install.".to_string();
            pending_update.set_error(message.clone());
            let _ = app.emit(UPDATE_ERROR_EVENT, &message);
            return Err(AppError(message));
        }
    };

    pending_update.set_phase("downloading");

    let progress_state = pending_update.inner().clone();
    let progress_app = app.clone();
    let mut downloaded: u64 = 0;

    let finished_state = pending_update.inner().clone();
    let finished_app = app.clone();

    update
        .download_and_install(
            move |chunk_length, content_length| {
                downloaded = downloaded.saturating_add(chunk_length as u64);
                let percent = content_length.and_then(|total| {
                    if total == 0 {
                        None
                    } else {
                        Some(((downloaded.saturating_mul(100)) / total).min(100) as u8)
                    }
                });
                let payload = UpdateDownloadProgress {
                    downloaded_bytes: downloaded,
                    total_bytes: content_length,
                    percent,
                };
                progress_state.set_progress(payload.clone());
                let _ = progress_app.emit(UPDATE_PROGRESS_EVENT, payload);
            },
            move || {
                finished_state.set_phase("installing");
                let _ = finished_app.emit(
                    UPDATE_PROGRESS_EVENT,
                    UpdateDownloadProgress {
                        downloaded_bytes: 0,
                        total_bytes: None,
                        percent: Some(100),
                    },
                );
            },
        )
        .await
        .map_err(|e| {
            let message = format!("Update installation failed: {e}");
            pending_update.set_error(message.clone());
            let _ = app.emit(UPDATE_ERROR_EVENT, &message);
            push_log("error", "updater::install", &message);
            AppError(message)
        })?;

    pending_update.set_phase("installing");
    push_log(
        "info",
        "updater::install",
        "Update installed; restarting LocalVoice.",
    );
    app.restart();
}

pub fn spawn_startup_check(app: AppHandle) {
    #[cfg(debug_assertions)]
    {
        log::info!("Skipping automatic updater check in debug builds.");
        let _ = app;
    }

    #[cfg(not(debug_assertions))]
    {
        tauri::async_runtime::spawn(async move {
            let pending_update = app.state::<PendingUpdate>().inner().clone();
            if let Err(e) = check_for_update_inner(app, pending_update, false).await {
                log::warn!("Automatic updater check failed: {e}");
            }
        });
    }
}

async fn check_for_update_inner(
    app: AppHandle,
    pending_update: PendingUpdate,
    manual: bool,
) -> CmdResult<Option<UpdateInfo>> {
    if !manual && !auto_update_enabled(&app) {
        pending_update.clear_update("idle");
        return Ok(None);
    }

    #[cfg(debug_assertions)]
    if !manual {
        pending_update.clear_update("idle");
        return Ok(None);
    }

    pending_update.set_phase("checking");

    let result = app
        .updater()
        .map_err(|e| AppError(format!("Updater is not available: {e}")))?
        .check()
        .await;

    update_last_check(&app);

    match result {
        Ok(Some(update)) => {
            let info = UpdateInfo {
                version: update.version.clone(),
                current_version: update.current_version.clone(),
            };
            pending_update.set_available(update, info.clone());
            let _ = app.emit(UPDATE_AVAILABLE_EVENT, &info);
            let _ = app
                .notification()
                .builder()
                .title("LocalVoice update available")
                .body(format!(
                    "Version {} is ready to download from GitHub releases.",
                    info.version
                ))
                .show();
            push_log(
                "info",
                "updater::check",
                &format!(
                    "Update available: {} (current {})",
                    info.version, info.current_version
                ),
            );
            Ok(Some(info))
        }
        Ok(None) => {
            pending_update.clear_update("upToDate");
            push_log("info", "updater::check", "No update available.");
            Ok(None)
        }
        Err(e) => {
            let message = format!("Update check failed: {e}");
            pending_update.set_error(message.clone());
            let _ = app.emit(UPDATE_ERROR_EVENT, &message);
            push_log("error", "updater::check", &message);
            Err(AppError(message))
        }
    }
}

fn auto_update_enabled(app: &AppHandle) -> bool {
    let state = app.state::<AppState>();
    settings_repo::get_all(&state.db)
        .ok()
        .and_then(|settings| settings.get("app.auto_update").cloned())
        .map(|value| value != "false")
        .unwrap_or(true)
}

fn update_last_check(app: &AppHandle) {
    let state = app.state::<AppState>();
    let now = Utc::now().to_rfc3339();
    if let Err(e) = settings_repo::upsert(&state.db, "app.last_update_check", &now) {
        log::warn!("Failed to persist updater check time: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_status_has_no_pending_update() {
        let pending = PendingUpdate::default();

        assert_eq!(
            pending.status(),
            UpdateStatus {
                phase: "idle".to_string(),
                available: None,
                progress: None,
                last_error: None,
            }
        );
        assert!(pending.take_update().is_none());
    }

    #[test]
    fn error_status_clears_progress() {
        let pending = PendingUpdate::default();
        pending.set_progress(UpdateDownloadProgress {
            downloaded_bytes: 10,
            total_bytes: Some(100),
            percent: Some(10),
        });

        pending.set_error("failed".to_string());

        assert_eq!(pending.status().phase, "error");
        assert_eq!(pending.status().last_error, Some("failed".to_string()));
        assert_eq!(pending.status().progress, None);
    }
}
