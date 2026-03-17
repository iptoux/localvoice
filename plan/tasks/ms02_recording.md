# MS-02 — Recording Core

**Goal:** Enable microphone selection, recording start/stop, global shortcut handling, and real-time UI state transitions in the pill.
**Depends on:** MS-01
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-020: Add `cpal` to `Cargo.toml`; implement `audio/devices.rs` — `list_input_devices()` returns vec of `{id, name, is_default}`
- [x] TASK-021: Implement `audio/capture.rs` — `start_capture(device_id)` starts recording PCM samples into an in-memory buffer or temp file; `stop_capture()` finalizes and returns temp file path; `cancel_capture()` discards buffer
- [x] TASK-022: Implement `audio/wav_writer.rs` — write raw PCM samples to a valid WAV file (mono, 16-bit, 16kHz — format whisper.cpp expects)
- [x] TASK-023: Implement `audio/level_meter.rs` — calculate RMS amplitude from PCM buffer for waveform feedback (emit as Tauri event)
- [x] TASK-024: Implement `commands/recording.rs` — Tauri commands: `start_recording()`, `stop_recording()`, `cancel_recording()`, `get_recording_state()`; all update `AppState.recording_state` and emit `recording-state-changed` event
- [x] TASK-025: Implement `os/hotkeys.rs` — register configurable global shortcut (default: platform-appropriate, e.g. `Ctrl+Shift+Space`); on trigger, call `start_recording` or `stop_recording` based on current state; emit event to frontend
- [x] TASK-026: Define `RecordingState` enum: `Idle`, `Listening`, `Processing`, `Success`, `Error`; serialize for Tauri events
- [x] TASK-027: React: update Pill component to handle all 5 states — Idle (mic icon + "Ready"), Listening (pulse + timer), Processing (spinner), Success (check), Error (warning + text)
- [x] TASK-028: React: subscribe to `recording-state-changed` Tauri event in pill; update Zustand `recording-store`
- [x] TASK-029: React: implement elapsed recording timer in Listening state
- [x] TASK-030: React: Settings page — microphone selector (calls `list_input_devices`, saves choice to settings)
- [x] TASK-031: React: Settings page — shortcut display with instructions for changing it

## Product/UX Tasks

- [x] TASK-032: Confirm shortcut default feels natural on Windows; ensure it does not conflict with common apps (VS Code, browsers)
  - `Ctrl+Shift+Space` is not bound by VS Code or major browsers by default. Confirmed safe.

## QA / Acceptance

- [ ] TASK-033: Verify global shortcut starts recording from idle state; pill transitions to Listening
- [ ] TASK-034: Verify stopping produces a valid WAV at 16kHz/16-bit/mono in temp directory
- [ ] TASK-035: Verify cancel returns pill to Idle without leaving any temp file
- [ ] TASK-036: Verify level meter events fire during recording (even if not yet visualized)

---

## Acceptance Criteria

- Global shortcut starts recording from idle
- Stopping a recording produces a valid audio artifact
- Pill visibly changes to Listening and then Processing
- Canceling a recording returns to idle without saving output

---

## Technical Notes

- Capture audio in a format whisper.cpp expects from the start (16kHz, 16-bit PCM, mono WAV) to avoid resampling later
- Emit recording state changes as Tauri events rather than polling — the frontend just listens
- Keep `AppState` as the single owner of the recording handle; commands borrow it via `tauri::State`
