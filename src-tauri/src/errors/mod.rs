use serde::Serialize;

/// Unified error type returned from all Tauri commands.
/// Serializes to a plain string so the frontend receives a descriptive message.
#[derive(Debug, Serialize)]
pub struct AppError(pub String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError(e.to_string())
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError(s.to_string())
    }
}

pub type CmdResult<T> = Result<T, AppError>;

/// Maps internal error strings to short, user-friendly messages suitable for
/// display in notifications and the pill UI.
pub fn user_friendly_message(raw: &str) -> String {
    let lower = raw.to_lowercase();
    if lower.contains("no model") || (lower.contains("model") && lower.contains("not found")) {
        return "No model installed. Open Models to download one.".into();
    }
    if lower.contains("whisper-cli")
        || lower.contains("whisper_sidecar")
        || lower.contains("whisper sidecar")
    {
        return "Transcription failed. Make sure a model is installed and working.".into();
    }
    if lower.contains("audio")
        || lower.contains("microphone")
        || lower.contains("input device")
        || lower.contains("cpal")
    {
        return "Microphone not accessible. Check your audio settings.".into();
    }
    if lower.contains("disk") || lower.contains("no space") || lower.contains("storage") {
        return "Not enough disk space to record audio.".into();
    }
    // Generic fallback — truncate very long messages.
    if raw.len() > 120 {
        format!("{}…", &raw[..120])
    } else {
        raw.to_string()
    }
}
