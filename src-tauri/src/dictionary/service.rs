use crate::db::repositories::ambiguous_terms_repo::{self, AmbiguousTerm};
use crate::db::repositories::dictionary_repo::{self, CorrectionRule, DictionaryEntry};
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

// ── Ambiguity ─────────────────────────────────────────────────────────────────

pub fn list_ambiguous_terms(db: &DbConn, min_occurrences: i64) -> Result<Vec<AmbiguousTerm>, AppError> {
    ambiguous_terms_repo::list_active(db, min_occurrences)
}

/// Accepts a suggestion: creates a "suggested" correction rule for the term,
/// then marks the term as resolved (dismissed).
pub fn accept_ambiguity_suggestion(
    db: &DbConn,
    id: &str,
    target_phrase: &str,
) -> Result<(), AppError> {
    let term = ambiguous_terms_repo::get(db, id)?
        .ok_or_else(|| AppError(format!("Ambiguous term '{}' not found", id)))?;

    dictionary_repo::create_rule(
        db,
        &term.phrase,
        target_phrase.trim(),
        term.language.as_deref(),
        "suggested",
        true,
    )?;

    ambiguous_terms_repo::mark_resolved(db, id)
}

pub fn dismiss_ambiguity_suggestion(db: &DbConn, id: &str) -> Result<(), AppError> {
    ambiguous_terms_repo::dismiss(db, id)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;
    use rusqlite::Connection;
    use std::sync::{Arc, Mutex};

    fn test_db() -> crate::db::DbConn {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        migrations::run(&conn).unwrap();
        Arc::new(Mutex::new(conn))
    }

    // ── Dictionary Entries ────────────────────────────────────────────────────

    #[test]
    fn create_entry_rejects_empty_phrase() {
        let db = test_db();
        let result = create_entry(&db, "  ", None, "term", None);
        assert!(result.is_err(), "empty phrase should be rejected");
    }

    #[test]
    fn create_and_list_entry() {
        let db = test_db();
        create_entry(&db, "Kubernetes", Some("en"), "term", None).unwrap();
        let entries = list_entries(&db).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].phrase, "Kubernetes");
        assert_eq!(entries[0].normalized_phrase, "kubernetes");
        assert_eq!(entries[0].language, Some("en".to_string()));
    }

    #[test]
    fn update_entry_changes_phrase() {
        let db = test_db();
        let entry = create_entry(&db, "k8s", None, "term", None).unwrap();
        update_entry(&db, &entry.id, "Kubernetes", Some("en"), "term", None).unwrap();
        let entries = list_entries(&db).unwrap();
        assert_eq!(entries[0].phrase, "Kubernetes");
    }

    #[test]
    fn delete_entry_removes_it() {
        let db = test_db();
        let entry = create_entry(&db, "Test", None, "term", None).unwrap();
        delete_entry(&db, &entry.id).unwrap();
        assert!(list_entries(&db).unwrap().is_empty());
    }

    // ── Correction Rules ──────────────────────────────────────────────────────

    #[test]
    fn create_rule_rejects_empty_source() {
        let db = test_db();
        let result = create_rule(&db, "  ", "target", None, false);
        assert!(result.is_err(), "empty source phrase should be rejected");
    }

    #[test]
    fn create_rule_rejects_empty_target() {
        let db = test_db();
        let result = create_rule(&db, "source", "  ", None, false);
        assert!(result.is_err(), "empty target phrase should be rejected");
    }

    #[test]
    fn create_and_list_rule() {
        let db = test_db();
        create_rule(&db, "k8s", "Kubernetes", None, true).unwrap();
        let rules = list_rules(&db).unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].source_phrase, "k8s");
        assert_eq!(rules[0].target_phrase, "Kubernetes");
        assert_eq!(rules[0].rule_mode, "manual");
        assert!(rules[0].auto_apply);
    }

    #[test]
    fn delete_rule_removes_it() {
        let db = test_db();
        let rule = create_rule(&db, "k8s", "Kubernetes", None, true).unwrap();
        delete_rule(&db, &rule.id).unwrap();
        assert!(list_rules(&db).unwrap().is_empty());
    }

    #[test]
    fn record_rule_usage_increments_count() {
        let db = test_db();
        let rule = create_rule(&db, "k8s", "Kubernetes", None, true).unwrap();
        assert_eq!(rule.usage_count, 0);
        record_rule_usage(&db, &[rule.id.clone()]).unwrap();
        let rules = list_rules(&db).unwrap();
        assert_eq!(rules[0].usage_count, 1);
    }

    #[test]
    fn record_rule_usage_multiple_calls_accumulate() {
        let db = test_db();
        let rule = create_rule(&db, "k8s", "Kubernetes", None, true).unwrap();
        record_rule_usage(&db, &[rule.id.clone()]).unwrap();
        record_rule_usage(&db, &[rule.id.clone()]).unwrap();
        record_rule_usage(&db, &[rule.id.clone()]).unwrap();
        let rules = list_rules(&db).unwrap();
        assert_eq!(rules[0].usage_count, 3);
    }
}
