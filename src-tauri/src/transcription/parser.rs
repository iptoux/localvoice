use std::path::Path;

use crate::transcription::types::{TranscriptSegment, WhisperJson};

/// Parse the JSON file written by `whisper-cli --output-json`.
///
/// Returns `None` if the file does not exist (sidecar may not have created it).
pub fn parse_json_file(json_path: &Path) -> Option<Vec<TranscriptSegment>> {
    let content = std::fs::read_to_string(json_path).ok()?;
    let parsed: WhisperJson = serde_json::from_str(&content).ok()?;

    let segments = parsed
        .transcription
        .into_iter()
        .map(|seg| {
            let confidence = seg.tokens.as_deref().map(|tokens| {
                if tokens.is_empty() {
                    0.0f32
                } else {
                    tokens.iter().map(|t| t.p).sum::<f32>() / tokens.len() as f32
                }
            });
            TranscriptSegment {
                start_ms: seg.offsets.from,
                end_ms: seg.offsets.to,
                text: seg.text,
                confidence,
            }
        })
        .collect();

    Some(segments)
}

/// Parse whisper.cpp stdout.
///
/// Each non-empty line looks like:
/// `[00:00:00.000 --> 00:00:02.500]   Some transcribed text.`
pub fn parse_stdout(stdout: &str) -> Vec<TranscriptSegment> {
    stdout
        .lines()
        .filter_map(parse_stdout_line)
        .collect()
}

fn parse_stdout_line(line: &str) -> Option<TranscriptSegment> {
    // Expected prefix: "[HH:MM:SS.mmm --> HH:MM:SS.mmm]"
    let line = line.trim();
    if !line.starts_with('[') {
        return None;
    }
    let close = line.find(']')?;
    let time_part = &line[1..close];
    let text = line[close + 1..].trim().to_string();
    if text.is_empty() {
        return None;
    }

    let mut parts = time_part.splitn(2, " --> ");
    let start_ms = parse_timestamp(parts.next()?.trim())?;
    let end_ms = parse_timestamp(parts.next()?.trim())?;

    Some(TranscriptSegment {
        start_ms,
        end_ms,
        text,
        confidence: None,
    })
}

/// Parse `HH:MM:SS.mmm` or `HH:MM:SS,mmm` → milliseconds.
fn parse_timestamp(s: &str) -> Option<i64> {
    // Normalise SRT comma separator to dot.
    let s = s.replace(',', ".");
    let mut parts = s.splitn(3, ':');
    let h: i64 = parts.next()?.parse().ok()?;
    let m: i64 = parts.next()?.parse().ok()?;
    let rest = parts.next()?;
    let mut sec_parts = rest.splitn(2, '.');
    let s_val: i64 = sec_parts.next()?.parse().ok()?;
    let ms_str = sec_parts.next().unwrap_or("0");
    // Normalise to 3 decimal places.
    let ms_str_padded = format!("{:0<3}", &ms_str[..ms_str.len().min(3)]);
    let ms: i64 = ms_str_padded.parse().ok()?;

    Some(h * 3_600_000 + m * 60_000 + s_val * 1_000 + ms)
}

/// Join segment texts into a single string.
pub fn segments_to_text(segments: &[TranscriptSegment]) -> String {
    segments
        .iter()
        .map(|s| s.text.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
