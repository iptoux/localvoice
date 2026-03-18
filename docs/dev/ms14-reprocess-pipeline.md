# MS-14 — Session Reprocessing & Configurable Post-Processing

## What Was Built
- Reprocess existing sessions with a different whisper model or language
- Optional audio file persistence for later reprocessing
- Audio file cleanup on startup (configurable retention period)
- Post-processing pipeline toggles in settings (auto-cap, auto-punct, filler removal, dictionary auto-apply)
- "Original" tab in session detail to compare pre-reprocess text

## Key Decisions
- Audio persistence is opt-in (default off) to avoid disk bloat
- Audio files stored in `{app_data_dir}/audio/` with UUID filenames
- Cleanup runs once on startup (no background timer)
- Reprocessing preserves `original_raw_text` on first reprocess only
- Migration 5 combines TASK-203 (session columns) and TASK-210 (settings) into one

## Architecture Notes
- `history/reprocess.rs` — core reprocess logic: loads session audio_path, runs whisper sidecar, pipeline, updates DB
- `audio/cleanup.rs` — startup job that deletes WAV files older than retention period and nulls `audio_path` in sessions
- `commands/history.rs::reprocess_session` — Tauri command wrapper, emits `session-reprocessed` event
- Audio persistence moved to `orchestrator.rs::transcribe_and_emit` (not capture.rs) for cleaner separation
- `pipeline.rs` already reads all settings toggles — no changes needed for TASK-207

## Known Limitations / Future Work
- `max_audio_storage_mb` setting is stored but not yet enforced (no total-size check during persistence)
- Sessions recorded before `recording.keep_audio` was enabled cannot be reprocessed (no audio file)
- Reprocess dialog is inline panel, not a full modal — could be enhanced with side-by-side diff view
