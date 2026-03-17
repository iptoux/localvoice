# MS-09 — Ambiguity v1

## What Was Built

Automatic detection of low-confidence transcript segments, persistence of recurring ambiguous phrases, and a Suggestions UI in the Dictionary page where users can turn any flagged phrase into a correction rule with one click.

## Key Decisions

- **Detection at orchestrator level, not pipeline**: `postprocess::ambiguity::detect` runs in `transcribe_and_emit` (after session persistence) rather than inside `transcription/pipeline.rs`. The pipeline already returns the cleaned segments; ambiguity detection operates on those, keeping the pipeline's responsibility narrow.
- **`dismissed_at_occurrences` column (migration v2)**: Tracks the occurrence count at the time of dismissal. A dismissed term is re-surfaced automatically when `occurrences >= dismissed_at_occurrences + 5`, ensuring high-signal phrases are never silently suppressed forever.
- **Rolling average confidence**: On each upsert, `avg_confidence = (avg * old_occ + new_conf) / new_occ`. This smooths over outlier transcriptions.
- **Suggestion derivation by normalized phrase match**: `dictionary/suggestions.rs` compares `normalized_phrase` (lowercased) of ambiguous terms against `normalized_source_phrase` of correction rules. No fuzzy matching — exact match only, preventing false suggestions.
- **Accept creates a "suggested" rule**: `accept_ambiguity_suggestion` inserts a `correction_rule` with `rule_mode = 'suggested'` and `auto_apply = true`, then marks the term resolved (dismissed). The rule enters the pipeline immediately on the next transcription.
- **Inline accept UX**: Instead of a modal, the "Accept" flow uses an inline input row within the Suggestions tab — fewer clicks, same context.

## Architecture Notes

```
postprocess/ambiguity.rs        — pure detection logic (no DB)
db/repositories/
  ambiguous_terms_repo.rs       — upsert, list_active, dismiss, get, set_suggested_target
db/migrations.rs                — v2 adds dismissed_at_occurrences column
dictionary/
  suggestions.rs                — derives suggested_target from existing rules
  service.rs                    — list_ambiguous_terms, accept/dismiss wrappers
commands/dictionary.rs          — 3 new Tauri commands
transcription/orchestrator.rs   — calls detect() + upsert + apply_suggestions after session save
src/stores/ambiguity-store.ts   — Zustand store (fetch, accept, dismiss)
src/pages/Dictionary.tsx        — Suggestions tab (third tab between Rules and Terms)
```

**Detection flow:**
1. `transcribe_and_emit` calls `postprocess::ambiguity::detect(&result.segments, threshold)`
2. For each candidate, `ambiguous_terms_repo::upsert` is called (insert or increment)
3. If any candidates were found, `dictionary::suggestions::apply_suggestions` fills in `suggested_target` for terms matching an existing rule

## Known Limitations / Future Work

- Suggestion derivation is exact-match only. Fuzzy/phonetic matching would catch more cases (e.g. "clawd" → "Claude") but adds complexity.
- TASK-142 (threshold tuning) is still open — needs real-world testing on DE/EN audio.
- History-based heuristic (most-frequent cleaned-text variant for a phrase) is not yet implemented — only rule-based suggestions exist.
- The Suggestions tab does not auto-refresh when new transcriptions complete; user must click Refresh manually.
