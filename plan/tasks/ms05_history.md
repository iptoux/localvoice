# MS-05 — History

**Goal:** Persist past dictation sessions and provide a browsable history with details, search, filtering, copy, and deletion.
**Depends on:** MS-04
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-068: Implement `db/repositories/sessions_repo.rs` — `insert_session`, `list_sessions(filter, limit, offset)`, `get_session(id)`, `delete_session(id)`, `search_sessions(query)`; also insert/fetch associated `session_segments`
- [x] TASK-069: Save completed session to DB at end of transcription pipeline — populate all `sessions` columns (raw_text, cleaned_text, language, model_id, word_count, char_count, duration_ms, avg_confidence, estimated_wpm, output_mode, inserted_successfully, error_message)
- [x] TASK-070: Save `session_segments` rows linked by `session_id` after each transcription
- [x] TASK-071: Implement `commands/history.rs` — Tauri commands: `list_sessions(filter)`, `get_session(session_id)`, `delete_session(session_id)`
- [x] TASK-072: Implement `history/export.rs` — export as plain text (one session per block) or JSON array; write to user-chosen path via Tauri dialog
- [x] TASK-073: Implement Tauri command `export_sessions(session_ids, format)` — opens save dialog, writes file
- [x] TASK-074: React: History page — list of session rows (timestamp, language badge, word count, first 60 chars of cleaned text)
- [x] TASK-075: React: Search bar — filters list client-side or calls `search_sessions` command; debounced input
- [x] TASK-076: React: Filter bar — language (all/de/en), date range picker, model filter
- [x] TASK-077: React: Session detail page/drawer — raw transcript vs cleaned transcript side-by-side; segments list optional
- [x] TASK-078: React: Session detail actions — Copy cleaned text, Copy raw text, Delete session (with confirmation), Export single session
- [x] TASK-079: React: Pagination or infinite scroll for history list

## Product/UX Tasks

- [ ] TASK-080: Review session list information density — confirm row shows enough context to identify a session without opening it

## QA / Acceptance

- [ ] TASK-081: Verify sessions persist across app restarts
- [ ] TASK-082: Verify search with a known word returns the correct session
- [ ] TASK-083: Verify deleting a session removes it from DB and list immediately
- [ ] TASK-084: Verify export produces a valid file in chosen format

---

## Acceptance Criteria

- Completed transcriptions appear in history automatically
- Search returns relevant prior sessions
- Deleting a session removes it from the list and database

---

## Technical Notes

- Persist both `raw_text` and `cleaned_text` so reprocessing remains possible in MS-09 without data loss
- Design history queries with `LIMIT`/`OFFSET` pagination in mind even if the initial UI is a simple list
- Use `ON DELETE CASCADE` on `session_segments.session_id` so deleting a session cleans up segments automatically (already in schema)
- Used `rfd = "0.15"` for native save-file dialog (avoids tauri-plugin-dialog complexity)
- `db/models.rs` added as the central types module for DB row structs
- Session duration tracks from recording start to transcription complete (stored in `AppState.recording_started_at`)
