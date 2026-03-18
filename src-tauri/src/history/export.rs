use crate::db::models::Session;
use crate::errors::{AppError, CmdResult};

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
