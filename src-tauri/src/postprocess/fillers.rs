/// Removes filler words (loaded from DB) from the transcription text.
#[allow(dead_code)]
pub fn remove_fillers(text: &str, words: &[String]) -> String {
    let mut sorted: Vec<&String> = words.iter().collect();
    sorted.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut r = text.to_string();
    for filler in &sorted { r = remove_filler_occurrences(&r, filler); }
    r.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Like `remove_fillers` but also returns the list of words that were actually removed.
pub fn remove_fillers_tracked(text: &str, words: &[String]) -> (String, Vec<String>) {
    let mut sorted: Vec<&String> = words.iter().collect();
    sorted.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut result = text.to_string();
    let mut removed: Vec<String> = Vec::new();

    for filler in &sorted {
        let before = result.clone();
        result = remove_filler_occurrences(&result, filler);
        if result != before {
            removed.push(filler.to_string());
        }
    }

    let cleaned = result.split_whitespace().collect::<Vec<_>>().join(" ");
    (cleaned, removed)
}

/// Removes all occurrences of the given filler words/phrases from `text`.
#[allow(dead_code)]
fn remove_with_list(text: &str, fillers: &[String]) -> String {
    let (result, _) = {
        let mut sorted: Vec<&String> = fillers.iter().collect();
        sorted.sort_by(|a, b| b.len().cmp(&a.len()));
        let mut r = text.to_string();
        for filler in &sorted {
            r = remove_filler_occurrences(&r, filler);
        }
        (r, ())
    };
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

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
            let before_ok = i == 0 || !lower_chars[i - 1].is_alphanumeric();
            let end_idx = i + lower_filler.chars().count();
            let after_ok = end_idx >= lower_chars.len() || !lower_chars[end_idx].is_alphanumeric();

            if before_ok && after_ok {
                i = end_idx;
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

    fn en() -> Vec<String> {
        ["uh", "um", "uhm", "hmm", "hm", "mhm", "you know", "like",
         "basically", "actually", "sort of", "kind of", "i mean", "right", "well", "so", "okay so"]
            .iter().map(|s| s.to_string()).collect()
    }

    fn de() -> Vec<String> {
        ["äh", "ähm", "öhm", "hm", "hmm", "mhm", "halt", "sozusagen",
         "quasi", "irgendwie", "also", "ja", "ne", "naja", "gewissermaßen", "sagen wir mal"]
            .iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn removes_english_fillers() {
        let output = remove_fillers("So um I was like basically going to the store", &en());
        assert_eq!(output, "I was going to the store");
    }

    #[test]
    fn removes_german_fillers() {
        let output = remove_fillers("Äh ich wollte halt quasi sagen", &de());
        assert_eq!(output, "ich wollte sagen");
    }

    #[test]
    fn does_not_mangle_words_containing_filler_substrings() {
        let output = remove_fillers("The umbrella was actually um useful", &en());
        assert!(output.contains("umbrella"));
        assert!(output.contains("useful"));
        assert!(!output.contains(" um "));
    }

    #[test]
    fn handles_multi_word_fillers() {
        let output = remove_fillers("I think you know that sort of works", &en());
        assert!(!output.contains("you know"));
        assert!(!output.contains("sort of"));
    }

    #[test]
    fn preserves_empty_text() {
        assert_eq!(remove_fillers("", &en()), "");
    }
}
