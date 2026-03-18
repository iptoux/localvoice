# MS-14 — Session Reprocessing & Configurable Post-Processing

**Goal:** Allow re-running transcription on existing sessions with a different model or settings, and let users toggle individual post-processing pipeline steps.
**Depends on:** MS-12
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-201: Rust: `history/reprocess.rs` — `reprocess_session(session_id, language, model_id)` loads the original audio file path from `audio_path` column, invokes whisper sidecar, runs post-processing, updates session record with new results while preserving original `raw_text` in `original_raw_text` column
- [ ] TASK-202: Tauri command `reprocess_session` in `commands/history.rs` — wraps `history/reprocess::reprocess_session()`; emits `session-reprocessed` event on completion
- [ ] TASK-203: DB migration 5 — add `audio_path TEXT` column to `sessions` table; add `original_raw_text TEXT` column; add `reprocessed_count INTEGER NOT NULL DEFAULT 0` column
- [ ] TASK-204: Update `audio/capture.rs` — optionally save recorded audio to app data dir (setting `recording.keep_audio`, default: false); store path in session's `audio_path`; respect max storage setting
- [ ] TASK-205: Frontend: `reprocessSession(sessionId, language?, modelId?)` in `lib/tauri.ts`; add "Reprocess" button to session detail view in History page
- [ ] TASK-206: React: Reprocess dialog — modal showing language picker, model picker, and "Reprocess" button; shows progress spinner; displays diff of old vs. new cleaned text on completion
- [ ] TASK-207: Configurable post-processing pipeline — `postprocess/pipeline.rs` reads settings to determine which steps to run: normalization (always on), punctuation (`transcription.auto_punctuation`), capitalization (`transcription.auto_capitalization`), filler removal (`transcription.remove_fillers`), dictionary auto-replace (`dictionary.auto_apply_rules`)
- [ ] TASK-208: React: Settings page — "Post-Processing" section with toggles for each pipeline step (punctuation, capitalization, filler removal, dictionary auto-apply); each toggle maps to its existing setting key
- [ ] TASK-209: Audio file cleanup job — on app startup, delete audio files older than configurable retention period (`recording.audio_retention_days`, default: 7); add setting in migration 5
- [ ] TASK-210: DB migration 5 — add settings: `recording.keep_audio` (false), `recording.audio_retention_days` (7), `recording.max_audio_storage_mb` (500)

## QA / Acceptance

- [ ] TASK-210a: Verify reprocessing with a different model produces different output
- [ ] TASK-210b: Verify original raw text is preserved after reprocessing
- [ ] TASK-210c: Verify audio cleanup removes files older than retention period
- [ ] TASK-210d: Verify toggling pipeline steps changes transcription output

---

## Acceptance Criteria

- Users can reprocess a session with a different model and see updated text
- Original raw text is preserved for comparison
- Audio files are kept when enabled and automatically cleaned after retention period
- Individual post-processing steps can be toggled independently in settings

---

## Technical Notes

- Audio retention is **opt-in** (default off) to avoid consuming significant disk space
- Reprocessing requires `audio_path` to exist — sessions recorded before this feature cannot be reprocessed
- The cleanup job runs once on startup, not as a background timer, to keep resource usage minimal
- TASK-203 and TASK-210 can be combined into a single migration 5 file
