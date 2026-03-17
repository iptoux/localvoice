# MS-08 — Dictionary v1

## What Was Built

Manual dictionary entries (terms, names, acronyms) and correction rules that automatically replace misheard words in every transcript. Rules are applied in the transcription pipeline after whitespace normalisation, before output.

## Key Decisions

- **Return fired IDs from apply_rules** — `rules::apply_rules` returns `(String, Vec<String>)` rather than just the corrected text. The caller (orchestrator) uses the ID list to increment `usage_count` in the same transaction. This avoids a second pass over the rules.
- **Language matching: NULL = universal** — `list_active_rules(language)` uses `(language IS NULL OR language = ?)`. A rule with no language applies to all transcriptions; a DE rule only applies to German sessions.
- **Pipeline signature change** — `pipeline::run` now takes `active_rules: &[CorrectionRule]` as a fourth argument. Rules are loaded by the orchestrator (which already has DB access), keeping the pipeline itself stateless.
- **No word-boundary matching** — substring replacement is used for simplicity. A `case_sensitive` flag and word-boundary option are planned for a later milestone.

## Architecture Notes

```
dictionary/
  rules.rs       apply_rules(text, rules) → (corrected_text, fired_ids); unit-tested
  service.rs     thin wrapper over dictionary_repo; validation + record_rule_usage
db/repositories/
  dictionary_repo.rs   CRUD for dictionary_entries + correction_rules; increment_rule_usage
commands/
  dictionary.rs  8 Tauri commands (4 entry + 4 rule)
src/
  stores/dictionary-store.ts   Zustand; fetchAll, addRule, editRule, toggleRule, removeRule
  pages/Dictionary.tsx         Two-tab page: Rules (default), Terms; modals for add/edit
transcription/
  pipeline.rs    Now accepts active_rules; applies after normalize, before output
  orchestrator.rs  Loads active rules, calls pipeline, records usage
```

## Known Limitations / Future Work

- No word-boundary enforcement — "AI" would also match inside "RAIN". Future: add `word_boundary` flag.
- No `case_sensitive` flag yet — all rules are case-insensitive.
- History detail does not show which rules fired on a given session (TASK-128 deferred).
