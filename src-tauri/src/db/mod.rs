use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub mod migrations;
pub mod models;
pub mod repositories;

pub type DbConn = Arc<Mutex<Connection>>;

/// Opens (or creates) the SQLite database in the platform app-data directory.
/// Runs all pending migrations before returning.
/// If migrations fail, the corrupt database is renamed to a timestamped backup
/// and a fresh database is created so the app can always start.
pub fn open(app: &tauri::AppHandle) -> Result<DbConn, String> {
    let app_dir: PathBuf = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Cannot resolve app data dir: {e}"))?;

    std::fs::create_dir_all(&app_dir).map_err(|e| format!("Cannot create app data dir: {e}"))?;

    let db_path = app_dir.join("localvoice.db");

    open_with_recovery(&db_path)
}

fn open_with_recovery(db_path: &std::path::Path) -> Result<DbConn, String> {
    match try_open(db_path) {
        Ok(conn) => Ok(conn),
        Err(e) => {
            // Migration or open failed — back up the broken DB and start fresh.
            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let backup_path = db_path.with_file_name(format!("localvoice.db.bak-{ts}"));
            let _ = std::fs::rename(db_path, &backup_path);
            log::warn!(
                "DB open/migration failed ({e}). Backed up to {} and created a fresh database.",
                backup_path.display()
            );
            try_open(db_path)
        }
    }
}

fn try_open(db_path: &std::path::Path) -> Result<DbConn, String> {
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Cannot open database at {}: {e}", db_path.display()))?;

    // Enable WAL mode and foreign keys for every connection.
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA foreign_keys=ON;",
    )
    .map_err(|e| format!("Cannot set PRAGMAs: {e}"))?;

    migrations::run(&conn).map_err(|e| format!("Migration failed: {e}"))?;

    Ok(Arc::new(Mutex::new(conn)))
}
