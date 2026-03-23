# MS-16 — Advanced History, Push-to-Talk & Audio Playback

**Goal:** Enhance history with bulk operations and pagination, add push-to-talk recording mode as an alternative to toggle, and enable optional session audio playback.
**Depends on:** MS-14
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-221: Rust: `commands/history.rs::bulk_delete_sessions(session_ids: Vec<String>)` — deletes multiple sessions and their segments in a single transaction
- [x] TASK-222: Rust: `commands/history.rs::bulk_export_sessions(session_ids, format)` — exports multiple sessions at once; format: "txt", "json", "csv"
- [x] TASK-223: React: History page — multi-select mode with checkboxes; bulk action bar appears at top with "Delete Selected", "Export Selected" buttons; "Select All" checkbox
- [x] TASK-224: React: History page — proper cursor-based or offset pagination with page size selector (25/50/100); update `listSessions` filter to pass offset+limit; show total count
- [x] TASK-225: React: History page — additional filter: "Has audio" toggle (only sessions with `audio_path`); date range quick presets (Today, This Week, This Month)
- [x] TASK-226: Push-to-talk mode in Rust — `os/hotkeys.rs` detects key-down and key-up events for the configured shortcut; when `recording.push_to_talk` is "true", key-down starts recording, key-up stops; requires `tauri-plugin-global-shortcut` key-up support or custom Windows hook
- [x] TASK-227: React: Settings page — "Recording Mode" selector: "Toggle" (current default) vs. "Push-to-Talk"; updates `recording.push_to_talk` setting
- [x] TASK-228: Pill indication for push-to-talk — show "Hold to record" hint in idle state when push-to-talk mode is active; show key icon instead of mic icon
- [x] TASK-229: Audio playback in session detail — React: `components/history/AudioPlayer.tsx` using HTML5 `<audio>` element; shows play/pause, seek bar, duration; only renders when `session.audioPath` exists
- [x] TASK-230: Tauri command `get_audio_file_path(session_id)` in `commands/history.rs` — returns the absolute path to the session's audio file; frontend uses `convertFileSrc()` from `@tauri-apps/api` to get a playable URL
- [x] TASK-231: CSV export format — add CSV output to `history/export.rs::export_sessions()`; columns: date, language, model, duration, word_count, wpm, raw_text, cleaned_text

## QA / Acceptance

- [x] TASK-231a: Verify bulk delete removes all selected sessions correctly
- [X] TASK-231b: Verify push-to-talk stops recording immediately on key release
- [X] TASK-231c: Verify audio playback plays the correct file for each session
- [X] TASK-231d: Verify CSV export opens correctly in Excel and LibreOffice Calc

---

## Acceptance Criteria

- Bulk delete and export work for multiple selected sessions
- Pagination handles large history gracefully with configurable page size
- Push-to-talk works as hold-to-record when enabled
- Audio playback works for sessions that have retained audio files
- CSV export produces valid, importable CSV files

---

## Technical Notes

- Push-to-talk requires key-up detection, which `tauri-plugin-global-shortcut` may not support directly; fallback is a custom Windows hook via `windows-sys`
- Audio playback uses Tauri's `convertFileSrc()` to convert local file paths to webview-accessible URLs
- Bulk operations wrap in a single SQLite transaction for atomicity
- CSV export uses comma delimiter with proper quoting for texts containing commas/newlines
