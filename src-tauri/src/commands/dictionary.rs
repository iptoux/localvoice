use serde::Deserialize;
use tauri::State;

use crate::db::repositories::ambiguous_terms_repo::AmbiguousTerm;
use crate::db::repositories::dictionary_repo::{CorrectionRule, DictionaryEntry};
use crate::db::repositories::settings_repo;
use crate::dictionary::service;
use crate::errors::CmdResult;
use crate::state::AppState;

// ── Payload types ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntryPayload {
    pub phrase: String,
    pub language: Option<String>,
    pub entry_type: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRulePayload {
    pub source_phrase: String,
    pub target_phrase: String,
    pub language: Option<String>,
    pub auto_apply: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRulePayload {
    pub source_phrase: String,
    pub target_phrase: String,
    pub language: Option<String>,
    pub is_active: bool,
    pub auto_apply: bool,
}

// ── Dictionary entry commands ─────────────────────────────────────────────────

#[tauri::command]
pub fn list_dictionary_entries(state: State<AppState>) -> CmdResult<Vec<DictionaryEntry>> {
    service::list_entries(&state.db).map_err(Into::into)
}

#[tauri::command]
pub fn create_dictionary_entry(
    state: State<AppState>,
    payload: CreateEntryPayload,
) -> CmdResult<DictionaryEntry> {
    service::create_entry(
        &state.db,
        &payload.phrase,
        payload.language.as_deref(),
        &payload.entry_type,
        payload.notes.as_deref(),
    )
    .map_err(Into::into)
}

#[tauri::command]
pub fn update_dictionary_entry(
    state: State<AppState>,
    id: String,
    payload: CreateEntryPayload,
) -> CmdResult<()> {
    service::update_entry(
        &state.db,
        &id,
        &payload.phrase,
        payload.language.as_deref(),
        &payload.entry_type,
        payload.notes.as_deref(),
    )
    .map_err(Into::into)
}

#[tauri::command]
pub fn delete_dictionary_entry(state: State<AppState>, id: String) -> CmdResult<()> {
    service::delete_entry(&state.db, &id).map_err(Into::into)
}

// ── Correction rule commands ──────────────────────────────────────────────────

#[tauri::command]
pub fn list_correction_rules(state: State<AppState>) -> CmdResult<Vec<CorrectionRule>> {
    service::list_rules(&state.db).map_err(Into::into)
}

#[tauri::command]
pub fn create_correction_rule(
    state: State<AppState>,
    payload: CreateRulePayload,
) -> CmdResult<CorrectionRule> {
    service::create_rule(
        &state.db,
        &payload.source_phrase,
        &payload.target_phrase,
        payload.language.as_deref(),
        payload.auto_apply,
    )
    .map_err(Into::into)
}

#[tauri::command]
pub fn update_correction_rule(
    state: State<AppState>,
    id: String,
    payload: UpdateRulePayload,
) -> CmdResult<()> {
    service::update_rule(
        &state.db,
        &id,
        &payload.source_phrase,
        &payload.target_phrase,
        payload.language.as_deref(),
        payload.is_active,
        payload.auto_apply,
    )
    .map_err(Into::into)
}

#[tauri::command]
pub fn delete_correction_rule(state: State<AppState>, id: String) -> CmdResult<()> {
    service::delete_rule(&state.db, &id).map_err(Into::into)
}

// ── Ambiguity commands ────────────────────────────────────────────────────────

#[tauri::command]
pub fn list_ambiguous_terms(state: State<AppState>) -> CmdResult<Vec<AmbiguousTerm>> {
    let settings = settings_repo::get_all(&state.db).unwrap_or_default();
    let min_occ: i64 = settings
        .get("ambiguity.min_occurrences")
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);
    service::list_ambiguous_terms(&state.db, min_occ).map_err(Into::into)
}

#[tauri::command]
pub fn accept_ambiguity_suggestion(
    state: State<AppState>,
    id: String,
    target_phrase: String,
) -> CmdResult<()> {
    service::accept_ambiguity_suggestion(&state.db, &id, &target_phrase).map_err(Into::into)
}

#[tauri::command]
pub fn dismiss_ambiguity_suggestion(state: State<AppState>, id: String) -> CmdResult<()> {
    service::dismiss_ambiguity_suggestion(&state.db, &id).map_err(Into::into)
}
