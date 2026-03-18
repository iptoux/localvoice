use std::path::Path;

use crate::db::DbConn;
use crate::db::repositories::settings_repo;

/// Deletes audio files older than the configured retention period.
///
/// Runs once on startup. Skips silently if `recording.keep_audio` is disabled
/// or the audio directory does not exist.
pub fn cleanup_old_audio(db: &DbConn, audio_dir: &Path) {
    let settings = settings_repo::get_all(db).unwrap_or_default();

    let keep_audio = settings
        .get("recording.keep_audio")
        .map(|v| v == "true")
        .unwrap_or(false);

    if !keep_audio {
        return;
    }

    let retention_days: u64 = settings
        .get("recording.audio_retention_days")
        .and_then(|v| v.parse().ok())
        .unwrap_or(7);

    if !audio_dir.exists() {
        return;
    }

    let max_age = std::time::Duration::from_secs(retention_days * 24 * 60 * 60);
    let now = std::time::SystemTime::now();
    let mut removed = 0u32;

    let entries = match std::fs::read_dir(audio_dir) {
        Ok(e) => e,
        Err(e) => {
            log::warn!("Failed to read audio dir for cleanup: {e}");
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("wav") {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let modified = match metadata.modified() {
            Ok(t) => t,
            Err(_) => continue,
        };

        if let Ok(age) = now.duration_since(modified) {
            if age > max_age {
                // Also clear the audio_path in any session referencing this file.
                let path_str = path.to_string_lossy();
                let conn = db.lock().unwrap();
                let _ = conn.execute(
                    "UPDATE sessions SET audio_path = NULL WHERE audio_path = ?1",
                    rusqlite::params![path_str.as_ref()],
                );
                drop(conn);

                if std::fs::remove_file(&path).is_ok() {
                    removed += 1;
                }
            }
        }
    }

    if removed > 0 {
        log::info!("Audio cleanup: removed {removed} file(s) older than {retention_days} days");
    }
}
