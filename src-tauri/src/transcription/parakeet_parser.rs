use serde::Deserialize;

use crate::postprocess::normalize;
use crate::transcription::types::{TranscriptSegment, TranscriptWord};

#[derive(Debug, Clone)]
pub struct ParakeetTranscript {
    pub text: String,
    pub segments: Vec<TranscriptSegment>,
    pub words: Vec<TranscriptWord>,
}

#[derive(Debug, Deserialize)]
struct ParakeetJson {
    text: Option<String>,
    segments: Option<Vec<ParakeetSegment>>,
    words: Option<Vec<ParakeetWord>>,
}

#[derive(Debug, Deserialize)]
struct ParakeetSegment {
    start: Option<f64>,
    end: Option<f64>,
    text: Option<String>,
    confidence: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct ParakeetWord {
    #[serde(alias = "word", alias = "w")]
    text: Option<String>,
    start: Option<f64>,
    end: Option<f64>,
    #[serde(alias = "conf")]
    confidence: Option<f32>,
    #[serde(alias = "probability")]
    prob: Option<f32>,
}

pub fn parse_stdout(stdout: &str) -> Option<ParakeetTranscript> {
    let parsed = parse_json(stdout).or_else(|| parse_last_json_line(stdout))?;
    Some(parsed)
}

fn parse_json(raw: &str) -> Option<ParakeetTranscript> {
    let parsed: ParakeetJson = serde_json::from_str(raw).ok()?;
    Some(parsed.into_transcript())
}

fn parse_last_json_line(raw: &str) -> Option<ParakeetTranscript> {
    raw.lines()
        .rev()
        .map(str::trim)
        .find(|line| line.starts_with('{') && line.ends_with('}'))
        .and_then(parse_json)
}

impl ParakeetJson {
    fn into_transcript(self) -> ParakeetTranscript {
        let words: Vec<TranscriptWord> = self
            .words
            .unwrap_or_default()
            .into_iter()
            .filter_map(|word| {
                let text = normalize::remove_language_tags(&word.text?);
                if text.trim().is_empty() {
                    return None;
                }
                Some(TranscriptWord {
                    start_ms: seconds_to_ms(word.start.unwrap_or(0.0)),
                    end_ms: seconds_to_ms(word.end.unwrap_or(0.0)),
                    text: text.trim().to_string(),
                    confidence: word.confidence.or(word.prob),
                })
            })
            .collect();

        let text = self
            .text
            .map(|text| normalize::remove_language_tags(&text))
            .unwrap_or_else(|| {
                words
                    .iter()
                    .map(|w| w.text.trim())
                    .filter(|w| !w.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            });

        let mut segments: Vec<TranscriptSegment> = self
            .segments
            .unwrap_or_default()
            .into_iter()
            .filter_map(|seg| {
                let text = normalize::remove_language_tags(&seg.text?);
                if text.trim().is_empty() {
                    return None;
                }
                Some(TranscriptSegment {
                    start_ms: seconds_to_ms(seg.start.unwrap_or(0.0)),
                    end_ms: seconds_to_ms(seg.end.unwrap_or(0.0)),
                    text: text.trim().to_string(),
                    confidence: seg.confidence,
                })
            })
            .collect();

        if segments.is_empty() && !text.trim().is_empty() {
            let start_ms = words.first().map(|w| w.start_ms).unwrap_or(0);
            let end_ms = words.last().map(|w| w.end_ms).unwrap_or(0);
            let confidence = mean_word_confidence(&words);
            segments.push(TranscriptSegment {
                start_ms,
                end_ms,
                text: text.clone(),
                confidence,
            });
        }

        ParakeetTranscript {
            text,
            segments,
            words,
        }
    }
}

fn mean_word_confidence(words: &[TranscriptWord]) -> Option<f32> {
    let values: Vec<f32> = words.iter().filter_map(|w| w.confidence).collect();
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f32>() / values.len() as f32)
    }
}

fn seconds_to_ms(value: f64) -> i64 {
    (value * 1000.0).round() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_parakeet_verbose_json() {
        let raw = r#"{
            "text": "Hallo Welt.",
            "words": [
                {"word": "Hallo", "start": 0.08, "end": 0.32, "confidence": 0.91},
                {"word": "Welt.", "start": 0.32, "end": 0.64, "confidence": 0.88}
            ]
        }"#;

        let parsed = parse_stdout(raw).expect("json should parse");
        assert_eq!(parsed.text, "Hallo Welt.");
        assert_eq!(parsed.words.len(), 2);
        assert_eq!(parsed.segments.len(), 1);
        assert_eq!(parsed.words[0].start_ms, 80);
    }

    #[test]
    fn parses_last_json_line_after_logs() {
        let raw = "loading model\n{\"text\":\"ok\",\"words\":[]}\n";
        let parsed = parse_stdout(raw).expect("json line should parse");
        assert_eq!(parsed.text, "ok");
    }

    #[test]
    fn removes_language_tags_from_parakeet_text() {
        let raw = r#"{
            "text": "Das ist ein Test. <de-DE> Hier geht es weiter.",
            "words": [
                {"word": "Das", "start": 0.0, "end": 0.1},
                {"word": "<de-DE>", "start": 0.1, "end": 0.2},
                {"word": "Hier", "start": 0.2, "end": 0.3}
            ],
            "segments": [
                {"text": "<de-DE> Hier geht es weiter.", "start": 0.2, "end": 1.0}
            ]
        }"#;

        let parsed = parse_stdout(raw).expect("json should parse");

        assert_eq!(parsed.text, "Das ist ein Test.  Hier geht es weiter.");
        assert_eq!(parsed.words.len(), 2);
        assert_eq!(parsed.segments[0].text, "Hier geht es weiter.");
    }
}
