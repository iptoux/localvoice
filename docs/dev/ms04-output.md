# MS-04 — Output Workflow

## What Was Built

After a transcription completes, the cleaned text is now automatically delivered to the user via one of two modes:

- **Clipboard** (default): text is written to the system clipboard ready to paste manually.
- **Auto-insert**: text is written to the clipboard and `Ctrl+V` is immediately simulated in the focused application; the previous clipboard content is restored afterwards.

The pill auto-returns to Idle after 2 s (success) or 3 s (error), allowing the next recording to start without user interaction.

## Key Decisions

- **`arboard` for clipboard**: cross-platform Rust clipboard crate; straightforward `get_text` / `set_text` API.
- **`enigo 0.1` for key simulation**: well-tested, uses OS-native `SendInput` on Windows; simulates `Ctrl+V` via `key_down(Control) + key_click(Layout('v')) + key_up(Control)`.
- **Clipboard-first insert**: `text_insertion::insert` always writes to the clipboard before simulating the keypress. If the paste simulation fails, the text is still accessible via clipboard.
- **Previous-clipboard restore**: after the 150 ms paste wait, the previous clipboard content is restored (best-effort). This preserves the user's workflow.
- **Insert-mode fallback to clipboard**: if `text_insertion::insert` returns an error, `perform_output` writes the text to the clipboard directly and returns a success result with a note in the `error` field.
- **Auto-reset via `tauri::async_runtime::spawn`**: instead of blocking the background thread, the idle reset is scheduled as a lightweight async task.

## Architecture Notes

### New modules

| Module | Purpose |
|--------|---------|
| `src-tauri/src/os/clipboard.rs` | `write(text)` → returns previous content; `restore(previous)` |
| `src-tauri/src/os/text_insertion.rs` | `insert(text)` → clipboard + 50 ms pause + Ctrl+V + 150 ms wait + restore |

### Extended types

- `TranscriptionResult.output: Option<OutputResult>` — carries mode, success, and optional error description.
- `OutputResult` is serialized as camelCase JSON; emitted in both `transcription-completed` and the dedicated `output-result` events.

### Event flow

```
transcribe_and_emit()
  └─ transcribe()             (unchanged)
  └─ perform_output()         (NEW — clipboard or insert)
  └─ emit("output-result")    (NEW — TASK-057)
  └─ result.output = …        (NEW — TASK-058)
  └─ emit("transcription-completed")
  └─ schedule_idle_reset(2 s) (NEW — auto-return)
```

### Settings

| Key | Default | Meaning |
|-----|---------|---------|
| `output.mode` | `"clipboard"` | `"clipboard"` or `"insert"` |

The `output.auto_paste` setting is preserved in the DB for legacy compatibility but the effective toggle is `output.mode` — `"insert"` mode implies auto-paste.

### Frontend

- `app-store`: new `lastOutputResult` / `setLastOutputResult` slice.
- `PillApp`: listens to `output-result` event and stores result.
- `Pill` — Success state: shows an `OutputBadge` ("Copied" / "Inserted" / "Failed") plus a truncated transcript preview. The badge is styled with `bg-white/20` (success) or `bg-rose-900/60` (failed).
- `SettingsPage`: new **Output** section with two radio options (Clipboard / Auto-insert) with explanatory copy.

## Known Limitations / Future Work

- `enigo 0.1` does not expose errors from `SendInput`; silent failures manifest as the text remaining in the clipboard (which is still useful).
- Clipboard restore after auto-paste uses a fixed 150 ms wait — very slow apps may not have pasted in time.
- No per-app overrides for insert mode (deferred to MS-10 polish).
- The `output.auto_paste` DB setting is no longer the canonical toggle; it can be removed in a future cleanup milestone.
