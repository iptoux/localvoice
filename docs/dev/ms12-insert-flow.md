# MS-12 — Improved Insert Flow

## What Was Built

Replaced `enigo 0.1` with direct `windows-sys` `SendInput` calls for more reliable keyboard simulation. Added chunked text insertion for long texts, configurable insert delay, graceful clipboard fallback, and foreground window detection.

## Key Decisions

- **`windows-sys` over `enigo 0.2+`** — direct `SendInput` gives full control over key event sequencing (4-event approach: Ctrl down → V down → V up → Ctrl up) without the overhead and API churn of enigo. One fewer dependency to maintain.
- **4 000-char chunk boundary** — empirically safe for all tested targets (Notepad, VS Code, Chrome, Word). Each chunk gets its own clipboard write + Ctrl+V cycle with the configured delay.
- **Foreground window detection in orchestrator, not in `text_insertion`** — the detection (`GetForegroundWindow` + `GetWindowTextW`) runs once before output, and the title is stored in the session. Keeps `text_insertion` a pure "paste text" utility.
- **User-facing fallback message** — when insert fails the `OutputResult` says "Text copied — paste manually" rather than exposing the raw error.

## Architecture Notes

- `os/text_insertion.rs` now accepts `insert_delay_ms` parameter — the orchestrator reads it from `output.insert_delay_ms` setting (default: 100 ms).
- `os/foreground_window.rs` is a new module with `get_foreground_window_title()` — returns `Option<String>`, platform-gated (`#[cfg(target_os = "windows")]`).
- `os/clipboard.rs` gains `read_previous()` — reads without writing, so `text_insertion::insert()` can save the previous clipboard once before chunked pastes.
- DB migration 4 seeds `output.insert_delay_ms` = 100.
- Settings page shows the insert delay slider (50–500 ms) only when output mode is "insert".

## Known Limitations / Future Work

- Non-Windows platforms get a no-op `send_ctrl_v()` and `get_foreground_window_title()` — macOS/Linux implementations deferred to MS-18.
- Chunk boundary splits on char count, not word boundary — could split mid-word for very long continuous text without spaces.
