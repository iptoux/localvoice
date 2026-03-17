# MS-09 — Ambiguity v1

**Goal:** Detect unclear or repeatedly problematic words heuristically and surface actionable suggestions to the user.
**Depends on:** MS-08
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-133: Implement `postprocess/ambiguity.rs` — detect low-confidence segments (confidence < threshold from settings, default 0.6) and/or short segments repeated across multiple sessions with varying transcriptions; return `Vec<AmbiguousTerm>` candidates
- [ ] TASK-134: Implement ambiguity recording in pipeline — after rule application in `transcription/pipeline.rs`, for each detected candidate: if `ambiguous_terms` row exists increment `occurrences` + update `avg_confidence` and `last_seen_at`; else insert new row
- [ ] TASK-135: Implement `dictionary/suggestions.rs` — for each ambiguous term, derive `suggested_target` if a correction rule already exists for a similar phrase, or if the term has been manually corrected in history (heuristic: most frequent cleaned-text variant)
- [ ] TASK-136: Implement `commands/dictionary.rs` additions — `list_ambiguous_terms()` (excludes dismissed), `accept_ambiguity_suggestion(id, target_phrase)`, `dismiss_ambiguity_suggestion(id)`
- [ ] TASK-137: `accept_ambiguity_suggestion` must: create a `correction_rule` with `rule_mode='suggested'`, `source_phrase=ambiguous_term.phrase`, `target_phrase=provided target`; mark the ambiguous term's `dismissed=1` (it was resolved)
- [ ] TASK-138: `dismiss_ambiguity_suggestion` must: set `dismissed=1` on the term; the term reappears only if `occurrences` increases significantly after dismissal (heuristic: +5 more occurrences)
- [ ] TASK-139: React: Ambiguous Terms section in Dictionary page (third tab or sub-section under Rules) — list rows: phrase, occurrences, avg confidence, suggested target, Accept / Dismiss actions
- [ ] TASK-140: React: Accept flow — opens "Create Correction Rule" modal pre-filled with `source_phrase` and `suggested_target`; on confirm calls `accept_ambiguity_suggestion`
- [ ] TASK-141: React: Dismiss action — calls `dismiss_ambiguity_suggestion(id)`; row disappears from list

## Product/UX Tasks

- [ ] TASK-142: Tune default confidence threshold — test on real German/English clips; confirm it catches genuine errors without flooding the list with false positives

## QA / Acceptance

- [ ] TASK-143: Verify ambiguous terms list is populated after multiple sessions with a consistently low-confidence segment
- [ ] TASK-144: Verify accepting a suggestion creates a functional correction rule (rule fires in next transcription)
- [ ] TASK-145: Verify dismissed items do not reappear unless occurrence count grows significantly after dismissal
- [ ] TASK-146: Verify zero ambiguous terms scenario renders empty state without errors

---

## Acceptance Criteria

- Repeatedly problematic terms appear in the ambiguity list
- User can accept a suggestion and create a correction rule
- Dismissed items no longer reappear without new evidence

---

## Technical Notes

- Start with conservative heuristics (confidence threshold + minimum occurrences ≥ 3) to avoid overwhelming the user with false positives
- Repeated correction patterns are a stronger signal than confidence alone — if a user has manually corrected the same word 3+ times, it should surface here
- whisper.cpp may not always provide per-token confidence; fall back to segment-level confidence or occurrence-based heuristics when unavailable
