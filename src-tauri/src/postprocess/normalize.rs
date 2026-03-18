/// Trims leading/trailing whitespace and collapses internal runs of whitespace
/// to a single space.
pub fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
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

/// Full normalisation pipeline applied in order:
/// 1. Collapse whitespace
/// 2. Ensure terminal punctuation (if `auto_punctuation` is enabled)
/// 3. Capitalise sentences (if `auto_capitalization` is enabled)
pub fn normalize(text: &str, auto_capitalization: bool, auto_punctuation: bool) -> String {
    let text = collapse_whitespace(text);
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
