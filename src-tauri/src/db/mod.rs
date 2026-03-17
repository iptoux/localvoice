use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub mod migrations;
pub mod repositories;

pub type DbConn = Arc<Mutex<Connection>>;

/// Opens (or creates) the SQLite database in the platform app-data directory.
/// Runs all pending migrations before returning.
pub fn open(app: &tauri::AppHandle) -> Result<DbConn, String> {
    let app_dir: PathBuf = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Cannot resolve app data dir: {e}"))?;

    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Cannot create app data dir: {e}"))?;

    let db_path = app_dir.join("localvoice.db");
    let conn = Connection::open(&db_path)
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
