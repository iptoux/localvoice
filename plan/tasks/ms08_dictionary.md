# MS-08 — Dictionary v1

**Goal:** Introduce manual dictionary entries and correction rules that automatically improve transcripts over time.
**Depends on:** MS-04
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-117: Implement `db/repositories/dictionary_repo.rs` — CRUD for `dictionary_entries`, CRUD for `correction_rules`; `increment_rule_usage(id)`; `list_active_rules(language?)`
- [ ] TASK-118: Implement `dictionary/rules.rs` — `apply_rules(text: &str, rules: &[CorrectionRule]) -> String` — iterate active rules, replace `source_phrase` with `target_phrase` (case-insensitive match, preserve casing of surrounding text)
- [ ] TASK-119: Implement `dictionary/service.rs` — wraps `dictionary_repo`; exposes `list_entries`, `create_entry`, `update_entry`, `delete_entry`, `list_rules`, `create_rule`, `update_rule`, `delete_rule`; calls `increment_rule_usage` when a rule fires
- [ ] TASK-120: Integrate rule application into transcription pipeline — in `transcription/pipeline.rs`, after `normalize.rs` but before output, call `rules::apply_rules` with active rules for current language
- [ ] TASK-121: Implement `commands/dictionary.rs` — Tauri commands: `list_dictionary_entries()`, `create_dictionary_entry(payload)`, `update_dictionary_entry(id, payload)`, `delete_dictionary_entry(id)`, `list_correction_rules()`, `create_correction_rule(payload)`, `update_correction_rule(id, payload)`, `delete_correction_rule(id)`
- [ ] TASK-122: React: Dictionary page with two tabs — "Terms" and "Rules"
- [ ] TASK-123: React: Terms tab — list of entries (phrase, type, language, notes), Add/Edit/Delete with inline or modal form; `entry_type` values: term, name, acronym, product, custom
- [ ] TASK-124: React: Rules tab — list rows (source → target, language, mode badge, active toggle, usage count); sortable by usage_count
- [ ] TASK-125: React: Add/Edit correction rule modal — fields: source phrase, target phrase, language (optional), auto_apply toggle, active toggle
- [ ] TASK-126: React: Active toggle on rule row — calls `update_correction_rule(id, { is_active })` without opening modal
- [ ] TASK-127: React: Dictionary store slice — `stores/dictionary-store.ts` fetches entries and rules on first load

## Product/UX Tasks

- [ ] TASK-128: Validate rule application UX — confirm user can see which rules fired on a session (future: could be shown in history detail)

## QA / Acceptance

- [ ] TASK-129: Verify adding a correction rule (e.g. "clawd" → "Claude") changes the next transcript that contains "clawd"
- [ ] TASK-130: Verify disabling a rule (`is_active=false`) stops it from applying in future transcripts
- [ ] TASK-131: Verify `usage_count` increments each time a rule fires during transcription
- [ ] TASK-132: Verify deleting a rule does not affect already-stored session transcripts

---

## Acceptance Criteria

- A configured correction rule changes future transcripts automatically
- Rules can be disabled without deletion
- Usage count increases when a rule is applied

---

## Technical Notes

- Run rule application after `normalize.rs` but before ambiguity detection (MS-09) and output — so replacements affect final transcript quality
- Case-insensitive matching is the default; consider a `case_sensitive` flag in the schema for power users in a later milestone
- Track `usage_count` now — it will power the "most valuable rules" metric in the dashboard and rule prioritization UX later
