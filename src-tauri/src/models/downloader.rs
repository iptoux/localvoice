use std::io::Write as IoWrite;
use std::path::PathBuf;

use futures_util::StreamExt;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::errors::AppError;

/// Payload for the `model-download-progress` event emitted during downloads.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub key: String,
    pub percent: u8,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
}

/// Downloads `url` to `dest_path`, writing to a `.tmp` sidecar file during the
/// transfer and atomically renaming it to `dest_path` on success.
///
/// Progress is emitted via the `model-download-progress` Tauri event (throttled
/// to one event per percent change to avoid flooding the frontend).
///
/// On failure, any partially written `.tmp` file is removed by `cleanup_tmp`.
pub async fn download(
    app: &AppHandle,
    key: &str,
    url: &str,
    dest_path: &PathBuf,
    hint_size_bytes: u64,
) -> Result<(), AppError> {
    let tmp_path = dest_path.with_extension("bin.tmp");

    // Ensure the models directory exists.
    if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AppError(format!("Cannot create models directory: {e}")))?;
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| AppError(format!("Download request failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError(format!(
            "HTTP {} while downloading {url}",
            resp.status()
        )));
    }

    let content_length = resp.content_length().unwrap_or(hint_size_bytes);
    let mut file = std::fs::File::create(&tmp_path)
        .map_err(|e| AppError(format!("Cannot create temp file: {e}")))?;

    let mut stream = resp.bytes_stream();
    let mut downloaded: u64 = 0;
    let mut last_percent: i16 = -1;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| AppError(format!("Download stream error: {e}")))?;
        file.write_all(&chunk)
            .map_err(|e| AppError(format!("Write error: {e}")))?;
        downloaded += chunk.len() as u64;

        let percent = if content_length > 0 {
            ((downloaded * 100) / content_length).min(100) as i16
        } else {
            0
        };

        if percent != last_percent {
            last_percent = percent;
            let _ = app.emit(
                "model-download-progress",
                DownloadProgress {
                    key: key.to_string(),
                    percent: percent as u8,
                    bytes_downloaded: downloaded,
                    total_bytes: content_length,
                },
            );
        }
    }

    // Atomic rename: .tmp → final path.
    std::fs::rename(&tmp_path, dest_path)
        .map_err(|e| AppError(format!("Cannot rename downloaded file: {e}")))?;

    Ok(())
}

/// Removes the `.tmp` sidecar file for `dest_path` if it exists.
/// Call this on failure or cancellation to avoid leaving partial files on disk.
pub fn cleanup_tmp(dest_path: &PathBuf) {
    let tmp_path = dest_path.with_extension("bin.tmp");
    let _ = std::fs::remove_file(&tmp_path);
}
