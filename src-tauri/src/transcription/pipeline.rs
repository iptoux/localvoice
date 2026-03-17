use std::collections::HashMap;

use crate::db::repositories::dictionary_repo::CorrectionRule;
use crate::dictionary::rules as dict_rules;
use crate::postprocess::normalize;
use crate::transcription::types::TranscriptSegment;

/// Runs the full post-processing pipeline on raw transcript data.
///
/// Returns `(cleaned_text, cleaned_segments, fired_rule_ids)`.
///
/// Stages:
/// 1. Normalise whitespace in all segment texts.
/// 2. Apply `normalize::normalize()` (collapse spaces + optional sentence capitalisation).
/// 3. Apply active correction rules (case-insensitive substring replacement).
pub fn run(
    raw_text: &str,
    segments: Vec<TranscriptSegment>,
    settings: &HashMap<String, String>,
    active_rules: &[CorrectionRule],
) -> (String, Vec<TranscriptSegment>, Vec<String>) {
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

    let normalized = normalize::normalize(raw_text, auto_cap);

    // Apply correction rules after normalization.
    let (cleaned_text, fired_ids) = dict_rules::apply_rules(&normalized, active_rules);

    (cleaned_text, cleaned_segments, fired_ids)
}
