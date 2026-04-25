# MS-10 — Polish

## What Was Built

First-run onboarding, autostart (Windows registry), window position persistence, user-friendly error messages, native OS notifications, in-app log viewer, and a React error boundary.

## Key Decisions

- **Plain-text crash log plus SQLite log list**: The logger writes each enabled entry immediately to `{app_data_dir}/localvoice.log` for support cases where the app crashes before the UI can export logs. It also queues entries to the SQLite-backed `app_logs` table for the in-app Logs page.
- **`user_friendly_message()` in `errors/mod.rs`**: Single function that maps raw Rust error strings to short user-facing messages. Used in `transcribe_and_emit` before emitting error state and before the OS notification body.
- **Notifications from Rust only**: `tauri-plugin-notification` is called exclusively from the orchestrator. No frontend capabilities file is needed — capabilities are only required when the JS side invokes plugin commands.
- **`log::set_max_level(Info)`**: Lets info-level log calls reach stderr (for dev builds) while only buffering warn/error entries. The existing `log::*` calls throughout the codebase produce visible output during `cargo tauri dev` without any additional tooling.
- **Window position on `Moved` event**: Writes are direct SQLite upserts (no debouncing). The pill window rarely moves; the overhead is negligible.

## Architecture Notes

```
logging/mod.rs                  — AppLogger + LogEntry + init() + localvoice.log writer
os/autostart.rs                 — Windows registry autostart (cfg-guarded)
commands/system.rs              — check_first_run, set_autostart, get_autostart
commands/logs.rs                — list_logs, export_logs, clear_logs
errors/mod.rs                   — user_friendly_message()
lib.rs                          — init logging, restore positions, window events, register commands
transcription/orchestrator.rs   — uses user_friendly_message + sends notifications
src/components/ErrorBoundary.tsx
src/components/Onboarding.tsx
src/components/Spinner.tsx
src/pages/Logs.tsx
src/pages/SettingsPage.tsx      — autostart + notification toggles added
src/components/layout/Sidebar.tsx — Logs link added
```

## Known Limitations / Future Work

- TASK-158 (physical smoke test) deferred — needs real audio hardware
- TASK-159/160 (UX string review, fresh-install validation) deferred
- Native process aborts can still stop the app before a final log line is written, but previous entries are flushed to `localvoice.log` synchronously.
- Autostart on macOS/Linux: `os/autostart.rs` is a no-op outside Windows; platform-specific implementations to be added when porting
- `reset_settings` re-seeds defaults but does not restore pill/main window position settings; window positions persist independently
