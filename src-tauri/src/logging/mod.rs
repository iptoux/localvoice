use serde::Serialize;
use std::sync::{Arc, OnceLock, RwLock};

/// A single captured log entry (warn or error level).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub id: String,
    pub level: String,
    pub area: String,
    pub message: String,
    pub created_at: String,
}

/// Maximum number of entries retained in memory.
const MAX_ENTRIES: usize = 1000;

static LOG_BUFFER: OnceLock<Arc<RwLock<Vec<LogEntry>>>> = OnceLock::new();

/// Initialises the global in-memory logger. Call once in app setup.
pub fn init() {
    let buf = Arc::new(RwLock::new(Vec::with_capacity(MAX_ENTRIES)));
    LOG_BUFFER.set(buf).ok();
    log::set_boxed_logger(Box::new(AppLogger)).ok();
    // Allow info through for stderr output; only warn+ are buffered.
    log::set_max_level(log::LevelFilter::Info);
}

/// Returns a clone of the shared log buffer handle.
pub fn get_buffer() -> Option<Arc<RwLock<Vec<LogEntry>>>> {
    LOG_BUFFER.get().cloned()
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
        // Always mirror to stderr so dev builds have console output.
        eprintln!(
            "[{}] {} — {}",
            record.level(),
            record.target(),
            record.args()
        );

        // Buffer only warn/error for the in-app Logs page.
        if record.level() > log::Level::Warn {
            return;
        }

        if let Some(buf) = LOG_BUFFER.get() {
            if let Ok(mut lock) = buf.write() {
                if lock.len() >= MAX_ENTRIES {
                    lock.remove(0);
                }
                lock.push(LogEntry {
                    id: uuid::Uuid::new_v4().to_string(),
                    level: record.level().to_string().to_lowercase(),
                    area: record.target().to_string(),
                    message: record.args().to_string(),
                    created_at: chrono::Utc::now().to_rfc3339(),
                });
            }
        }
    }

    fn flush(&self) {}
}
