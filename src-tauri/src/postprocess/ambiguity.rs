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

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcription::types::TranscriptSegment;

    fn seg(text: &str, conf: Option<f32>) -> TranscriptSegment {
        TranscriptSegment {
            start_ms: 0,
            end_ms: 1000,
            text: text.to_string(),
            confidence: conf,
        }
    }

    #[test]
    fn detects_low_confidence_segment() {
        let segs = vec![seg("hello world", Some(0.3))];
        let candidates = detect(&segs, 0.6);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].phrase, "hello world");
        let conf = candidates[0].confidence.unwrap();
        assert!((conf - 0.3).abs() < 0.001);
    }

    #[test]
    fn skips_high_confidence_segment() {
        let segs = vec![seg("hello world", Some(0.9))];
        let candidates = detect(&segs, 0.6);
        assert!(candidates.is_empty());
    }

    #[test]
    fn skips_segment_at_exact_threshold() {
        let segs = vec![seg("hello world", Some(0.6))];
        let candidates = detect(&segs, 0.6);
        assert!(candidates.is_empty(), "segment at threshold should not be flagged");
    }

    #[test]
    fn skips_segment_without_confidence() {
        let segs = vec![seg("hello world", None)];
        let candidates = detect(&segs, 0.6);
        assert!(candidates.is_empty());
    }

    #[test]
    fn filters_single_char_segment() {
        let segs = vec![seg("a", Some(0.1))];
        let candidates = detect(&segs, 0.6);
        assert!(candidates.is_empty());
    }

    #[test]
    fn filters_non_alphabetic_segment() {
        let segs = vec![seg("123", Some(0.1))];
        let candidates = detect(&segs, 0.6);
        assert!(candidates.is_empty());
    }

    #[test]
    fn normalizes_phrase_to_lowercase() {
        let segs = vec![seg("Hello World", Some(0.3))];
        let candidates = detect(&segs, 0.6);
        assert_eq!(candidates[0].phrase, "hello world");
    }

    #[test]
    fn deduplicates_keeping_lowest_confidence() {
        let segs = vec![
            seg("hello world", Some(0.4)),
            seg("HELLO WORLD", Some(0.2)), // same normalized phrase, lower confidence
        ];
        let candidates = detect(&segs, 0.6);
        assert_eq!(candidates.len(), 1);
        let conf = candidates[0].confidence.unwrap();
        assert!((conf - 0.2).abs() < 0.001, "lowest confidence should be kept");
    }

    #[test]
    fn trims_whitespace_from_phrase() {
        let segs = vec![seg("  hello  ", Some(0.3))];
        let candidates = detect(&segs, 0.6);
        assert_eq!(candidates[0].phrase, "hello");
    }

    #[test]
    fn empty_segments_returns_empty() {
        let candidates = detect(&[], 0.6);
        assert!(candidates.is_empty());
    }
}
