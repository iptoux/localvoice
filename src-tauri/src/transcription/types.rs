use serde::{Deserialize, Serialize};

/// A single time-stamped transcript segment from whisper.cpp.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegment {
    /// Start of segment in milliseconds.
    pub start_ms: i64,
    /// End of segment in milliseconds.
    pub end_ms: i64,
    /// Transcribed text (may include leading whitespace from whisper).
    pub text: String,
    /// Mean token probability [0, 1] — available when parsing JSON output.
    pub confidence: Option<f32>,
}

/// Outcome of the output step (clipboard write or auto-insert).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputResult {
    /// Effective output mode used: "clipboard" or "insert".
    pub mode: String,
    /// Whether the output step completed successfully.
    pub success: bool,
    /// Error description when `success` is false.
    pub error: Option<String>,
}

/// Full result returned after a successful transcription.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionResult {
    /// Raw joined text as whisper produced it (before post-processing).
    pub raw_text: String,
    /// Cleaned text after normalization / post-processing pipeline.
    pub cleaned_text: String,
    /// Time-stamped segments.
    pub segments: Vec<TranscriptSegment>,
    /// ISO 639-1 language code actually used (e.g. "de", "en", "auto").
    pub language: String,
    /// Stem of the model filename used (e.g. "ggml-base").
    pub model_id: String,
    /// Wall-clock transcription time in milliseconds.
    pub duration_ms: u64,
    /// Result of the output step (set by the orchestrator after transcription).
    pub output: Option<OutputResult>,
    /// Filler words that were removed during post-processing (for stats tracking).
    #[serde(default)]
    pub removed_fillers: Vec<String>,
}

// ── Whisper JSON deserialization helpers ──────────────────────────────────────

/// Root of the JSON file written by `whisper-cli --output-json`.
#[derive(Deserialize)]
pub(super) struct WhisperJson {
    pub transcription: Vec<WhisperJsonSegment>,
}

#[derive(Deserialize)]
pub(super) struct WhisperJsonSegment {
    pub offsets: WhisperOffsets,
    pub text: String,
    pub tokens: Option<Vec<WhisperToken>>,
}

#[derive(Deserialize)]
pub(super) struct WhisperOffsets {
    pub from: i64,
    pub to: i64,
}

#[derive(Deserialize)]
pub(super) struct WhisperToken {
    pub p: f32,
}
