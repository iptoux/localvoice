use crate::transcription::types::TranscriptSegment;

/// A candidate phrase extracted from a low-confidence transcript segment.
#[derive(Debug, Clone)]
pub struct AmbiguousCandidate {
    /// Normalised (lowercased, trimmed) phrase — used as the DB lookup key.
    pub phrase: String,
    /// Mean confidence of the segment this phrase came from, if available.
    pub confidence: Option<f32>,
}

/// Analyses `segments` and returns candidates whose confidence falls below
/// `threshold`. Only segments that provide a confidence score are considered;
/// segments without confidence data are skipped.
///
/// Each qualifying segment contributes its trimmed text as a single candidate.
/// Segments shorter than 2 characters or containing no alphabetic content are
/// excluded. Duplicates (same normalised phrase) are merged — keeping the
/// lowest confidence value seen.
pub fn detect(segments: &[TranscriptSegment], threshold: f32) -> Vec<AmbiguousCandidate> {
    use std::collections::HashMap;

    let mut map: HashMap<String, Option<f32>> = HashMap::new();

    for seg in segments {
        let conf = match seg.confidence {
            Some(c) => c,
            None => continue, // no confidence data — skip
        };
        if conf >= threshold {
            continue; // confident enough — not ambiguous
        }

        let phrase = seg.text.trim().to_lowercase();
        // Filter out trivially short or non-alphabetic segments.
        if phrase.len() < 2 || !phrase.chars().any(|c| c.is_alphabetic()) {
            continue;
        }

        let entry = map.entry(phrase).or_insert(None);
        match (*entry, Some(conf)) {
            (None, _) => *entry = Some(conf),
            (Some(existing), Some(new)) if new < existing => *entry = Some(new),
            _ => {}
        }
    }

    map.into_iter()
        .map(|(phrase, confidence)| AmbiguousCandidate { phrase, confidence })
        .collect()
}
