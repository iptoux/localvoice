/// Trims leading/trailing whitespace and collapses internal runs of whitespace
/// to a single space.
pub fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Removes ASR language markers such as `<de-DE>`, `<DE>`, or `<en-US>`.
///
/// Some Parakeet/Nemotron streaming models emit the language prompt token as
/// plain transcript text. These markers are metadata, not dictated content.
pub fn remove_language_tags(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0;

    while let Some(relative_start) = text[cursor..].find('<') {
        let start = cursor + relative_start;
        out.push_str(&text[cursor..start]);

        if let Some(relative_end) = text[start..].find('>') {
            let end = start + relative_end;
            let candidate = text[start + 1..end].trim();
            if is_language_tag(candidate) {
                cursor = end + 1;
                continue;
            }
        }

        out.push('<');
        cursor = start + '<'.len_utf8();
    }

    out.push_str(&text[cursor..]);
    out
}

fn is_language_tag(candidate: &str) -> bool {
    if candidate.is_empty() {
        return false;
    }

    let parts = candidate.split(['-', '_']).collect::<Vec<_>>();
    if !(1..=2).contains(&parts.len()) {
        return false;
    }

    let language = parts[0];
    if !(2..=3).contains(&language.len()) || !language.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }

    if let Some(region) = parts.get(1) {
        if !(2..=4).contains(&region.len()) || !region.chars().all(|c| c.is_ascii_alphabetic()) {
            return false;
        }
    }

    true
}

/// Ensures the first alphabetic character of the string is uppercase.
#[allow(dead_code)]
pub fn capitalize_first(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let upper: String = first.to_uppercase().collect();
            upper + chars.as_str()
        }
    }
}

/// Capitalises the first letter of each sentence.
///
/// A sentence boundary is detected after `.`, `!`, or `?` followed by whitespace.
pub fn capitalize_sentences(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(text.len());
    let mut capitalize_next = true;

    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if capitalize_next && c.is_alphabetic() {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
            if matches!(c, '.' | '!' | '?') {
                // Capitalise after punctuation + whitespace.
                if chars.peek().map(|n| n.is_whitespace()).unwrap_or(false) {
                    capitalize_next = true;
                }
            }
        }
    }

    result
}

