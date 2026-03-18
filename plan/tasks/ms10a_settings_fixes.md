# MS-10a — Backend Implementation for Existing Settings

**Goal:** Wire up all frontend settings that currently have UI controls but no backend implementation. These are bugs — the user sees and can toggle these settings, but they have zero effect.
**Depends on:** MS-10
**Status:** `todo`
**Priority:** Critical — must be done before v0.2, these are v0.1 regressions

---

## Audit Summary

| Setting | Frontend | Backend reads it? | Effect? |
|---------|:--------:|:-----------------:|:-------:|
| `recording.push_to_talk` | Toggle in Settings | No | None |
| `recording.silence_timeout_ms` | Dropdown in Settings | No | None |
| `transcription.auto_punctuation` | Toggle in Settings | No | None |
| `transcription.remove_fillers` | Toggle in Settings | No | None |
| `app.start_hidden` | Toggle in Settings | No | None |
| `ui.default_mode` | Dropdown in Settings | No | None |

All 6 settings exist as DB defaults (migrations + settings_repo) and have UI controls in `src/pages/SettingsPage.tsx`, but the Rust backend never reads or acts on their values.

---

## Engineering Tasks

### 1. Auto-Punctuation (transcription.auto_punctuation)

- [ ] TASK-286: Read `transcription.auto_punctuation` in `transcription/pipeline.rs::run()` — when enabled, apply basic punctuation rules: ensure text ends with a period if no terminal punctuation exists; capitalize after sentence boundaries. Implement in `postprocess/normalize.rs::ensure_terminal_punctuation(text: &str) -> String`
- [ ] TASK-287: Test auto-punctuation — verify "hello world" becomes "Hello world." when both auto-cap and auto-punct are enabled; verify no double-punctuation when whisper already provides it

### 2. Filler Word Removal (transcription.remove_fillers)

- [ ] TASK-288: Create `postprocess/fillers.rs` — define filler word lists for DE (`äh`, `ähm`, `öhm`, `halt`, `sozusagen`, `quasi`, `irgendwie`) and EN (`uh`, `um`, `uhm`, `you know`, `like`, `basically`, `actually`, `sort of`, `kind of`); implement `remove_fillers(text: &str, language: &str) -> String` using case-insensitive whole-word matching (regex `\b` boundaries) that collapses remaining whitespace
- [ ] TASK-289: Wire `remove_fillers` into `transcription/pipeline.rs::run()` — read `transcription.remove_fillers` setting; if enabled, apply filler removal AFTER normalization but BEFORE correction rules; pass the session language for language-appropriate filler list
- [ ] TASK-290: Add `mod fillers;` to `postprocess/mod.rs`; export publicly

### 3. Silence Detection (recording.silence_timeout_ms)

- [ ] TASK-291: Implement silence detection in `audio/capture.rs` — in the recording callback, calculate RMS energy per frame; track consecutive silent frames below threshold (hardcoded 0.01 RMS for now); when silence duration exceeds `silence_timeout_ms`, set a shared `AtomicBool` flag `silence_triggered`
- [ ] TASK-292: Read `recording.silence_timeout_ms` setting in `commands/recording.rs::start_recording_internal()` — pass the timeout value to the capture module; default to 0 (disabled) if setting is missing or 0
- [ ] TASK-293: In the recording loop or a watcher thread, check `silence_triggered` flag; when set, call `stop_recording_internal()` automatically and emit `recording-state-changed` with a `"silence"` trigger type
- [ ] TASK-294: Frontend: Listen for silence-triggered stop — pill shows "Silence detected" briefly before transitioning to processing (use existing `recording-state-changed` event with an extra `trigger` field, or a separate `silence-detected` event)

### 4. Push-to-Talk (recording.push_to_talk)

- [ ] TASK-295: Modify `os/hotkeys.rs::handle()` — read `recording.push_to_talk` setting from AppState; when enabled, start recording on `ShortcutState::Pressed` AND stop recording on key release; currently the handler ignores all non-Pressed events (line 13: `if event.state() != ShortcutState::Pressed { return; }`)
- [ ] TASK-296: Handle `ShortcutState::Released` event — when push-to-talk is enabled and state is `Listening`, call `stop_recording_internal()`; when push-to-talk is disabled, ignore release events (current behavior)
- [ ] TASK-297: Cache the push-to-talk setting in AppState to avoid DB reads on every keypress — read on startup and refresh when `update_setting` is called with key `recording.push_to_talk`

### 5. Start Hidden / Default Mode (app.start_hidden, ui.default_mode)

- [ ] TASK-298: Read `app.start_hidden` and `ui.default_mode` in `lib.rs::setup()` — after restoring window geometry, apply visibility logic:
  - If `ui.default_mode == "main"`: show main window, hide pill
  - If `ui.default_mode == "pill"` (default): show pill, keep main hidden (current hardcoded behavior)
  - If `app.start_hidden == "true"`: hide ALL windows on startup (tray-only mode); user opens via tray
- [ ] TASK-299: Update `os/tray.rs` context menu — ensure "Show Pill" and "Open App" always work even when everything starts hidden; make tray the reliable entry point when `start_hidden` is active

---

## QA / Acceptance

- [ ] TASK-300: Verify auto-punctuation adds terminal period when whisper output lacks it
- [ ] TASK-301: Verify filler removal strips "uh", "um", "äh" without removing legitimate words containing those substrings (e.g., "umbrella" must not become "brella")
- [ ] TASK-302: Verify silence detection stops recording after configured timeout with no speech
- [ ] TASK-303: Verify push-to-talk starts on key down and stops on key up
- [ ] TASK-304: Verify `ui.default_mode = "main"` opens the main window instead of pill on launch
- [ ] TASK-305: Verify `app.start_hidden = true` starts the app with no visible windows (tray only)

---

## Acceptance Criteria

- Every toggle and dropdown in the Settings page has a measurable effect on app behavior
- Auto-punctuation ensures terminal punctuation on transcriptions
- Filler words are removed for both German and English
- Silence detection auto-stops recording after configured timeout
- Push-to-talk works as hold-to-record mode
- Start hidden and default view mode control window visibility on startup

---

## Technical Notes

- **Auto-punctuation** is deliberately simple — just terminal period + sentence-start capitalization. whisper.cpp usually provides good punctuation already; this is a safety net.
- **Filler removal** must use word-boundary matching (`\b`), not substring replacement. "umbrella" must not be affected by removing "um". Consider using `regex` crate.
- **Silence detection** runs in the audio callback thread. Use `AtomicBool` or `Arc<Mutex>` for cross-thread signaling — not channels (too heavy for per-frame checks).
- **Push-to-talk** requires `ShortcutState::Released` handling. `tauri-plugin-global-shortcut` v2 does support this — the handler already receives `ShortcutEvent` with a `state()` method. The current code explicitly ignores non-Pressed events on line 13 of `hotkeys.rs`.
- **Start hidden** interacts with `tauri.conf.json` — pill is `visible: true` by default. The setup code must call `pill.hide()` when `app.start_hidden` is true.
