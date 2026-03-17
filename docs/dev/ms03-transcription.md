# MS-03 — Local Transcription

## What Was Built

End-to-end offline transcription via whisper.cpp CLI sidecar.
After every `stop_recording`, the Rust backend automatically spawns the sidecar in a
background thread, parses the output, normalises the text, and emits the result to
the frontend. The pill transitions to **Success** (with a transcript preview) or **Error**.

## Key Decisions

- **`std::process::Command` over `tauri-plugin-shell`**: Rust-side process spawning needs no
  Tauri plugin. The shell plugin is only required for JS-side subprocess access (not our use-case).
- **JSON file output preferred over stdout parsing**: `whisper-cli --output-json` writes a
  structured `.json` file that includes per-token confidence scores. The parser falls back to
  stdout SRT-format parsing if the file is absent (older whisper.cpp builds).
- **Background thread via `tokio::spawn` + `spawn_blocking`**: Transcription is synchronous
  (runs a subprocess). We wrap it in `spawn_blocking` so the async Tauri runtime isn't blocked,
  and the command returns immediately while the pill shows "Transcribing…".
- **`emit_recording_state` moved to `state/app_state.rs`**: Both `commands/recording.rs` and
  `transcription/orchestrator.rs` need to emit state changes. Centralising the function avoids
  duplication without creating circular imports.
- **Placeholder binary in `src-tauri/binaries/`**: Tauri's build script validates `externalBin`
  paths at compile time. A placeholder `.exe` file satisfies this check during development.
  The real binary must be substituted before shipping.

## Architecture Notes

```
whisper_sidecar.rs  — resolve binary path, resolve model path, invoke CLI
parser.rs           — parse JSON file (with confidence) OR stdout SRT (fallback)
language.rs         — map ISO 639-1 codes to whisper CLI flags
pipeline.rs         — post-processing: collapse whitespace, capitalise sentences
orchestrator.rs     — coordinate all steps; emit events via transcribe_and_emit()

commands/transcription.rs
  transcribe_last_recording  — manual re-trigger from frontend
  get_last_transcription     — retrieve cached result

state/app_state.rs
  last_wav_path        — set after stop_recording; used by transcribe_last_recording
  last_transcription   — cached after each successful transcription

Events emitted:
  recording-state-changed { state: "success"|"error", error? }
  transcription-completed  TranscriptionResult
```

## Binary Setup (Manual Step)

1. Download a whisper.cpp release from https://github.com/ggerganov/whisper.cpp/releases
2. Rename the Windows CLI binary to `whisper-cli-x86_64-pc-windows-msvc.exe`
3. Place it in `src-tauri/binaries/`
4. Download a model (e.g. `ggml-base.bin`) from HuggingFace or the whisper.cpp releases
5. Place the model in `{AppData}/com.localvoice.app/models/` or set `WHISPER_MODEL_PATH`

Development shortcuts:
- Set `WHISPER_BIN_PATH=/path/to/whisper-cli.exe` to override binary resolution
- Set `WHISPER_MODEL_PATH=/path/to/model.bin` to override model resolution

## Known Limitations / Future Work

- No resampling: if the microphone captured at a rate other than 16 kHz, whisper.cpp
  still attempts transcription (it typically handles this internally, but quality may vary).
- `model_id` parameter in `transcribe_last_recording` is a no-op placeholder until MS-07
  (Models) implements the model registry.
- The pill stays in Success state until the next recording starts. A timeout to auto-reset
  to Idle is planned for MS-10 polish.
- The transcript preview in the pill is truncated to ~40 characters. Clicking the pill
  opens the main window; full history/detail view arrives in MS-05.
