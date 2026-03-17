# MS-05 — History

## What Was Built

Every completed transcription session is now persisted to SQLite and viewable in the **History** page. The page provides search, filtering, a session detail drawer, copy/delete/export actions, and offset-based pagination.

## Key Decisions

- **`db/models.rs`** centralises all DB model types (`Session`, `SessionSegment`, `SessionWithSegments`, `SessionFilter`). This keeps the repository layer thin and lets other modules (commands, orchestrator) import clean types without pulling in rusqlite.
- **Parameterized WHERE builder** in `sessions_repo::list_sessions`: conditions and parameter values are built together as parallel vecs. A `Vec<&dyn rusqlite::ToSql>` slice is passed to rusqlite, which is safe and avoids SQL injection. String interpolation is used only for the structural SQL (table/column names, `LIMIT`/`OFFSET` literals).
- **`rfd` for the save dialog** (export): lightweight native Rust crate for OS file dialogs; avoids the `tauri-plugin-dialog` plugin with its additional capability configuration overhead. Works synchronously from the Tauri command thread on Windows.
- **Session duration = recording start → transcription complete**: `AppState.recording_started_at` (a `chrono::DateTime<Utc>`) is set when `start_recording_internal` is called and consumed in `transcribe_and_emit`. This includes both recording and transcription time — accurate enough for WPM estimation.
- **Estimated WPM = word_count / (duration_ms / 60_000)**: stored as `REAL` in the DB. Used for MS-06 dashboard charts.
- **ON DELETE CASCADE** on `session_segments.session_id` was already in the v1 schema, so `delete_session` only needs to delete the parent row.

## Architecture Notes

### New Rust modules

| Module | Purpose |
|--------|---------|
| `src-tauri/src/db/models.rs` | `Session`, `SessionSegment`, `SessionWithSegments`, `SessionFilter` structs |
| `src-tauri/src/db/repositories/sessions_repo.rs` | `insert_session`, `insert_segments`, `list_sessions`, `get_session`, `delete_session`, `get_sessions_by_ids` |
| `src-tauri/src/history/export.rs` | `to_text()`, `to_json()` renderers |
| `src-tauri/src/commands/history.rs` | `list_sessions`, `get_session`, `delete_session`, `export_sessions` Tauri commands |

### Orchestrator additions

`transcribe_and_emit` now:
1. Takes the `recording_started_at` timestamp from `AppState` (clearing it).
2. Computes `duration_ms`, `word_count`, `char_count`, `avg_confidence`, `estimated_wpm`.
3. Calls `sessions_repo::insert_session` + `sessions_repo::insert_segments`.

### Frontend

- `src/lib/tauri.ts`: `listSessions`, `getSessionDetail`, `deleteSession`, `exportSessions` wrappers.
- `src/types/index.ts`: `Session` extended with `triggerType`, `inputDeviceId`, `outputTargetApp`, `createdAt`; new `SessionSegment`, `SessionWithSegments`, `SessionFilter` interfaces.
- `src/pages/History.tsx`: full page with debounced search, language/date filters, paginated session list, slide-in detail drawer, copy/delete/export actions.

## Known Limitations / Future Work

- Total count is approximated (no `COUNT(*)` query — avoids a second DB round-trip). The pagination shows `from–to` rather than exact totals.
- Export dialog is triggered from the Rust command thread via `rfd` blocking call; on macOS this would require dispatching to the main thread (deferred to cross-platform polish).
- `input_device_id` is read from the settings value at session-save time; it may be `None` if no explicit device was configured.
- Date range filter uses the ISO 8601 string from `started_at` directly — works correctly because SQLite string comparison is lexicographic and RFC 3339 is sortable.
