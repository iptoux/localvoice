use crate::db::repositories::dictionary_repo::{
    self, CorrectionRule, DictionaryEntry,
};
use crate::db::DbConn;
use crate::errors::AppError;

// ── Dictionary Entries ────────────────────────────────────────────────────────

pub fn list_entries(db: &DbConn) -> Result<Vec<DictionaryEntry>, AppError> {
    dictionary_repo::list_entries(db)
}

pub fn create_entry(
    db: &DbConn,
    phrase: &str,
    language: Option<&str>,
    entry_type: &str,
    notes: Option<&str>,
) -> Result<DictionaryEntry, AppError> {
    if phrase.trim().is_empty() {
        return Err(AppError("Phrase must not be empty".into()));
    }
    dictionary_repo::create_entry(db, phrase.trim(), language, entry_type, notes)
}

pub fn update_entry(
    db: &DbConn,
    id: &str,
    phrase: &str,
    language: Option<&str>,
    entry_type: &str,
    notes: Option<&str>,
) -> Result<(), AppError> {
    dictionary_repo::update_entry(db, id, phrase.trim(), language, entry_type, notes)
}

pub fn delete_entry(db: &DbConn, id: &str) -> Result<(), AppError> {
    dictionary_repo::delete_entry(db, id)
}

// ── Correction Rules ──────────────────────────────────────────────────────────

pub fn list_rules(db: &DbConn) -> Result<Vec<CorrectionRule>, AppError> {
    dictionary_repo::list_rules(db)
}

pub fn create_rule(
    db: &DbConn,
    source_phrase: &str,
    target_phrase: &str,
    language: Option<&str>,
    auto_apply: bool,
) -> Result<CorrectionRule, AppError> {
    if source_phrase.trim().is_empty() {
        return Err(AppError("Source phrase must not be empty".into()));
    }
    if target_phrase.trim().is_empty() {
        return Err(AppError("Target phrase must not be empty".into()));
    }
    dictionary_repo::create_rule(
        db,
        source_phrase.trim(),
        target_phrase.trim(),
        language,
        "manual",
        auto_apply,
    )
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
    dictionary_repo::update_rule(
        db, id,
        source_phrase.trim(),
        target_phrase.trim(),
        language,
        is_active,
        auto_apply,
    )
}

pub fn delete_rule(db: &DbConn, id: &str) -> Result<(), AppError> {
    dictionary_repo::delete_rule(db, id)
}

/// Called by the transcription pipeline after rules fire — increments usage counters.
pub fn record_rule_usage(db: &DbConn, fired_ids: &[String]) -> Result<(), AppError> {
    for id in fired_ids {
        dictionary_repo::increment_rule_usage(db, id)?;
    }
    Ok(())
}