/// Ensures the text ends with a terminal punctuation mark (`.`, `!`, or `?`).
///
/// If the trimmed text already ends with punctuation, it is returned unchanged.
/// Otherwise, a period is appended.
pub fn ensure_terminal_punctuation(text: &str) -> String {
    let trimmed = text.trim_end();
    if trimmed.is_empty() {
        return trimmed.to_string();
    }
    if trimmed.ends_with(|c: char| matches!(c, '.' | '!' | '?' | '…' | ':' | ';')) {
        trimmed.to_string()
    } else {
        format!("{trimmed}.")
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── collapse_whitespace ───────────────────────────────────────────────────

    #[test]
    fn collapse_whitespace_trims_leading_trailing() {
        assert_eq!(collapse_whitespace("  hello  "), "hello");
    }

    #[test]
    fn collapse_whitespace_collapses_internal_runs() {
        assert_eq!(collapse_whitespace("hello   world"), "hello world");
    }

    #[test]
    fn collapse_whitespace_handles_tabs_and_newlines() {
        assert_eq!(collapse_whitespace("hello\t\nworld"), "hello world");
    }

    #[test]
    fn collapse_whitespace_empty_string() {
        assert_eq!(collapse_whitespace(""), "");
    }

    #[test]
    fn remove_language_tags_removes_locale_markers() {
        assert_eq!(
            remove_language_tags("Das ist ein Test. <de-DE> Hier geht es weiter."),
            "Das ist ein Test.  Hier geht es weiter."
        );
        assert_eq!(remove_language_tags("<DE> Hallo"), " Hallo");
        assert_eq!(remove_language_tags("Hello <en-US> world"), "Hello  world");
    }

    #[test]
    fn remove_language_tags_keeps_non_locale_angle_text() {
        assert_eq!(
            remove_language_tags("Use <placeholder> here"),
            "Use <placeholder> here"
        );
        assert_eq!(remove_language_tags("Score <3"), "Score <3");
    }

    // ── capitalize_first ─────────────────────────────────────────────────────

    #[test]
    fn capitalize_first_lowercased_word() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn capitalize_first_already_uppercase() {
        assert_eq!(capitalize_first("Hello"), "Hello");
    }

    #[test]
    fn capitalize_first_empty_string() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn capitalize_first_single_char() {
        assert_eq!(capitalize_first("a"), "A");
    }

    // ── capitalize_sentences ─────────────────────────────────────────────────

    #[test]
    fn capitalize_sentences_first_word() {
        assert_eq!(capitalize_sentences("hello world"), "Hello world");
    }

    #[test]
    fn capitalize_sentences_after_period_space() {
        // capitalize_sentences also capitalises the first letter of the string,
        // so both "hello" and "world" get capitalised here.
        assert_eq!(capitalize_sentences("hello. world"), "Hello. World");
    }

    #[test]
    fn capitalize_sentences_after_exclamation_space() {
        assert_eq!(capitalize_sentences("stop! go"), "Stop! Go");
    }

    #[test]
    fn capitalize_sentences_after_question_space() {
        assert_eq!(capitalize_sentences("done? yes"), "Done? Yes");
    }

    #[test]
    fn capitalize_sentences_multiple_boundaries() {
        let input = "first. second! third? fourth";
        let expected = "First. Second! Third? Fourth";
        assert_eq!(capitalize_sentences(input), expected);
    }

    #[test]
    fn capitalize_sentences_no_capitalize_after_period_without_space() {
        // "e.g." should not trigger capitalization of next letter
        assert_eq!(capitalize_sentences("e.g.hello"), "E.g.hello");
    }

    #[test]
    fn capitalize_sentences_empty_string() {
        assert_eq!(capitalize_sentences(""), "");
    }

    // ── ensure_terminal_punctuation ──────────────────────────────────────────

    #[test]
    fn ensure_terminal_punctuation_adds_period_when_missing() {
        assert_eq!(ensure_terminal_punctuation("hello world"), "hello world.");
    }

    #[test]
    fn ensure_terminal_punctuation_preserves_period() {
        assert_eq!(ensure_terminal_punctuation("hello."), "hello.");
    }

    #[test]
    fn ensure_terminal_punctuation_preserves_exclamation() {
        assert_eq!(ensure_terminal_punctuation("hello!"), "hello!");
    }

    #[test]
    fn ensure_terminal_punctuation_preserves_question() {
        assert_eq!(ensure_terminal_punctuation("hello?"), "hello?");
    }

    #[test]
    fn ensure_terminal_punctuation_trims_trailing_whitespace() {
        assert_eq!(ensure_terminal_punctuation("hello   "), "hello.");
    }

    #[test]
    fn ensure_terminal_punctuation_empty_string() {
        assert_eq!(ensure_terminal_punctuation(""), "");
    }

    // ── normalize (full pipeline) ─────────────────────────────────────────────

    #[test]
    fn normalize_full_pipeline_caps_and_punct() {
        let result = normalize("  hello world  ", true, true);
        assert_eq!(result, "Hello world.");
    }

    #[test]
    fn normalize_no_caps_no_punct() {
        let result = normalize("  hello world  ", false, false);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn normalize_caps_only() {
        let result = normalize("hello world", true, false);
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn normalize_punct_only() {
        let result = normalize("hello world", false, true);
        assert_eq!(result, "hello world.");
    }
}

/// Full normalisation pipeline applied in order:
/// 1. Collapse whitespace
/// 2. Ensure terminal punctuation (if `auto_punctuation` is enabled)
/// 3. Capitalise sentences (if `auto_capitalization` is enabled)
pub fn normalize(text: &str, auto_capitalization: bool, auto_punctuation: bool) -> String {
    let text = remove_language_tags(text);
    let text = collapse_whitespace(&text);
    let text = if auto_punctuation {
        ensure_terminal_punctuation(&text)
    } else {
        text
    };
    if auto_capitalization {
        capitalize_sentences(&text)
    } else {
        text
    }
}
