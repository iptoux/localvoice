use serde::{Deserialize, Serialize};

/// A persisted dictation session row from the `sessions` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub started_at: String,
    pub ended_at: String,
    pub duration_ms: i64,
    pub language: String,
    pub model_id: Option<String>,
    pub trigger_type: String,
    pub input_device_id: Option<String>,
    pub raw_text: String,
    pub cleaned_text: String,
    pub word_count: i64,
    pub char_count: i64,
    pub avg_confidence: Option<f64>,
    pub estimated_wpm: Option<f64>,
    pub output_mode: String,
    pub output_target_app: Option<String>,
    pub inserted_successfully: bool,
    pub error_message: Option<String>,
    pub created_at: String,
    pub audio_path: Option<String>,
    pub original_raw_text: Option<String>,
    pub reprocessed_count: i64,
    pub original_model_id: Option<String>,
    pub original_language: Option<String>,
    pub original_avg_confidence: Option<f64>,
}

/// A single time-stamped segment row from `session_segments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionSegment {
    pub id: String,
    pub session_id: String,
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
    pub confidence: Option<f64>,
    pub segment_index: i64,
}

/// A session with its associated segments, returned by `get_session`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionWithSegments {
    pub session: Session,
    pub segments: Vec<SessionSegment>,
}

/// Filter / pagination parameters for `list_sessions`.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SessionFilter {
    /// Full-text search across `cleaned_text` and `raw_text`.
    pub query: Option<String>,
    /// ISO 639-1 language code (e.g. "de", "en").
    pub language: Option<String>,
    /// ISO 8601 lower bound for `started_at`.
    pub date_from: Option<String>,
    /// ISO 8601 upper bound for `started_at`.
    pub date_to: Option<String>,
    /// Filter by model stem (e.g. "ggml-base").
    pub model_id: Option<String>,
    /// When true, only return sessions that have an audio file.
    pub has_audio: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
