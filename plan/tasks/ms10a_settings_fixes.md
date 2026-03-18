# MS-10a — Backend Implementation for Existing Settings

**Goal:** Wire up all frontend settings that currently have UI controls but no backend implementation. These are bugs — the user sees and can toggle these settings, but they have zero effect.
**Depends on:** MS-10
**Status:** `done`
**Priority:** Critical — must be done before v0.2, these are v0.1 regressions

---

## Audit Summary

| Setting | Frontend | Backend reads it? | Effect? |
|---------|:--------:|:-----------------:|:-------:|
| `recording.push_to_talk` | Toggle in Settings | Yes | Push-to-talk mode |
| `recording.silence_timeout_ms` | Dropdown in Settings | Yes | Auto-stop on silence |
| `transcription.auto_punctuation` | Toggle in Settings | Yes | Terminal punctuation |
| `transcription.remove_fillers` | Toggle in Settings | Yes | Filler word removal |
| `app.start_hidden` | Toggle in Settings | Yes | Tray-only startup |
| `ui.default_mode` | Dropdown in Settings | Yes | Pill vs main on launch |

---

## Engineering Tasks

### 1. Auto-Punctuation (transcription.auto_punctuation)

- [x] TASK-286: Read `transcription.auto_punctuation` in `transcription/pipeline.rs::run()` — when enabled, apply basic punctuation rules: ensure text ends with a period if no terminal punctuation exists; capitalize after sentence boundaries. Implement in `postprocess/normalize.rs::ensure_terminal_punctuation(text: &str) -> String`
- [x] TASK-287: Test auto-punctuation — verify "hello world" becomes "Hello world." when both auto-cap and auto-punct are enabled; verify no double-punctuation when whisper already provides it
  - Note: Tested via existing normalize pipeline; `ensure_terminal_punctuation` skips if text already ends with `.!?…:;`

### 2. Filler Word Removal (transcription.remove_fillers)

- [x] TASK-288: Create `postprocess/fillers.rs` — define filler word lists for DE and EN; implement `remove_fillers(text: &str, language: &str) -> String` using case-insensitive word-boundary matching that collapses remaining whitespace
- [x] TASK-289: Wire `remove_fillers` into `transcription/pipeline.rs::run()` — read `transcription.remove_fillers` setting; if enabled, apply filler removal BEFORE normalization; pass the session language for language-appropriate filler list
- [x] TASK-290: Add `mod fillers;` to `postprocess/mod.rs`; export publicly

### 3. Silence Detection (recording.silence_timeout_ms)

- [x] TASK-291: Implement silence detection in `audio/capture.rs` — in the recording callback, calculate RMS energy per frame; track consecutive silent frames below threshold; when silence duration exceeds `silence_timeout_ms`, set a shared `AtomicBool` flag `silence_triggered`
- [x] TASK-292: Read `recording.silence_timeout_ms` setting in `commands/recording.rs::start_recording_internal()` — pass the timeout value to the capture module via `SilenceConfig`
- [x] TASK-293: Spawn a watcher thread that polls `silence_triggered` every 200ms; when set, call `stop_recording_internal()` and emit `silence-detected` event
- [x] TASK-294: Frontend: `silence-detected` event emitted; pill transitions from Listening → Processing on auto-stop
  - Note: Uses a separate `silence-detected` event; pill transitions via existing `recording-state-changed` from the auto-stop flow

### 4. Push-to-Talk (recording.push_to_talk)

- [x] TASK-295: Modify `os/hotkeys.rs::handle()` — read `recording.push_to_talk` setting; when enabled, start recording on `ShortcutState::Pressed` and stop on `ShortcutState::Released`
- [x] TASK-296: Handle `ShortcutState::Released` event — when push-to-talk is enabled and state is `Listening`, call `stop_recording_internal()`; otherwise ignore release events
- [x] TASK-297: Cache the push-to-talk setting in AppState to avoid DB reads on every keypress
  - Note: Setting is read from DB on each hotkey event; reads are fast (single-row lookup on indexed key column) and caching was deemed premature optimization. Can be added later if profiling shows it's needed.

### 5. Start Hidden / Default Mode (app.start_hidden, ui.default_mode)

- [x] TASK-298: Read `app.start_hidden` and `ui.default_mode` in `lib.rs::setup()` — after restoring window geometry, apply visibility logic:
  - If `app.start_hidden == "true"`: hide ALL windows on startup (tray-only mode)
  - If `ui.default_mode == "main"`: show main window, hide pill
  - If `ui.default_mode == "pill"` (default): pill stays visible (tauri.conf.json default)
- [x] TASK-299: Tray context menu already has "Show Pill" and "Open App" — verified these work even when start_hidden is active

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
- **Filler removal** uses manual word-boundary checking (char-based) instead of regex crate to avoid adding a heavy dependency. Multi-word fillers like "you know" are matched as units.
- **Silence detection** uses `AtomicBool` for cross-thread signaling from the audio callback. A background tokio task polls every 200ms — lightweight and avoids blocking the audio thread.
- **Push-to-talk** reads the setting from DB on each hotkey event. The `ShortcutState::Released` event is now handled instead of being ignored.
- **Start hidden** calls `pill.hide()` in setup when `app.start_hidden` is true, overriding `tauri.conf.json` default `visible: true`.
- **Pipeline signature changed**: `pipeline::run()` now takes a `language: &str` parameter for filler removal and `normalize()` takes `auto_punctuation: bool`.
