# MS-12 — Improved Insert Flow

**Goal:** Replace the clipboard+paste hack with a more robust text insertion mechanism with configurable delays, chunking, fallback, and target app detection.
**Depends on:** MS-10a
**Status:** `todo`

> **Note:** Basic silence detection and push-to-talk are handled in MS-10a (settings fixes). This milestone focuses on the insert/output pipeline.

---

## Engineering Tasks

- [ ] TASK-181: Research and upgrade text insertion — replace `enigo 0.1` with `enigo 0.2+` or direct `windows-sys` `SendInput` calls in `os/text_insertion.rs`; measure reliability vs. current approach
- [ ] TASK-182: Implement chunked text insertion in `os/text_insertion.rs` — for texts longer than 4000 chars, split into clipboard chunks with sequential paste to avoid target app buffer overflow
- [ ] TASK-183: Add configurable insert delay setting `output.insert_delay_ms` (default: 100) in migration 4; expose in settings; used in `text_insertion::insert()` between clipboard write and Ctrl+V
- [ ] TASK-184: Fallback logic in `os/text_insertion.rs::insert()` — if paste simulation fails, keep text on clipboard and emit `output-result` with `mode: "clipboard"` and a user-facing message "Text copied — paste manually"
- [ ] TASK-185: Detect active window title before insertion via `GetForegroundWindow` + `GetWindowText` (Windows API) in `os/text_insertion.rs`; store in session's `output_target_app` field
- [ ] TASK-186: React: Settings page — add "Insert delay" slider (50ms–500ms) in Output section; maps to `output.insert_delay_ms` setting
- [ ] TASK-187: DB migration 4 — add settings: `output.insert_delay_ms` (100)

## QA / Acceptance

- [ ] TASK-187a: Verify text insertion in Notepad, VS Code, Chrome text input, and Word
- [ ] TASK-187b: Verify insert delay prevents text loss in slow apps
- [ ] TASK-187c: Verify fallback to clipboard when insert fails
- [ ] TASK-187d: Verify `output_target_app` is stored in session after insertion

---

## Acceptance Criteria

- Text insertion works reliably in common apps (Notepad, VS Code, browser text fields)
- Insert delay is configurable and prevents text from being lost
- Failed insertion gracefully falls back to clipboard with clear user feedback
- Target app name is recorded in session metadata

---

## Technical Notes

- `enigo 0.1` is outdated; `enigo 0.2+` or direct `windows-sys` `SendInput` gives more control over timing
- The clipboard+paste approach remains fundamental — true direct text insertion is unreliable across all Windows apps
- Chunked insertion is needed for long dictations where clipboard limits may apply in certain apps
- `GetForegroundWindow` + `GetWindowText` are reliable on Windows; requires `windows-sys` or `winapi` crate
