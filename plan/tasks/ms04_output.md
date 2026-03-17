# MS-04 — Output Workflow

**Goal:** Make the transcription immediately useful by copying it to the clipboard and optionally inserting/pasting into the active application.
**Depends on:** MS-03
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-054: Implement `os/clipboard.rs` — write a string to the system clipboard using `arboard` or Tauri's clipboard plugin
- [x] TASK-055: Implement `os/text_insertion.rs` — best-effort auto-paste: write to clipboard then simulate `Ctrl+V` keypress via `enigo` or `windows-rs` SendInput; must restore previous clipboard content after paste attempt
- [x] TASK-056: Implement output step at end of transcription pipeline in `transcription/pipeline.rs` — after cleaned text is ready, read `output.mode` from settings (`clipboard` | `insert`); call appropriate `os/` module; record success/failure
- [x] TASK-057: Emit Tauri event `output-result` with payload `{ mode, success: bool, error?: string }` after output step
- [x] TASK-058: Extend `RecordingState` / result to carry output outcome so pill can show "Copied" vs "Inserted" vs error
- [x] TASK-059: React: Pill Success state — show "Copied" or "Inserted" label based on output mode; auto-return to Idle after ~2s
- [x] TASK-060: React: Pill Error state — show brief error text; auto-return to Idle after ~3s
- [x] TASK-061: React: Expanded view — show latest transcript text, copy-to-clipboard button, output mode badge
- [x] TASK-062: React: Settings — output mode toggle (Clipboard / Auto-insert); `auto_paste` toggle explanation

## Product/UX Tasks

- [ ] TASK-063: Test clipboard output in at least 3 apps (browser, VS Code, Notepad) — confirm text arrives correctly
- [ ] TASK-064: Test insert mode in the same 3 apps — document any that require special handling or fallback

## QA / Acceptance

- [ ] TASK-065: Verify clipboard default output works reliably across apps
- [ ] TASK-066: Verify insert mode pastes into the focused app immediately after transcription
- [ ] TASK-067: Verify pill always returns to Idle after Success or Error, regardless of output mode

---

## Acceptance Criteria

- Successful transcription copies text to clipboard by default
- Optional paste/insert mode can be toggled in settings
- Pill shows Success or Error outcome after output step

---

## Technical Notes

- Clipboard is the default and most reliable cross-app output mode — keep it as the safe fallback
- Treat direct insertion as best-effort; easy to disable via settings
- Restore clipboard contents after auto-paste so the user's prior clipboard is not lost
- On Windows, `SendInput` with `VK_CONTROL + V` is the standard paste simulation approach
- Used `arboard = "3"` for clipboard and `enigo = "0.1"` for key simulation
- Auto-reset to Idle is scheduled via `tauri::async_runtime::spawn` + `tokio::time::sleep`
