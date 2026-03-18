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
