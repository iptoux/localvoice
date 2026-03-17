use crate::db::repositories::{ambiguous_terms_repo, dictionary_repo};
use crate::db::DbConn;
use crate::errors::AppError;

/// Scans active ambiguous terms that have no `suggested_target` yet, and fills
/// one in if an existing correction rule's normalized source phrase matches the
/// term's normalized phrase.
pub fn apply_suggestions(db: &DbConn) -> Result<(), AppError> {
    let terms = ambiguous_terms_repo::list_active(db, 1)?;
    let rules = dictionary_repo::list_rules(db)?;

    for term in &terms {
        if term.suggested_target.is_some() {
            continue;
        }
        if let Some(rule) = rules
            .iter()
            .find(|r| r.normalized_source_phrase == term.normalized_phrase)
        {
            ambiguous_terms_repo::set_suggested_target(db, &term.id, Some(&rule.target_phrase))?;
        }
    }
    Ok(())
}
