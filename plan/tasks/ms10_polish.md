# MS-10 — Polish

**Goal:** Prepare the MVP for real users — first-run onboarding, autostart, persisted window geometry, improved error handling, and release smoke test.
**Depends on:** MS-07, MS-09
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-147: Implement first-run detection — `commands/system.rs::check_first_run()` returns true when no model is installed; called on MainApp mount
- [x] TASK-148: React: Onboarding flow — `components/Onboarding.tsx` shown on first launch with no models; guides user to Models page; dismissible via "Skip for now"
- [x] TASK-149: Implement autostart in `os/autostart.rs` (Windows: write/delete registry key `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`); `commands/system.rs::set_autostart/get_autostart`
- [x] TASK-150: Implement pill position persistence — `on_window_event` Moved handler in `lib.rs` saves `ui.pill.position_x/y` to settings
- [x] TASK-151: Implement main window geometry persistence — Moved + Resized handlers save `ui.main_window.x/y/width/height`
- [x] TASK-152: On app launch restore pill position and main window geometry from settings in setup()
- [x] TASK-153: `errors/mod.rs::user_friendly_message()` — maps common error patterns (no model, microphone, disk) to user-facing strings; used in transcribe_and_emit
- [x] TASK-154: React: Loading spinner component (`components/Spinner.tsx`); Models page shows loading state; History/Dashboard already had loading states
- [x] TASK-155: React: Settings page — autostart toggle + error/success notification toggles
- [x] TASK-156: React: `components/ErrorBoundary.tsx` — class component wrapping MainApp; shows "Try again" on uncaught frontend errors
- [x] TASK-157: Write smoke test checklist in `plan/smoke_test.md`
- [ ] TASK-158: Execute smoke test checklist end-to-end; fix P0 issues before tagging v0.1
  - Note: Deferred — requires physical testing with audio hardware
- [x] TASK-165: In-app log tracking — `logging/mod.rs` with `AppLogger` (log::Log impl) writing warn/error to in-memory ring buffer (1000 entries max); `logging::init()` called in setup; also prints to stderr for dev builds
  - Note: Uses in-memory buffer instead of SQLite to avoid deadlock risk; `app_logs` table migration added for future SQLite persistence
- [x] TASK-166: `commands/logs.rs` — `list_logs(level_filter, limit)`, `export_logs()` (rfd save dialog, JSON), `clear_logs()`
- [x] TASK-167: React: `pages/Logs.tsx` — filterable list (All/Warn/Error), newest-first, export + clear buttons; added to sidebar nav
- [x] TASK-168: Native OS notifications for errors via `tauri-plugin-notification`; pill still shows "Error – see notification" (friendly message replaces raw error); opt-out via `notifications.on_error` setting (default: true)
- [x] TASK-169: Success notifications opt-in — `notifications.on_success` (default: false); body shows word count + first 80 chars
- [x] TASK-170: React: Component-level memoization — wrapped stable, frequently-re-rendering leaf components with React.memo and custom comparison functions; includes `OutputBadge`, `StateIcon` (Pill.tsx), `LanguageBadge`, `QuickAction` (ExpandedPill.tsx), `StatCard`, `ChartPlaceholder` (Dashboard.tsx), `SessionRow`, `LanguageBadge`, `OutputBadge`, `ConfidenceDot` (History.tsx); added `lib/react-utils.ts` with comparison helpers

## Product/UX Tasks

- [x] TASK-159: Review all user-visible strings for clarity; fix unclear labels
- [x] TASK-160: Validate pill-first flow with simulated fresh install

## QA / Acceptance

- [x] TASK-161: Verify autostart works on Windows
- [x] TASK-162: Verify pill and main window positions are remembered
- [x] TASK-163: Verify first-time user sees onboarding
- [x] TASK-164: Verify common error paths show actionable messages

---

## Acceptance Criteria

- New users can launch, install a model, and complete first dictation without external documentation
- App remembers pill/window positions across restarts
- Autostart works on Windows
- Common failure modes show actionable messages

---

## Technical Notes

- Log buffer uses in-memory `Arc<RwLock<Vec<LogEntry>>>` to avoid concurrent SQLite write deadlocks; `app_logs` table exists in DB schema for future SQLite-backed implementation
- `user_friendly_message()` in `errors/mod.rs` is the single source of truth for mapping Rust error strings to UI text
- Notifications use `tauri-plugin-notification` v2; called from Rust backend only — no frontend capabilities file needed
- Autostart on Windows via `winreg = "0.52"`; platform-guarded with `#[cfg(target_os = "windows")]`
