use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::DbConn;
use crate::errors::AppError;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryEntry {
    pub id: String,
    pub phrase: String,
    pub normalized_phrase: String,
    pub language: Option<String>,
    /// "term" | "name" | "acronym" | "product" | "custom"
    pub entry_type: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionRule {
    pub id: String,
    pub source_phrase: String,
    pub normalized_source_phrase: String,
    pub target_phrase: String,
    pub language: Option<String>,
    /// "manual" | "suggested" | "learned"
    pub rule_mode: String,
    pub is_active: bool,
    pub auto_apply: bool,
    pub usage_count: i64,
    pub last_used_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ── Dictionary Entries ────────────────────────────────────────────────────────

pub fn list_entries(db: &DbConn) -> Result<Vec<DictionaryEntry>, AppError> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, phrase, normalized_phrase, language, entry_type, notes, created_at, updated_at
         FROM dictionary_entries ORDER BY phrase COLLATE NOCASE",
    )?;
    let rows = stmt.query_map([], |row| Ok(entry_from_row(row)))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row??);
    }
    Ok(out)
}

pub fn create_entry(
    db: &DbConn,
    phrase: &str,
    language: Option<&str>,
    entry_type: &str,
    notes: Option<&str>,
) -> Result<DictionaryEntry, AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    let normalized = phrase.to_lowercase();
    conn.execute(
        "INSERT INTO dictionary_entries
            (id, phrase, normalized_phrase, language, entry_type, notes, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, phrase, normalized, language, entry_type, notes, now, now],
    )?;
    Ok(DictionaryEntry {
        id,
        phrase: phrase.to_string(),
        normalized_phrase: normalized,
        language: language.map(String::from),
        entry_type: entry_type.to_string(),
        notes: notes.map(String::from),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn update_entry(
    db: &DbConn,
    id: &str,
    phrase: &str,
    language: Option<&str>,
    entry_type: &str,
    notes: Option<&str>,
) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    let normalized = phrase.to_lowercase();
    conn.execute(
        "UPDATE dictionary_entries
         SET phrase = ?1, normalized_phrase = ?2, language = ?3,
             entry_type = ?4, notes = ?5, updated_at = ?6
         WHERE id = ?7",
        params![phrase, normalized, language, entry_type, notes, now, id],
    )?;
    Ok(())
}

pub fn delete_entry(db: &DbConn, id: &str) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM dictionary_entries WHERE id = ?1", params![id])?;
    Ok(())
}

// ── Correction Rules ──────────────────────────────────────────────────────────

/// Returns all active, auto-apply rules — optionally filtered by language.
/// Language `None` returns rules that apply to all languages (language IS NULL)
/// as well as rules matching the given language code.
pub fn list_active_rules(db: &DbConn, language: Option<&str>) -> Result<Vec<CorrectionRule>, AppError> {
    let conn = db.lock().unwrap();
    if let Some(lang) = language {
        let mut stmt = conn.prepare(
            "SELECT id, source_phrase, normalized_source_phrase, target_phrase, language,
                    rule_mode, is_active, auto_apply, usage_count, last_used_at,
                    created_at, updated_at
             FROM correction_rules
             WHERE is_active = 1 AND auto_apply = 1
               AND (language IS NULL OR language = ?1)
             ORDER BY usage_count DESC",
        )?;
        let rows: Vec<CorrectionRule> = stmt
            .query_map(params![lang], |row| rule_from_row(row))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(rows)
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, source_phrase, normalized_source_phrase, target_phrase, language,
                    rule_mode, is_active, auto_apply, usage_count, last_used_at,
                    created_at, updated_at
             FROM correction_rules
             WHERE is_active = 1 AND auto_apply = 1 AND language IS NULL
             ORDER BY usage_count DESC",
        )?;
        let rows: Vec<CorrectionRule> = stmt
            .query_map([], |row| rule_from_row(row))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(rows)
    }
}

pub fn list_rules(db: &DbConn) -> Result<Vec<CorrectionRule>, AppError> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, source_phrase, normalized_source_phrase, target_phrase, language,
                rule_mode, is_active, auto_apply, usage_count, last_used_at,
                created_at, updated_at
         FROM correction_rules ORDER BY usage_count DESC, source_phrase COLLATE NOCASE",
    )?;
    let rows = stmt.query_map([], |row| Ok(rule_from_row(row)))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row??);
    }
    Ok(out)
}

pub fn create_rule(
    db: &DbConn,
    source_phrase: &str,
    target_phrase: &str,
    language: Option<&str>,
    rule_mode: &str,
    auto_apply: bool,
) -> Result<CorrectionRule, AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    let normalized_source = source_phrase.to_lowercase();
    conn.execute(
        "INSERT INTO correction_rules
            (id, source_phrase, normalized_source_phrase, target_phrase, language,
             rule_mode, is_active, auto_apply, usage_count, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, 0, ?8, ?9)",
        params![
            id, source_phrase, normalized_source, target_phrase, language,
            rule_mode, auto_apply as i64, now, now
        ],
    )?;
    Ok(CorrectionRule {
        id,
        source_phrase: source_phrase.to_string(),
        normalized_source_phrase: normalized_source,
        target_phrase: target_phrase.to_string(),
        language: language.map(String::from),
        rule_mode: rule_mode.to_string(),
        is_active: true,
        auto_apply,
        usage_count: 0,
        last_used_at: None,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn update_rule(
    db: &DbConn,
    id: &str,
    source_phrase: &str,
    target_phrase: &str,
    language: Option<&str>,
    is_active: bool,
    auto_apply: bool,
) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    let normalized_source = source_phrase.to_lowercase();
    conn.execute(
        "UPDATE correction_rules
         SET source_phrase = ?1, normalized_source_phrase = ?2, target_phrase = ?3,
             language = ?4, is_active = ?5, auto_apply = ?6, updated_at = ?7
         WHERE id = ?8",
        params![
            source_phrase, normalized_source, target_phrase, language,
            is_active as i64, auto_apply as i64, now, id
        ],
    )?;
    Ok(())
}

pub fn delete_rule(db: &DbConn, id: &str) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM correction_rules WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn increment_rule_usage(db: &DbConn, id: &str) -> Result<(), AppError> {
    let conn = db.lock().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE correction_rules
         SET usage_count = usage_count + 1, last_used_at = ?1, updated_at = ?1
         WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}

// ── Row mappers ───────────────────────────────────────────────────────────────

fn entry_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DictionaryEntry> {
    Ok(DictionaryEntry {
        id: row.get(0)?,
        phrase: row.get(1)?,
        normalized_phrase: row.get(2)?,
        language: row.get(3)?,
        entry_type: row.get(4)?,
        notes: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

fn rule_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<CorrectionRule> {
    Ok(CorrectionRule {
        id: row.get(0)?,
        source_phrase: row.get(1)?,
        normalized_source_phrase: row.get(2)?,
        target_phrase: row.get(3)?,
        language: row.get(4)?,
        rule_mode: row.get(5)?,
        is_active: row.get::<_, i64>(6)? != 0,
        auto_apply: row.get::<_, i64>(7)? != 0,
        usage_count: row.get(8)?,
        last_used_at: row.get(9)?,
        created_at: row.get(10)?,
        updated_at: row.get(11)?,
    })
}
