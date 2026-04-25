use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::DbConn;
use crate::errors::AppError;

/// Row from the `ambiguous_terms` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmbiguousTerm {
    pub id: String,
    pub phrase: String,
    pub normalized_phrase: String,
    pub language: Option<String>,
    pub occurrences: i64,
    pub avg_confidence: Option<f64>,
    pub last_seen_at: String,
    pub suggested_target: Option<String>,
    pub dismissed: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Upserts an ambiguous term. If a row already exists for `normalized_phrase`
/// (and optional language), increments `occurrences`, updates `avg_confidence`
/// (rolling average) and `last_seen_at`. Also resets `dismissed` to 0 if the
/// new occurrence count exceeds `dismissed_at_occurrences + 5`.
pub fn upsert(
    db: &DbConn,
    phrase: &str,
    language: Option<&str>,
    confidence: Option<f32>,
) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    let normalized = phrase.to_lowercase();
    let conf_f64 = confidence.map(|c| c as f64);

    // Try to find existing row.
    let existing: Option<(String, i64, Option<f64>, i64)> = {
        let mut stmt = conn.prepare(
            "SELECT id, occurrences, avg_confidence, dismissed_at_occurrences
             FROM ambiguous_terms
             WHERE normalized_phrase = ?1 AND (language IS ?2)",
        )?;
        stmt.query_row(params![normalized, language], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, Option<f64>>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .ok()
    };

    if let Some((id, occ, avg_conf, dismissed_at)) = existing {
        let new_occ = occ + 1;
        // Rolling average for confidence.
        let new_avg = match (avg_conf, conf_f64) {
            (Some(avg), Some(c)) => Some((avg * occ as f64 + c) / new_occ as f64),
            (Some(avg), None) => Some(avg),
            (None, Some(c)) => Some(c),
            (None, None) => None,
        };
        // Re-surface if 5+ new occurrences have accumulated since last dismissal.
        let reset_dismissed = new_occ >= dismissed_at + 5;

        conn.execute(
            "UPDATE ambiguous_terms
             SET occurrences = ?1, avg_confidence = ?2, last_seen_at = ?3,
                 updated_at = ?3, dismissed = CASE WHEN ?4 THEN 0 ELSE dismissed END
             WHERE id = ?5",
            params![new_occ, new_avg, now, reset_dismissed as i64, id],
        )?;
    } else {
        // Insert new row.
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO ambiguous_terms
                (id, phrase, normalized_phrase, language, occurrences, avg_confidence,
                 last_seen_at, dismissed, dismissed_at_occurrences, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6, 0, 0, ?6, ?6)",
            params![id, phrase, normalized, language, conf_f64, now],
        )?;
    }

    Ok(())
}

/// Lists active (non-dismissed) ambiguous terms with enough occurrences.
/// Also surfaces previously dismissed terms if they've accumulated 5+ new occurrences.
pub fn list_active(db: &DbConn, min_occurrences: i64) -> Result<Vec<AmbiguousTerm>, AppError> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, phrase, normalized_phrase, language, occurrences, avg_confidence,
                last_seen_at, suggested_target, dismissed, created_at, updated_at
         FROM ambiguous_terms
         WHERE occurrences >= ?1
           AND (dismissed = 0 OR occurrences >= dismissed_at_occurrences + 5)
         ORDER BY occurrences DESC, avg_confidence ASC",
    )?;
    let rows: Vec<AmbiguousTerm> = stmt
        .query_map(params![min_occurrences], |row| term_from_row(row))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

pub fn set_suggested_target(db: &DbConn, id: &str, target: Option<&str>) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE ambiguous_terms SET suggested_target = ?1, updated_at = ?2 WHERE id = ?3",
        params![target, now, id],
    )?;
    Ok(())
}

pub fn dismiss(db: &DbConn, id: &str) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE ambiguous_terms
         SET dismissed = 1, dismissed_at_occurrences = occurrences, updated_at = ?1
         WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}

/// Marks a term as resolved (dismissed=1) after the user accepted a suggestion.
pub fn mark_resolved(db: &DbConn, id: &str) -> Result<(), AppError> {
    dismiss(db, id)
}

pub fn get(db: &DbConn, id: &str) -> Result<Option<AmbiguousTerm>, AppError> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, phrase, normalized_phrase, language, occurrences, avg_confidence,
                last_seen_at, suggested_target, dismissed, created_at, updated_at
         FROM ambiguous_terms WHERE id = ?1",
    )?;
    let row = stmt
        .query_row(params![id], |row| Ok(term_from_row(row)))
        .ok();
    Ok(row.and_then(|r| r.ok()))
}

fn term_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AmbiguousTerm> {
    Ok(AmbiguousTerm {
        id: row.get(0)?,
        phrase: row.get(1)?,
        normalized_phrase: row.get(2)?,
        language: row.get(3)?,
        occurrences: row.get(4)?,
        avg_confidence: row.get(5)?,
        last_seen_at: row.get(6)?,
        suggested_target: row.get(7)?,
        dismissed: row.get::<_, i64>(8)? != 0,
        created_at: row.get(9)?,
        updated_at: row.get(10)?,
    })
}
