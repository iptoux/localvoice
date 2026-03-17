use std::collections::HashMap;

use crate::postprocess::normalize;
use crate::transcription::types::TranscriptSegment;

/// Runs the full post-processing pipeline on raw transcript data.
///
/// Returns `(cleaned_text, cleaned_segments)`.
///
/// Current stages:
/// 1. Normalise whitespace in all segment texts.
/// 2. Join segments → full raw text.
/// 3. Apply `normalize::normalize()` (collapse spaces + optional sentence capitalisation).
pub fn run(
    raw_text: &str,
    segments: Vec<TranscriptSegment>,
    settings: &HashMap<String, String>,
) -> (String, Vec<TranscriptSegment>) {
    let auto_cap = settings
        .get("transcription.auto_capitalization")
        .map(|v| v == "true")
        .unwrap_or(true);

    // Clean individual segment texts.
    let cleaned_segments: Vec<TranscriptSegment> = segments
        .into_iter()
        .map(|mut seg| {
            seg.text = normalize::collapse_whitespace(&seg.text);
            seg
        })
        .collect();

    let cleaned_text = normalize::normalize(raw_text, auto_cap);

    (cleaned_text, cleaned_segments)
}
