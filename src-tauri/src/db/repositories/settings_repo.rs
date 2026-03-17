use crate::db::DbConn;
use rusqlite::Result;
use std::collections::HashMap;

/// Returns all settings as a key→value map.
pub fn get_all(db: &DbConn) -> Result<HashMap<String, String>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    rows.collect()
}

/// Upserts a single setting row.
pub fn upsert(db: &DbConn, key: &str, value: &str) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO settings (key, value, updated_at)
         VALUES (?1, ?2, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        rusqlite::params![key, value],
    )?;
    Ok(())
}
