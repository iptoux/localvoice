# MS-21 - Streaming Transcription

**Goal:** Add true low-latency streaming transcription for streaming-capable Parakeet/NeMo models while preserving the existing stop-to-transcribe workflow for Whisper and non-streaming models.
**Depends on:** MS-19 hybrid transcription runtime packaging
**Status:** `done`

---

## Engineering Tasks

### Product Behavior

- [x] TASK-310: Add streaming mode semantics - preview-only streaming is the default; live insert is optional and inserts finalized deltas only.
- [x] TASK-311: Add or extend settings for `transcription.streaming.enabled`, `transcription.streaming.chunk_ms`, and `transcription.streaming.output_mode`.
- [x] TASK-312: Update Settings UI under Transcription with a streaming toggle, chunk size selector, and output mode selector.

### Frontend Events And UI

- [x] TASK-313: Add `transcription-stream-update` event types and frontend store state for live streaming text.
- [x] TASK-314: Show live streaming text in the pill and expanded pill while recording.
  - Superseded by TASK-332 for the default overlay mode; live text remains Classic-pill-only.
- [x] TASK-315: Reset streaming UI state on new recording, cancel, error, and final `transcription-completed`.

### Backend Streaming Pipeline

- [x] TASK-316: Add a `StreamingSessionManager` to Rust app state for lifecycle, cancellation, and worker ownership.
- [x] TASK-317: Add a chunk pump that reads newly captured 16 kHz PCM from `ActiveRecording.samples` every configured chunk interval.
- [x] TASK-318: Resolve streaming eligibility at recording start: streaming enabled, selected model installed, and `supportsStreaming=true`.
- [x] TASK-319: On stop, finalize the streaming session and use the final streamed transcript; fall back to existing WAV transcription if streaming fails or returns empty text.
- [x] TASK-320: On cancel/error, cancel the streaming session and clean up worker processes.

### Runtime Workers

- [x] TASK-321: Add a LocalVoice Parakeet streaming worker sidecar around the pinned `mudler/parakeet.cpp` `v0.3.2` streaming C API.
- [x] TASK-322: Define and implement NDJSON worker protocol messages: `health`, `load`, `audio`, `partial`, `final`, `finalize`, `cancel`, and `error`.
- [x] TASK-323: Extend the optional NeMo worker to the same streaming protocol where the installed NeMo runtime supports it.
- [x] TASK-324: Update CI/release setup and artifact audits to include the Parakeet streaming worker sidecar.

### Live Insert

- [x] TASK-325: Implement live insert for finalized streaming deltas only, reusing the existing OS insertion path and graceful clipboard fallback.

### Model List Fix

- [x] TASK-326: Extract model sorting/filtering into a testable helper and fix `Most accurate first` ordering so Parakeet and NeMo models are included and ranked correctly.
- [x] TASK-327: Add deterministic tie-breakers for accuracy sorting: accuracy, engine priority, artifact precision, size, display name.

### Tests And Documentation

- [x] TASK-328: Add Rust tests for streaming eligibility, session lifecycle, fallback behavior, cancellation, and finalized-delta live insert.
- [x] TASK-329: Add frontend tests for streaming settings, stream update events, pill preview state, and model accuracy sorting.
- [x] TASK-330: Add worker/protocol tests with mock Parakeet and NeMo workers.
- [x] TASK-331: Update README, user docs, dev docs, changelog, and release notes for streaming behavior and limitations.
- [x] TASK-332: Add selectable Pill modes: default recording overlay with waveform-only UI, plus Classic pill preserving persistent/expanded preview behavior.

---

## Acceptance Criteria

- Selecting a streaming-capable Parakeet model and enabling streaming shows transcript text before recording stops in Classic pill mode, or emits streaming data without overlay text in Recording overlay mode.
- Preview-only mode never writes partial text into the target application.
- Live insert mode writes only finalized deltas into the focused text field.
- Stopping the recording finalizes the stream, persists history, runs post-processing, applies dictionary rules, and preserves existing output semantics.
- Whisper and non-streaming models continue to use the existing stop-to-transcribe flow.
- `Most accurate first` includes Parakeet and NeMo models and ranks them above lower-accuracy Whisper models.
- CI covers Rust, TypeScript, protocol, and sorting tests; release audits include all required streaming sidecars/resources.

---

## Implementation Notes

- Do not implement fake streaming by repeatedly running full file transcription on chunks.
- Keep heavy transcription work out of the CPAL capture callback; chunking must run in a background task.
- Base installers may bundle small sidecars/workers, but must not bundle model weights, Python, NeMo, CUDA, or Vulkan runtime packs.
- If upstream Parakeet CLI streaming is not suitable for stdin PCM, use a LocalVoice-owned worker around the Parakeet C API.
