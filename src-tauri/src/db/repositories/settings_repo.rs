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

/// Resets all settings to factory defaults by clearing the table and re-seeding.
pub fn reset_to_defaults(db: &DbConn) -> Result<()> {
    let conn = db.lock().unwrap();
    conn.execute_batch("DELETE FROM settings;")?;
    conn.execute_batch(
        "INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES
            ('app.theme',                    'system',    datetime('now')),
            ('app.language',                 'de',        datetime('now')),
            ('app.start_hidden',             'false',     datetime('now')),
            ('app.autostart',                'false',     datetime('now')),
            ('ui.default_mode',              'pill',      datetime('now')),
            ('ui.pill.always_on_top',        'true',      datetime('now')),
            ('recording.shortcut',           'CommandOrControl+Shift+Space', datetime('now')),
            ('recording.push_to_talk',       'false',     datetime('now')),
            ('recording.silence_timeout_ms', '1500',      datetime('now')),
            ('transcription.default_language','de',       datetime('now')),
            ('transcription.auto_punctuation','true',     datetime('now')),
            ('transcription.auto_capitalization','true',  datetime('now')),
            ('transcription.remove_fillers', 'false',     datetime('now')),
            ('output.mode',                  'clipboard', datetime('now')),
            ('output.auto_paste',            'false',     datetime('now')),
            ('dictionary.auto_apply_rules',  'true',      datetime('now')),
            ('ambiguity.confidence_threshold','0.6',      datetime('now')),
            ('ambiguity.min_occurrences',    '3',         datetime('now')),
            ('notifications.on_error',       'true',      datetime('now')),
            ('notifications.on_success',     'false',     datetime('now'));",
    )?;
    Ok(())
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
