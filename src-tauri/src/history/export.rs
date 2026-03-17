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
