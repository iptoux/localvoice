# MS-03 — Local Transcription

**Goal:** Deliver end-to-end local transcription via whisper.cpp sidecar, with German and English language selection and parsed transcript output.
**Depends on:** MS-02
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-037: Obtain whisper.cpp CLI binary for Windows (x86_64); add to `src-tauri/binaries/` as a Tauri sidecar; configure `tauri.conf.json` sidecar allowlist entry
- [ ] TASK-038: Implement `transcription/whisper_sidecar.rs` — spawn whisper.cpp sidecar via `tauri::api::process::Command`; pass args: `--model <path>`, `--language <lang>`, `--output-json` or parse stdout; capture stdout/stderr; return raw output string
- [ ] TASK-039: Implement `transcription/parser.rs` — parse whisper.cpp output into `Vec<TranscriptSegment { start_ms, end_ms, text, confidence? }>` and a joined `full_text` string
- [ ] TASK-040: Implement `transcription/language.rs` — map `"de"` → `"german"`, `"en"` → `"english"` for whisper CLI flag
- [ ] TASK-041: Implement `transcription/orchestrator.rs` — coordinate: resolve model path from `AppState`/settings, invoke sidecar, parse output, run post-processing pipeline, return `TranscriptionResult`
- [ ] TASK-042: Implement `transcription/pipeline.rs` — stub pipeline that runs `postprocess/normalize.rs` (whitespace trim only at this stage)
- [ ] TASK-043: Implement `postprocess/normalize.rs` — trim whitespace, collapse multiple spaces, basic sentence-start capitalization
- [ ] TASK-044: Implement `commands/transcription.rs` — Tauri command `transcribe_last_recording(language: String, model_id: Option<String>)` which calls the orchestrator and returns `TranscriptionResult`
- [ ] TASK-045: Wire transcription to be triggered automatically after `stop_recording()` — recording transitions to Processing, calls `transcribe_last_recording`, then to Success/Error
- [ ] TASK-046: React: expanded pill view (click on pill) shows latest cleaned transcript text
- [ ] TASK-047: React: language selector (de/en) persisted to settings, passed to transcription command
- [ ] TASK-048: React: display transcription error clearly in pill Error state

## Product/UX Tasks

- [ ] TASK-049: Validate the end-to-end latency on typical hardware (record 5s clip, measure time to Success state); document baseline

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
- If whisper.cpp supports `--output-json`, prefer that over stdout parsing for reliability
- Keep the model path configurable from settings so MS-07 (Models) can plug in without rework
