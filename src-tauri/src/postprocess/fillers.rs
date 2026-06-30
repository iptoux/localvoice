/// Removes filler words (loaded from DB) from the transcription text.
#[allow(dead_code)]
pub fn remove_fillers(text: &str, words: &[String]) -> String {
    let mut sorted: Vec<&String> = words.iter().collect();
    sorted.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut r = text.to_string();
    for filler in &sorted {
        r = remove_filler_occurrences(&r, filler);
    }
    r.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Removes filler words while preserving surrounding spacing.
pub(crate) fn remove_fillers_preserving_spacing(text: &str, words: &[String]) -> String {
    let mut sorted: Vec<&String> = words.iter().collect();
    sorted.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut result = text.to_string();
    for filler in &sorted {
        result = remove_filler_occurrences(&result, filler);
    }
    result
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
    let filler_chars: Vec<char> = lower_filler.chars().collect();

    if filler_chars.is_empty() {
        return text.to_string();
    }

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;

    while i < lower_chars.len() {
        if matches_at(&lower_chars, i, &filler_chars) {
            let before_ok = i == 0 || !lower_chars[i - 1].is_alphanumeric();
            let end_idx = i + filler_chars.len();
            let after_ok = end_idx >= lower_chars.len() || !lower_chars[end_idx].is_alphanumeric();

            if before_ok && after_ok {
                let (remove_start, remove_end) =
                    expand_filler_removal_range(&text_chars, i, end_idx);
                push_removal_range(&mut ranges, remove_start, remove_end);
                i = end_idx;
                continue;
            }
        }
        i += 1;
    }

    if ranges.is_empty() {
        return text.to_string();
    }

    remove_ranges(text, &text_chars, &ranges)
}

fn matches_at(chars: &[char], start: usize, needle: &[char]) -> bool {
    start + needle.len() <= chars.len() && chars[start..start + needle.len()] == *needle
}

fn expand_filler_removal_range(
    chars: &[char],
    filler_start: usize,
    filler_end: usize,
) -> (usize, usize) {
    let mut start = filler_start;
    let mut before = filler_start;
    while before > 0 && chars[before - 1].is_whitespace() {
        before -= 1;
    }
    if before > 0 && is_leading_filler_separator(chars[before - 1]) {
        start = before - 1;
    }

    (start, expand_trailing_filler_punctuation(chars, filler_end))
}

fn expand_trailing_filler_punctuation(chars: &[char], filler_end: usize) -> usize {
    let mut end = filler_end;
    let mut scan = filler_end;

    loop {
        while scan < chars.len() && chars[scan].is_whitespace() {
            scan += 1;
        }
        if scan >= chars.len() || !is_trailing_filler_punctuation(chars[scan]) {
            break;
        }

        scan += 1;
        while scan < chars.len() && is_trailing_filler_punctuation(chars[scan]) {
            scan += 1;
        }
        end = scan;
    }

    end
}

fn is_leading_filler_separator(c: char) -> bool {
    matches!(c, ',' | ';' | ':')
}

fn is_trailing_filler_punctuation(c: char) -> bool {
    matches!(c, ',' | '.' | ';' | ':' | '!' | '?')
}

fn push_removal_range(ranges: &mut Vec<(usize, usize)>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    if let Some(last) = ranges.last_mut() {
        if start <= last.1 {
            last.1 = last.1.max(end);
            return;
        }
    }

    ranges.push((start, end));
}

fn remove_ranges(text: &str, chars: &[char], ranges: &[(usize, usize)]) -> String {
    let mut result = String::with_capacity(text.len());
    let mut cursor = 0;

    for &(start, end) in ranges {
        let start = start.max(cursor);
        if start > cursor {
            result.extend(chars[cursor..start].iter());
        }
        if needs_boundary_separator(chars, start, end)
            && !result
                .chars()
                .next_back()
                .map(|c| c.is_whitespace())
                .unwrap_or(false)
        {
            result.push(' ');
        }
        cursor = cursor.max(end);
    }

    if cursor < chars.len() {
        result.extend(chars[cursor..].iter());
    }

    result
}

fn needs_boundary_separator(chars: &[char], start: usize, end: usize) -> bool {
    start > 0
        && end < chars.len()
        && chars[start - 1].is_alphanumeric()
        && chars[end].is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn en() -> Vec<String> {
        [
            "uh",
            "um",
            "uhm",
            "hmm",
            "hm",
            "mhm",
            "you know",
            "like",
            "basically",
            "actually",
            "sort of",
            "kind of",
            "i mean",
            "right",
            "well",
            "so",
            "okay so",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn de() -> Vec<String> {
        [
            "äh",
            "ähm",
            "öhm",
            "hm",
            "hmm",
            "mhm",
            "halt",
            "sozusagen",
            "quasi",
            "irgendwie",
            "also",
            "ja",
            "ne",
            "naja",
            "gewissermaßen",
            "sagen wir mal",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
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
    fn removes_sentence_ending_filler_punctuation() {
        let output = remove_fillers("immer mit der Ruhe Kenn stress, ne. .", &de());
        assert_eq!(output, "immer mit der Ruhe Kenn stress");
    }

    #[test]
    fn removes_inline_filler_punctuation() {
        let output = remove_fillers("Hello, um, world", &en());
        assert_eq!(output, "Hello world");
    }

    #[test]
    fn preserves_empty_text() {
        assert_eq!(remove_fillers("", &en()), "");
    }
}
