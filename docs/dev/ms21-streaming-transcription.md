# MS-21 - Streaming Transcription

## What Was Built

LocalVoice now has a streaming transcription path for streaming-capable models. Recording still captures 16 kHz mono PCM into `ActiveRecording.samples`; a background `StreamingSessionManager` reads newly captured samples at the configured interval, feeds a warm worker process, emits `transcription-stream-update` events, and finalizes the stream on stop.

Whisper and non-streaming models keep the existing stop-to-transcribe behavior. If a streaming worker is unavailable, fails, or returns empty text, stop falls back to the normal WAV transcription pipeline.

## Key Decisions

- **Preview first:** `transcription.streaming.output_mode=preview` is the default. It updates the pill UI only and never writes partial text into another app.
- **Live insert is opt-in:** `live_insert` writes finalized streaming deltas only. The final cleaned transcript is still persisted and copied according to the normal output path to avoid duplicate full-text insertion.
- **Worker-owned Parakeet streaming:** `parakeet-cli --stream` streams from a WAV file, so LocalVoice uses a dedicated `parakeet-stream-worker` sidecar around the pinned `mudler/parakeet.cpp` C streaming API.
- **Release-safe packaging:** the base installer bundles small sidecars only: `whisper-cli`, `parakeet-cli`, and `parakeet-stream-worker`. Model weights, Python, NeMo, CUDA, and Vulkan packs stay outside the installer.

## Architecture Notes

- `src-tauri/src/transcription/streaming.rs` owns eligibility, chunk pumping, NDJSON worker communication, cancellation, finalization, and live insert deltas.
- `src-tauri/src/transcription/parakeet_stream_worker.rs` resolves the Parakeet streaming sidecar from environment overrides, app directories, Tauri resources, development binaries, or `PATH`.
- `src-tauri/sidecars/parakeet-stream-worker/` contains the C++ worker source built by `.github/actions/setup-parakeet-cpp`.
- The frontend listens for `transcription-stream-update`, stores the current preview in `app-store`, and shows it in the compact and expanded pill while recording.
- Model sorting lives in `src/lib/model-sort.ts` so the Models page and tests use the same deterministic accuracy ranking.

## Known Limitations / Future Work

- NeMo uses the same message names but returns an explicit unsupported streaming error until a compatible warm NeMo streaming API is available in the configured Python runtime.
- The Parakeet streaming worker is built in CI from the pinned upstream source. Local development needs the target-triple worker binary in `src-tauri/binaries/` or `PARAKEET_STREAM_WORKER_PATH`.
- Live insert cannot retroactively rewrite text after final dictionary/post-processing changes; the cleaned final transcript remains available in history and clipboard output.
