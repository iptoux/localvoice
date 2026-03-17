# MS-10 — Polish

**Goal:** Prepare the MVP for real users — first-run onboarding, autostart, persisted window geometry, improved error handling, and release smoke test.
**Depends on:** MS-07, MS-09
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-147: Implement first-run detection in `main.rs` — on launch check if any `model_installations` row has `installed=1`; if not, emit event or set app state flag to trigger onboarding
- [ ] TASK-148: React: Onboarding flow — shown once on first launch with no models; guides user to Models page to download a model; dismissible after model is installed
- [ ] TASK-149: Implement autostart in `os/autostart.rs` (Windows: write/delete registry key `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`); expose via `commands/system.rs` as `set_autostart(enabled: bool)` and `get_autostart() -> bool`
- [ ] TASK-150: Implement pill position persistence — on pill window `moved` event, call `update_setting("ui.pill.position_x", x)` and `update_setting("ui.pill.position_y", y)` (debounced, ~500ms)
- [ ] TASK-151: Implement main window geometry persistence — on `resize` and `moved`, save `ui.main_window.width/height/x/y` to settings (debounced)
- [ ] TASK-152: On app launch restore pill position and main window geometry from settings before showing windows
- [ ] TASK-153: Implement `errors/mod.rs` — map common Rust error variants to user-facing strings (e.g. `ModelNotFound` → "No model installed. Open Models to download one.", `AudioDeviceError` → "Microphone not accessible. Check your audio settings.")
- [ ] TASK-154: React: Add loading spinner / skeleton states to all async data fetches (history list, dashboard, models, dictionary)
- [ ] TASK-155: React: Settings page — autostart toggle (calls `set_autostart`); reads current state on mount
- [ ] TASK-156: React: Global error boundary — catch unexpected frontend errors and show a non-crashing error message
- [ ] TASK-157: Write smoke test checklist in `plan/smoke_test.md` covering: launch → onboarding → download model → record clip → transcribe → view in history → dashboard updates → add correction rule → verify rule fires → check pill positions remembered after restart
- [ ] TASK-158: Execute smoke test checklist end-to-end; open issues for any blocking failures; fix all P0 issues before tagging v0.1
- [ ] TASK-165: Implement in-app log tracking — `db/repositories/logs_repo.rs` inserts rows into `app_logs` table (id, level, area, message, created_at); add migration; implement a custom `log::Log` subscriber in `logging/mod.rs` that writes warn/error entries to SQLite; register in `lib.rs` setup
- [ ] TASK-166: Implement `commands/logs.rs` — Tauri commands: `list_logs(level_filter, limit)` returning `Vec<LogEntry>`; `export_logs()` opens a save-file dialog (rfd) and writes JSON or plain text; `clear_logs()` truncates the table
- [ ] TASK-167: React: Logs page — filterable list (All / Warn / Error) sorted newest-first; each row shows timestamp, level badge, area tag, and message; "Export" button and "Clear" button; add "Logs" entry to sidebar nav
- [ ] TASK-168: Replace pill in-line error text with `tauri-plugin-notification` native OS toast — on Error state emit a native notification with title "LocalVoice" and the actionable error message; keep the pill Error state for visual feedback but shorten the displayed text to "Error – see notification"
- [ ] TASK-169: Emit native OS notification on transcription Success too (opt-in setting `notifications.on_success`, default false) — body shows word count and first ~80 chars of cleaned text

## Product/UX Tasks

- [ ] TASK-159: Review all user-visible strings for clarity (German and English); fix any unclear labels or missing translations
- [ ] TASK-160: Validate pill-first flow with a simulated fresh install (delete DB and models, relaunch); confirm onboarding is clear

## QA / Acceptance

- [ ] TASK-161: Verify autostart works on Windows — app appears in startup programs; can be toggled off
- [ ] TASK-162: Verify pill and main window positions are remembered across full app restarts
- [ ] TASK-163: Verify first-time user with no models sees onboarding and can complete first dictation without external help
- [ ] TASK-164: Verify all common error paths (missing model, mic unavailable, disk full on temp audio) show actionable messages instead of crashes

---

## Acceptance Criteria

- New users can launch, install a model, and complete first dictation without external documentation
- App remembers pill/window positions across restarts
- Autostart works on Windows
- Common failure modes show actionable messages

---

## Technical Notes

- Use a release checklist (smoke_test.md) to keep polish work bounded and verifiable
- Debounce window position saving to avoid excessive SQLite writes during dragging
- Autostart on Windows via registry is the most reliable method; document the key path for future macOS/Linux support
- Platform-specific quirks for autostart, tray, and text insertion should be documented in `plan/` for future platform ports
- Log tracking: only persist warn/error/fatal — debug-level logs stay ephemeral to avoid DB bloat; cap the table at a rolling 1000 rows (delete oldest on insert when over limit)
- Native notifications: use `tauri-plugin-notification`; requires `"notification"` capability in `tauri.conf.json`; on Windows shows Windows toast notifications
- Logs export format: JSON preferred (structured, machine-readable for support triage); plain text as secondary option
