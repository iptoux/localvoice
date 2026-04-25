use std::collections::HashMap;

use crate::db::repositories::dictionary_repo::CorrectionRule;
use crate::dictionary::rules as dict_rules;
use crate::postprocess::{fillers, normalize};
use crate::transcription::types::TranscriptSegment;

/// Runs the full post-processing pipeline on raw transcript data.
///
/// Returns `(cleaned_text, cleaned_segments, fired_rule_ids, removed_fillers)`.
pub fn run(
    raw_text: &str,
    segments: Vec<TranscriptSegment>,
    settings: &HashMap<String, String>,
    active_rules: &[CorrectionRule],
    _language: &str,
    filler_words: &[String],
) -> (String, Vec<TranscriptSegment>, Vec<String>, Vec<String>) {
    let auto_cap = settings
        .get("transcription.auto_capitalization")
        .map(|v| v == "true")
        .unwrap_or(true);

    let auto_punct = settings
        .get("transcription.auto_punctuation")
        .map(|v| v == "true")
        .unwrap_or(true);

    let remove_fillers_enabled = settings
        .get("transcription.remove_fillers")
        .map(|v| v == "true")
        .unwrap_or(false);

    // Clean individual segment texts.
    let cleaned_segments: Vec<TranscriptSegment> = segments
        .into_iter()
        .map(|mut seg| {
            seg.text = normalize::collapse_whitespace(&seg.text);
            seg
        })
        .collect();

    // 1. Filler-word removal (before normalization so punctuation isn't disrupted).
    let (text, removed_fillers) = if remove_fillers_enabled {
        fillers::remove_fillers_tracked(raw_text, filler_words)
    } else {
        (raw_text.to_string(), vec![])
    };

    // 2. Normalize (whitespace + optional punctuation + optional capitalization).
    let normalized = normalize::normalize(&text, auto_cap, auto_punct);

    // 3. Apply correction rules after normalization.
    let (cleaned_text, fired_ids) = dict_rules::apply_rules(&normalized, active_rules);

    (cleaned_text, cleaned_segments, fired_ids, removed_fillers)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// Integration tests for the full post-processing pipeline.
///
/// These tests simulate the path from raw whisper output through normalization,
/// filler removal, and dictionary rule application — matching TASK-237.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;
    use crate::db::repositories::dictionary_repo;
    use rusqlite::Connection;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    fn test_db() -> crate::db::DbConn {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        migrations::run(&conn).unwrap();
        Arc::new(Mutex::new(conn))
    }

    fn seg(text: &str) -> TranscriptSegment {
        TranscriptSegment {
            start_ms: 0,
            end_ms: 1000,
            text: text.to_string(),
            confidence: Some(0.95),
        }
    }

    fn default_settings() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("transcription.auto_capitalization".into(), "true".into());
        m.insert("transcription.auto_punctuation".into(), "true".into());
        m.insert("transcription.remove_fillers".into(), "false".into());
        m
    }

    // ── Baseline pipeline ────────────────────────────────────────────────────

    #[test]
    fn pipeline_normalizes_raw_text() {
        let settings = default_settings();
        let (cleaned, _, _, _) = run(
            "  hello world  ",
            vec![seg("hello world")],
            &settings,
            &[],
            "en",
            &[],
        );
        assert_eq!(cleaned, "Hello world.");
    }

    #[test]
    fn pipeline_no_caps_no_punct() {
        let mut settings = default_settings();
        settings.insert("transcription.auto_capitalization".into(), "false".into());
        settings.insert("transcription.auto_punctuation".into(), "false".into());
        let (cleaned, _, _, _) = run(
            "  hello world  ",
            vec![seg("hello world")],
            &settings,
            &[],
            "en",
            &[],
        );
        assert_eq!(cleaned, "hello world");
    }

    // ── Filler removal ───────────────────────────────────────────────────────

    #[test]
    fn pipeline_removes_fillers_when_enabled() {
        let mut settings = default_settings();
        settings.insert("transcription.remove_fillers".into(), "true".into());
        let filler_words = vec!["uh".to_string(), "um".to_string()];
        let (cleaned, _, _, removed) = run(
            "uh hello um world",
            vec![seg("uh hello um world")],
            &settings,
            &[],
            "en",
            &filler_words,
        );
        let lower = cleaned.to_lowercase();
        assert!(
            !lower.contains(" uh ") && !lower.starts_with("uh "),
            "filler 'uh' should be removed"
        );
        assert!(!lower.contains(" um "), "filler 'um' should be removed");
        assert!(
            lower.contains("hello"),
            "content word 'hello' should remain"
        );
        assert!(
            lower.contains("world"),
            "content word 'world' should remain"
        );
        assert_eq!(removed.len(), 2);
    }

    #[test]
    fn pipeline_keeps_fillers_when_disabled() {
        let settings = default_settings();
        let filler_words = vec!["uh".to_string()];
        let (cleaned, _, _, removed) = run(
            "uh hello world",
            vec![seg("uh hello world")],
            &settings,
            &[],
            "en",
            &filler_words,
        );
        // auto_capitalization is on, so "uh" becomes "Uh" at sentence start
        assert!(
            cleaned.to_lowercase().contains("uh"),
            "fillers should not be removed when disabled"
        );
        assert!(removed.is_empty());
    }

    // ── Dictionary rules ─────────────────────────────────────────────────────

    #[test]
    fn pipeline_applies_correction_rules() {
        let db = test_db();
        let rule =
            dictionary_repo::create_rule(&db, "k8s", "Kubernetes", None, "manual", true).unwrap();
        let active_rules = vec![rule];
        let settings = default_settings();
        let (cleaned, _, fired_ids, _) = run(
            "we use k8s in production",
            vec![seg("we use k8s in production")],
            &settings,
            &active_rules,
            "en",
            &[],
        );
        assert!(
            cleaned.contains("Kubernetes"),
            "rule should replace 'k8s' with 'Kubernetes'"
        );
        assert_eq!(fired_ids.len(), 1, "one rule should fire");
    }

    #[test]
    fn pipeline_records_no_fired_rules_when_no_match() {
        let db = test_db();
        let rule =
            dictionary_repo::create_rule(&db, "k8s", "Kubernetes", None, "manual", true).unwrap();
        let active_rules = vec![rule];
        let settings = default_settings();
        let (cleaned, _, fired_ids, _) = run(
            "nothing to replace here",
            vec![seg("nothing to replace here")],
            &settings,
            &active_rules,
            "en",
            &[],
        );
        // auto_capitalization capitalises first letter, so check case-insensitively
        assert!(cleaned.to_lowercase().contains("nothing to replace here"));
        assert!(fired_ids.is_empty());
    }

    // ── Segment whitespace cleaning ───────────────────────────────────────────

    #[test]
    fn pipeline_collapses_whitespace_in_segments() {
        let settings = default_settings();
        let (_, cleaned_segs, _, _) = run(
            "hello world",
            vec![seg("  hello  world  ")],
            &settings,
            &[],
            "en",
            &[],
        );
        assert_eq!(cleaned_segs[0].text, "hello world");
    }
}
