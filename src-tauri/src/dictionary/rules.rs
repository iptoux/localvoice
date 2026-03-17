use crate::db::repositories::dictionary_repo::CorrectionRule;

/// Applies all active correction rules to `text`, returning the corrected string.
///
/// Rules are applied in the order provided (callers should sort by `usage_count DESC`
/// so most-proven rules run first). Each rule performs a case-insensitive global
/// substring replacement — the target phrase is used verbatim.
///
/// Returns the corrected text and a list of rule IDs that fired (for usage tracking).
pub fn apply_rules(text: &str, rules: &[CorrectionRule]) -> (String, Vec<String>) {
    let mut result = text.to_string();
    let mut fired_ids: Vec<String> = Vec::new();

    for rule in rules {
        let replaced = replace_case_insensitive(&result, &rule.source_phrase, &rule.target_phrase);
        if replaced != result {
            fired_ids.push(rule.id.clone());
            result = replaced;
        }
    }

    (result, fired_ids)
}

/// Case-insensitive global replacement of `from` with `to` inside `text`.
///
/// Matching is done on UTF-8 character boundaries using lowercased comparison.
/// The rest of the text (non-matching segments) is preserved byte-for-byte.
fn replace_case_insensitive(text: &str, from: &str, to: &str) -> String {
    if from.is_empty() {
        return text.to_string();
    }

    let text_lower = text.to_lowercase();
    let from_lower = from.to_lowercase();
    let from_len = from.len(); // byte length of the original (UTF-8 safe via lowercase mirror)

    let mut result = String::with_capacity(text.len());
    let mut search_start = 0usize;

    while search_start <= text.len() {
        match text_lower[search_start..].find(&from_lower) {
            None => {
                result.push_str(&text[search_start..]);
                break;
            }
            Some(rel_pos) => {
                let abs_pos = search_start + rel_pos;
                result.push_str(&text[search_start..abs_pos]);
                result.push_str(to);
                search_start = abs_pos + from_len;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rule(id: &str, source: &str, target: &str) -> CorrectionRule {
        CorrectionRule {
            id: id.to_string(),
            source_phrase: source.to_string(),
            normalized_source_phrase: source.to_lowercase(),
            target_phrase: target.to_string(),
            language: None,
            rule_mode: "manual".to_string(),
            is_active: true,
            auto_apply: true,
            usage_count: 0,
            last_used_at: None,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    #[test]
    fn test_case_insensitive_match() {
        let rules = [make_rule("1", "clawd", "Claude")];
        let (out, fired) = apply_rules("clawd is cool, CLAWD!", &rules);
        assert_eq!(out, "Claude is cool, Claude!");
        assert_eq!(fired, vec!["1"]);
    }

    #[test]
    fn test_no_match_returns_unchanged() {
        let rules = [make_rule("1", "xyz", "ABC")];
        let (out, fired) = apply_rules("hello world", &rules);
        assert_eq!(out, "hello world");
        assert!(fired.is_empty());
    }

    #[test]
    fn test_multiple_rules() {
        let rules = [
            make_rule("1", "clawd", "Claude"),
            make_rule("2", "anthropick", "Anthropic"),
        ];
        let (out, fired) = apply_rules("clawd from anthropick", &rules);
        assert_eq!(out, "Claude from Anthropic");
        assert_eq!(fired, vec!["1", "2"]);
    }
}
