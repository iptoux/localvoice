/// Removes common filler words from the transcription text.
///
/// Uses word-boundary matching to avoid mangling legitimate words that contain
/// filler substrings (e.g. "umbrella" must not be affected by removing "um").

/// German filler words and phrases.
const FILLERS_DE: &[&str] = &[
    "äh", "ähm", "öhm", "hm", "hmm", "mhm",
    "halt", "sozusagen", "quasi", "irgendwie",
    "also", "ja", "ne", "naja", "naja",
    "gewissermaßen", "sagen wir mal",
];

/// English filler words and phrases.
const FILLERS_EN: &[&str] = &[
    "uh", "um", "uhm", "hmm", "hm", "mhm",
    "you know", "like", "basically", "actually",
    "sort of", "kind of", "i mean", "right",
    "well", "so", "okay so",
];

/// Removes filler words for the given language from `text`.
///
/// Multi-word fillers (e.g. "you know") are matched first so they are removed
/// as a unit. Single-word fillers use word-boundary semantics.
pub fn remove_fillers(text: &str, language: &str) -> String {
    let fillers: &[&str] = match language {
        "de" => FILLERS_DE,
        "en" => FILLERS_EN,
        _ => {
            // For unknown languages, apply both lists as a best-effort fallback.
            let mut result = remove_with_list(text, FILLERS_DE);
            result = remove_with_list(&result, FILLERS_EN);
            return result;
        }
    };
    remove_with_list(text, fillers)
}

/// Removes all occurrences of the given filler words/phrases from `text`.
///
/// - Multi-word fillers are removed via case-insensitive substring replacement
///   with word-boundary checks.
/// - Single-word fillers are removed only at word boundaries.
/// - After removal, multiple consecutive spaces are collapsed to one.
fn remove_with_list(text: &str, fillers: &[&str]) -> String {
    let mut result = text.to_string();

    // Process multi-word fillers first (longer matches first to avoid partial removal).
    let mut sorted: Vec<&&str> = fillers.iter().collect();
    sorted.sort_by(|a, b| b.len().cmp(&a.len()));

    for filler in sorted {
        result = remove_filler_occurrences(&result, filler);
    }

    // Collapse whitespace.
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Removes all case-insensitive occurrences of `filler` from `text`,
/// only when bounded by word boundaries (start/end of string or non-alphanumeric chars).
fn remove_filler_occurrences(text: &str, filler: &str) -> String {
    if filler.is_empty() {
        return text.to_string();
    }

    let lower_text = text.to_lowercase();
    let lower_filler = filler.to_lowercase();
    let text_chars: Vec<char> = text.chars().collect();
    let lower_chars: Vec<char> = lower_text.chars().collect();

    let mut result = String::with_capacity(text.len());
    let mut i = 0;

    while i < lower_chars.len() {
        let remaining: String = lower_chars[i..].iter().collect();
        if remaining.starts_with(&lower_filler) {
            // Check word boundary before the match.
            let before_ok = if i == 0 {
                true
            } else {
                !lower_chars[i - 1].is_alphanumeric()
            };

            // Check word boundary after the match.
            let end_idx = i + lower_filler.chars().count();
            let after_ok = if end_idx >= lower_chars.len() {
                true
            } else {
                !lower_chars[end_idx].is_alphanumeric()
            };

            if before_ok && after_ok {
                // Skip the filler — advance past it.
                i = end_idx;
                // Add a space to prevent words on both sides from merging.
                if !result.ends_with(' ') && i < lower_chars.len() {
                    result.push(' ');
                }
                continue;
            }
        }

        result.push(text_chars[i]);
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_english_fillers() {
        let input = "So um I was like basically going to the store";
        let output = remove_fillers(input, "en");
        assert_eq!(output, "I was going to the store");
    }

    #[test]
    fn removes_german_fillers() {
        let input = "Äh ich wollte halt quasi sagen";
        let output = remove_fillers(input, "de");
        assert_eq!(output, "ich wollte sagen");
    }

    #[test]
    fn does_not_mangle_words_containing_filler_substrings() {
        let input = "The umbrella was actually um useful";
        let output = remove_fillers(input, "en");
        assert!(output.contains("umbrella"), "umbrella should be preserved: {output}");
        assert!(output.contains("useful"), "useful should be preserved: {output}");
        assert!(!output.contains(" um "), "standalone 'um' should be removed: {output}");
    }

    #[test]
    fn handles_multi_word_fillers() {
        let input = "I think you know that sort of works";
        let output = remove_fillers(input, "en");
        assert!(!output.contains("you know"), "multi-word filler should be removed: {output}");
        assert!(!output.contains("sort of"), "multi-word filler should be removed: {output}");
    }

    #[test]
    fn preserves_empty_text() {
        assert_eq!(remove_fillers("", "en"), "");
    }
}
