use rusqlite::params;
use serde::Serialize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    OnceLock,
};
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::db::DbConn;

/// A single captured log entry.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub id: String,
    pub level: String,
    pub area: String,
    pub message: String,
    pub created_at: String,
}

static LOG_SENDER: OnceLock<UnboundedSender<LogEntry>> = OnceLock::new();
/// Shared DB handle so `commands/logs.rs` can query from it.
static LOG_DB: OnceLock<DbConn> = OnceLock::new();
static LOGGING_ENABLED: AtomicBool = AtomicBool::new(true);

/// Initialises the persistent logger.
///
/// Call once in app setup **after** the DB is open.  Spawns a background
/// tokio task that writes every log entry to the `app_logs` table so entries
/// survive app restarts.
pub fn init(enabled: bool, db: DbConn) {
    LOGGING_ENABLED.store(enabled, Ordering::Relaxed);
    LOG_DB.set(db.clone()).ok();

    let (tx, mut rx) = mpsc::unbounded_channel::<LogEntry>();
    LOG_SENDER.set(tx).ok();

    // Background writer — receives entries and persists them to SQLite.
    tauri::async_runtime::spawn(async move {
        while let Some(entry) = rx.recv().await {
            let db = db.clone();
            // spawn_blocking so we don't block the async executor with a sync
            // SQLite call (rusqlite is synchronous).
            tauri::async_runtime::spawn_blocking(move || {
                if let Ok(conn) = db.lock() {
                    conn.execute(
                        "INSERT OR IGNORE INTO app_logs
                             (id, level, area, message, created_at)
                         VALUES (?1, ?2, ?3, ?4, ?5)",
                        params![
                            entry.id,
                            entry.level,
                            entry.area,
                            entry.message,
                            entry.created_at
                        ],
                    )
                    .ok();
                }
            });
        }
    });

    log::set_boxed_logger(Box::new(AppLogger)).ok();
    // Allow info through for stderr; warn/error/info get buffered.
    log::set_max_level(log::LevelFilter::Info);
}

/// Enable or disable log buffering at runtime.
pub fn set_enabled(enabled: bool) {
    LOGGING_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Returns the shared DB handle so log commands can query persisted entries.
pub fn get_db() -> Option<DbConn> {
    LOG_DB.get().cloned()
}

/// Pushes a structured app event into the log at any level.
///
/// Use this for structured events (model downloaded, session created, etc.)
/// that should always appear in the Logs panel.
pub fn push_log(level: &str, area: &str, message: &str) {
    if !LOGGING_ENABLED.load(Ordering::Relaxed) {
        return;
    }
    send_entry(level, area, message);
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn send_entry(level: &str, area: &str, message: &str) {
    if let Some(tx) = LOG_SENDER.get() {
        let entry = LogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            level: level.to_string(),
            area: area.to_string(),
            message: message.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        tx.send(entry).ok();
    }
}

// ── Logger implementation ─────────────────────────────────────────────────────

struct AppLogger;

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        // Mirror every record to stderr for dev builds.
        eprintln!(
            "[{}] {} — {}",
            record.level(),
            record.target(),
            record.args()
        );

        // Only buffer warn/error/info for the Logs page.
        if record.level() > log::Level::Info {
            return;
        }

        if !LOGGING_ENABLED.load(Ordering::Relaxed) {
            return;
        }

        send_entry(
            &record.level().to_string().to_lowercase(),
            record.target(),
            &record.args().to_string(),
        );
    }

    fn flush(&self) {}
}
