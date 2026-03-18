use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::DbConn;
use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillerWord {
    pub id: String,
    pub word: String,
    pub language: String,
    pub is_default: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillerStat {
    pub word: String,
    pub language: String,
    pub count: i64,
    pub last_removed_at: String,
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<FillerWord> {
    Ok(FillerWord {
        id: row.get(0)?,
        word: row.get(1)?,
        language: row.get(2)?,
        is_default: row.get::<_, i64>(3)? != 0,
        created_at: row.get(4)?,
    })
}

pub fn list(db: &DbConn, language: Option<&str>) -> Result<Vec<FillerWord>, AppError> {
    let conn = db.lock().unwrap();
    let rows: Vec<FillerWord> = match language {
        Some(l) => {
            let mut stmt = conn.prepare(
                "SELECT id, word, language, is_default, created_at FROM filler_words WHERE language = ?1 ORDER BY word COLLATE NOCASE",
            )?;
            let x = stmt.query_map(params![l], map_row)?.collect::<rusqlite::Result<Vec<_>>>()?; x
        }
        None => {
            let mut stmt = conn.prepare(
                "SELECT id, word, language, is_default, created_at FROM filler_words ORDER BY language, word COLLATE NOCASE",
            )?;
            let x = stmt.query_map([], map_row)?.collect::<rusqlite::Result<Vec<_>>>()?; x
        }
    };
    Ok(rows)
}

pub fn list_words_for_language(db: &DbConn, language: &str) -> Result<Vec<String>, AppError> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT word FROM filler_words WHERE language = ?1 ORDER BY length(word) DESC",
    )?;
    let x = stmt
        .query_map(params![language], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(x)
}

pub fn add(db: &DbConn, word: &str, language: &str) -> Result<FillerWord, AppError> {
    let conn = db.lock().unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO filler_words (id, word, language, is_default, created_at) VALUES (?1, ?2, ?3, 0, ?4)",
        params![id, word, language, now],
    )?;
    Ok(FillerWord { id, word: word.to_string(), language: language.to_string(), is_default: false, created_at: now })
}

pub fn delete(db: &DbConn, id: &str) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM filler_words WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn reset_to_defaults(db: &DbConn, language: &str) -> Result<(), AppError> {
    {
        let conn = db.lock().unwrap();
        conn.execute("DELETE FROM filler_words WHERE language = ?1", params![language])?;
    }
    seed_defaults(db, language)
}

/// Records which filler words were removed in a session.
pub fn log_removals(db: &DbConn, session_id: Option<&str>, words: &[String], language: &str) -> Result<(), AppError> {
    if words.is_empty() { return Ok(()); }
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    for word in words {
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO filler_removal_log (id, session_id, word, language, removed_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, session_id, word, language, now],
        )?;
    }
    Ok(())
}

/// Returns top removed filler words, optionally filtered by language, ordered by count desc.
pub fn get_stats(db: &DbConn, language: Option<&str>) -> Result<Vec<FillerStat>, AppError> {
    let conn = db.lock().unwrap();
    let rows: Vec<FillerStat> = match language {
        Some(l) => {
            let mut stmt = conn.prepare(
                "SELECT word, language, COUNT(*) as count, MAX(removed_at) as last_removed_at
                 FROM filler_removal_log WHERE language = ?1
                 GROUP BY word, language ORDER BY count DESC LIMIT 20",
            )?;
            let x = stmt.query_map(params![l], |row| Ok(FillerStat {
                word: row.get(0)?,
                language: row.get(1)?,
                count: row.get(2)?,
                last_removed_at: row.get(3)?,
            }))?.collect::<rusqlite::Result<Vec<_>>>()?; x
        }
        None => {
            let mut stmt = conn.prepare(
                "SELECT word, language, COUNT(*) as count, MAX(removed_at) as last_removed_at
                 FROM filler_removal_log
                 GROUP BY word, language ORDER BY count DESC LIMIT 20",
            )?;
            let x = stmt.query_map([], |row| Ok(FillerStat {
                word: row.get(0)?,
                language: row.get(1)?,
                count: row.get(2)?,
                last_removed_at: row.get(3)?,
            }))?.collect::<rusqlite::Result<Vec<_>>>()?; x
        }
    };
    Ok(rows)
}

/// Returns total filler removals count (for stat card).
pub fn get_total_count(db: &DbConn) -> Result<i64, AppError> {
    let conn = db.lock().unwrap();
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM filler_removal_log", [], |row| row.get(0)
    )?;
    Ok(count)
}

fn seed_defaults(db: &DbConn, language: &str) -> Result<(), AppError> {
    let defaults: &[&str] = match language {
        "de" => &["äh", "ähm", "öhm", "hm", "hmm", "mhm", "halt", "sozusagen",
                  "quasi", "irgendwie", "also", "ja", "ne", "naja", "gewissermaßen", "sagen wir mal"],
        "en" => &["uh", "um", "uhm", "hmm", "hm", "mhm", "you know", "like",
                  "basically", "actually", "sort of", "kind of", "i mean", "right", "well", "so", "okay so"],
        "fr" => &["euh", "bah", "ben", "hein", "voilà", "quoi", "genre", "du coup", "en fait", "tu vois"],
        "es" => &["eh", "este", "pues", "bueno", "o sea", "a ver", "sabes", "mira"],
        "it" => &["allora", "cioè", "quindi", "praticamente", "tipo", "ecco", "vabbè", "insomma"],
        "pt" => &["né", "então", "tipo", "sabe", "assim", "ahn", "bom"],
        "nl" => &["eh", "uhm", "nou", "eigenlijk", "gewoon", "zeg maar", "weet je", "toch"],
        "pl" => &["eee", "yyy", "no", "właśnie", "znaczy", "tak jakby", "wiesz"],
        "ru" => &["э", "ну", "вот", "значит", "короче", "типа", "как бы", "понимаешь"],
        "ja" => &["えーと", "あの", "まあ", "なんか", "ちょっと", "そうですね"],
        "zh" => &["那个", "就是", "然后", "这个", "嗯", "啊", "对对对"],
        _ => &[],
    };
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    for word in defaults {
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT OR IGNORE INTO filler_words (id, word, language, is_default, created_at) VALUES (?1, ?2, ?3, 1, ?4)",
            params![id, word, language, now],
        )?;
    }
    Ok(())
}
