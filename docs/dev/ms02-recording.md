# MS-02 — Recording Core

## What Was Built

Audio capture pipeline, global hotkey, and real-time UI state transitions.
The pill now shows five distinct states (Idle → Listening → Processing → Success/Error)
driven by Tauri events from the Rust backend.

## Key Decisions

- **cpal for audio input**: Industry-standard Rust audio library. Tries to negotiate 16 kHz mono first;
  falls back to the device default if not supported. The actual sample rate is written into the WAV
  header so MS-03 can handle resampling if needed.
- **hound for WAV encoding**: Writes 16-bit signed mono PCM — the format whisper.cpp expects without
  conversion. WAV is finalised on `stop_capture()`, not during recording.
- **tauri-plugin-global-shortcut with Builder + `with_handler`**: The handler closure is attached at
  plugin-build time, before `setup`. The shortcut string itself is registered inside `setup` after
  `AppState` is available (so we can read `recording.shortcut` from the DB).
- **Throttled `audio-level` emit**: The cpal callback emits an RMS float to the frontend at most once
  per 80 ms, preventing event flooding on fast audio hardware.
- **`cpal::Stream` held in `AppState`**: Dropping the `ActiveRecording` struct automatically stops
  the hardware stream. The samples `Arc<Mutex<Vec<i16>>>` is extracted before dropping.
- **`unsafe impl Send for ActiveRecording`**: cpal's WASAPI backend declares `Stream: Send` via
  `unsafe impl`. We mirror that at the wrapper level so AppState satisfies Tauri's `Send + Sync` bound.

## Architecture Notes

```
os/hotkeys.rs     — registers shortcut; calls command internals on keypress
commands/recording.rs
  ├── start_recording_internal()   — used by command + hotkey
  ├── stop_recording_internal()    — returns WAV path; transitions to Processing
  └── cancel_recording_internal()  — discards buffer; back to Idle

audio/
  devices.rs    — list_input_devices(), get_input_device(Option<&str>)
  capture.rs    — start_capture() / stop_capture() / cancel_capture()
  wav_writer.rs — write_wav(samples, rate, path)
  level_meter.rs — calculate_rms(data: &[f32]) -> f32

Frontend:
  PillApp.tsx          — subscribes to recording-state-changed + audio-level
  stores/app-store.ts  — holds recordingState, audioLevel, recordingError, audioDevices
  components/pill/Pill.tsx — 5-state UI + elapsed timer (Listening only)
  pages/SettingsPage.tsx   — microphone selector, shortcut badge
```

## Emission flow

```
Rust command / hotkey
  → emit("recording-state-changed", { state, error? })
  → PillApp.tsx listener → useAppStore.setRecordingState()
  → Pill re-renders

cpal callback (every ~80 ms)
  → emit("audio-level", rms: f32)
  → PillApp.tsx listener → useAppStore.setAudioLevel()
```

## Known Limitations / Future Work

- No resampling: if the device doesn't support 16 kHz, the WAV will be at a different rate.
  MS-03 must handle this when calling whisper.cpp.
- The global shortcut string is read-only from the UI (Settings shows it with instructions
  to edit the DB). A proper shortcut recorder is planned for MS-10.
- `recording.device_id` is persisted but there is no automatic fallback if the stored device
  is disconnected at startup. MS-10 polish should add graceful handling.
- The Success/Error states are set only by MS-03 (transcription). In MS-02, `stop_recording`
  leaves the pill in **Processing** — this is correct and expected.
