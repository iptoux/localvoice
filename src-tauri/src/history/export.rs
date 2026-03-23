use crate::db::models::Session;
use crate::errors::{AppError, CmdResult};

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_session(id: &str, lang: &str, text: &str, wpm: Option<f64>) -> Session {
        Session {
            id: id.to_string(),
            started_at: "2026-01-01T10:00:00Z".to_string(),
            ended_at: "2026-01-01T10:01:00Z".to_string(),
            duration_ms: 60_000,
            language: lang.to_string(),
            model_id: Some("ggml-base".to_string()),
            trigger_type: "hotkey".to_string(),
            input_device_id: None,
            raw_text: text.to_string(),
            cleaned_text: text.to_string(),
            word_count: text.split_whitespace().count() as i64,
            char_count: text.len() as i64,
            avg_confidence: Some(0.9),
            estimated_wpm: wpm,
            output_mode: "clipboard".to_string(),
            output_target_app: None,
            inserted_successfully: true,
            error_message: None,
            created_at: "2026-01-01T10:00:00Z".to_string(),
            audio_path: None,
            original_raw_text: None,
            reprocessed_count: 0,
            original_model_id: None,
            original_language: None,
            original_avg_confidence: None,
        }
    }

    // ── to_text ───────────────────────────────────────────────────────────────

    #[test]
    fn to_text_contains_session_text() {
        let sessions = vec![make_session("1", "de", "Hello world", Some(100.0))];
        let output = to_text(&sessions);
        assert!(output.contains("Hello world"));
    }

    #[test]
    fn to_text_contains_language_and_wpm() {
        let sessions = vec![make_session("1", "de", "Hello world", Some(100.0))];
        let output = to_text(&sessions);
        assert!(output.contains("de"));
        assert!(output.contains("~100 wpm"));
    }

    #[test]
    fn to_text_no_wpm_when_missing() {
        let sessions = vec![make_session("1", "en", "Test", None)];
        let output = to_text(&sessions);
        assert!(!output.contains("wpm"));
    }

    #[test]
    fn to_text_empty_sessions_returns_empty_string() {
        assert_eq!(to_text(&[]), "");
    }

    #[test]
    fn to_text_multiple_sessions_separated() {
        let sessions = vec![
            make_session("1", "de", "First session", None),
            make_session("2", "en", "Second session", None),
        ];
        let output = to_text(&sessions);
        assert!(output.contains("First session"));
        assert!(output.contains("Second session"));
    }

    // ── to_json ───────────────────────────────────────────────────────────────

    #[test]
    fn to_json_produces_valid_json_array() {
        let sessions = vec![make_session("1", "en", "Test text", None)];
        let json = to_json(&sessions).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 1);
    }

    #[test]
    fn to_json_empty_sessions_produces_empty_array() {
        let json = to_json(&[]).unwrap();
        assert_eq!(json.trim(), "[]");
    }

    #[test]
    fn to_json_contains_session_fields() {
        let sessions = vec![make_session("abc123", "en", "Test", None)];
        let json = to_json(&sessions).unwrap();
        assert!(json.contains("abc123"));
        assert!(json.contains("ggml-base"));
    }

    // ── to_csv ────────────────────────────────────────────────────────────────

    #[test]
    fn to_csv_has_correct_header() {
        let csv = to_csv(&[]);
        let first_line = csv.lines().next().unwrap();
        assert_eq!(
            first_line,
            "date,language,model,duration_s,word_count,wpm,raw_text,cleaned_text"
        );
    }

    #[test]
    fn to_csv_empty_sessions_has_only_header() {
        let csv = to_csv(&[]);
        let line_count = csv.trim_end_matches('\n').lines().count();
        assert_eq!(line_count, 1);
    }

    #[test]
    fn to_csv_data_row_contains_session_fields() {
        let sessions = vec![make_session("1", "en", "Test text", Some(90.0))];
        let csv = to_csv(&sessions);
        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("ggml-base"));
        assert!(lines[1].contains("en"));
        assert!(lines[1].contains("90.0"));
        // duration_s = 60_000 / 1000 = 60
        assert!(lines[1].contains(",60,"));
    }

    #[test]
    fn to_csv_escapes_double_quotes_in_text() {
        let sessions = vec![make_session("1", "en", r#"say "hello""#, None)];
        let csv = to_csv(&sessions);
        // Each inner " is doubled to "" per RFC 4180 (the escape function).
        // Input: say "hello"  →  CSV field content: "say ""hello"""
        // The subsequence ""hello"" must appear inside the quoted field.
        assert!(csv.contains("\"\"hello\"\""));
    }

    #[test]
    fn to_csv_duration_is_in_seconds() {
        let sessions = vec![make_session("1", "en", "test", None)];
        let csv = to_csv(&sessions);
        // duration_ms=60_000 → duration_s=60
        let data_line = csv.lines().nth(1).unwrap();
        let fields: Vec<&str> = data_line.split(',').collect();
        // date,language,model,duration_s → index 3
        assert_eq!(fields[3], "60");
    }
}

/// Renders sessions as human-readable plain text, one block per session.
pub fn to_text(sessions: &[Session]) -> String {
    sessions
        .iter()
        .map(|s| {
            let wpm = s
                .estimated_wpm
                .map(|w| format!(" | ~{:.0} wpm", w))
                .unwrap_or_default();
            format!(
                "── {} ──────────────────────────────\n\
                 Language: {}  |  Words: {}{}  |  Output: {}\n\
                 {}\n",
                s.started_at, s.language, s.word_count, wpm, s.output_mode, s.cleaned_text
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Renders sessions as a pretty-printed JSON array.
pub fn to_json(sessions: &[Session]) -> CmdResult<String> {
    serde_json::to_string_pretty(sessions)
        .map_err(|e| AppError(format!("JSON serialisation failed: {e}")))
}

/// Renders sessions as CSV with a header row.
/// Columns: date, language, model, duration_s, word_count, wpm, raw_text, cleaned_text
pub fn to_csv(sessions: &[Session]) -> String {
    fn escape(s: &str) -> String {
        // Wrap in quotes and escape inner quotes by doubling them.
        format!("\"{}\"", s.replace('"', "\"\""))
    }

    let mut out = String::from("date,language,model,duration_s,word_count,wpm,raw_text,cleaned_text\n");
    for s in sessions {
        let model = s.model_id.as_deref().unwrap_or("");
        let duration_s = s.duration_ms / 1000;
        let wpm = s
            .estimated_wpm
            .map(|w| format!("{:.1}", w))
            .unwrap_or_default();
        out.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            escape(&s.started_at),
            escape(&s.language),
            escape(model),
            duration_s,
            s.word_count,
            wpm,
            escape(&s.raw_text),
            escape(&s.cleaned_text),
        ));
    }
    out
}
