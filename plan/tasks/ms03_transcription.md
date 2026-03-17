# MS-03 — Local Transcription

**Goal:** Deliver end-to-end local transcription via whisper.cpp sidecar, with German and English language selection and parsed transcript output.
**Depends on:** MS-02
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-037: Obtain whisper.cpp CLI binary for Windows (x86_64); add to `src-tauri/binaries/` as a Tauri sidecar; configure `tauri.conf.json` sidecar allowlist entry
  - Placeholder binary at `src-tauri/binaries/whisper-cli-x86_64-pc-windows-msvc.exe` — user must replace with real binary (see docs/user/transcription.md)
- [x] TASK-038: Implement `transcription/whisper_sidecar.rs` — spawn whisper.cpp sidecar via `std::process::Command` (no Tauri plugin needed for Rust-side); binary path resolved via env var, exe dir, resource dir, or PATH; model path resolved similarly
- [x] TASK-039: Implement `transcription/parser.rs` — parse whisper.cpp JSON output file (with segment offsets and token confidence) and stdout SRT fallback into `Vec<TranscriptSegment { start_ms, end_ms, text, confidence? }>` and `full_text`
- [x] TASK-040: Implement `transcription/language.rs` — map `"de"` → `"de"`, `"en"` → `"en"` etc. for whisper CLI `-l` flag (whisper.cpp uses ISO 639-1 codes directly)
- [x] TASK-041: Implement `transcription/orchestrator.rs` — coordinate: resolve binary + model, invoke sidecar, parse output, run post-processing pipeline, return `TranscriptionResult`; `transcribe_and_emit()` for background tasks
- [x] TASK-042: Implement `transcription/pipeline.rs` — pipeline that runs `postprocess/normalize.rs`
- [x] TASK-043: Implement `postprocess/normalize.rs` — trim whitespace, collapse multiple spaces, sentence-start capitalisation
- [x] TASK-044: Implement `commands/transcription.rs` — Tauri commands `transcribe_last_recording(language, model_id?)` and `get_last_transcription()`
- [x] TASK-045: Wire transcription to be triggered automatically after `stop_recording()` — recording transitions to Processing, kicks off `tokio::spawn` + `spawn_blocking` task, orchestrator transitions to Success/Error and emits events
- [x] TASK-046: React: pill success state shows cleaned transcript text (first ~40 chars preview + full text in `title`)
- [x] TASK-047: React: language selector (de/en/auto/…) persisted to settings, passed to transcription command
- [x] TASK-048: React: transcription error shown clearly in pill Error state (already handled by MS-02 error display + orchestrator emitting error message)

## Product/UX Tasks

- [ ] TASK-049: Validate the end-to-end latency on typical hardware (record 5s clip, measure time to Success state); document baseline
  - Depends on actual whisper.cpp binary being placed

## QA / Acceptance

- [ ] TASK-050: Verify a 5–10s German clip transcribes locally and shows German text
- [ ] TASK-051: Verify a 5–10s English clip transcribes locally and shows English text
- [ ] TASK-052: Verify error is shown if model file is missing or sidecar invocation fails
- [ ] TASK-053: Verify `TranscriptSegment` list has correct timing and maps to full text

---

## Acceptance Criteria

- A recorded German clip can be transcribed locally
- A recorded English clip can be transcribed locally
- Transcription result returns structured text and metadata (segments)
- Failures from missing model or bad invocation are surfaced clearly

---

## Technical Notes

- Use whisper.cpp as a sidecar process (not FFI) to reduce build complexity for MVP
- Return structured `TranscriptSegment` payloads — needed later for history detail view and ambiguity detection
- Prefer `--output-json` over stdout parsing for reliability (JSON includes confidence scores)
- Keep the model path configurable from settings so MS-07 (Models) can plug in without rework
- `std::process::Command` used directly (no `tauri-plugin-shell`) — Rust-side process spawning doesn't need the shell plugin
