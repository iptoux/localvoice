# MS-19 — Installer, Updater, Signing & Data Management

**Goal:** Prepare for public release with a proper Windows installer (NSIS), auto-update mechanism, code signing, and user data backup/restore/import capabilities.
**Depends on:** MS-18
**Status:** `todo`

---

## Engineering Tasks

### Installer & Updater

- [ ] TASK-258: Configure Tauri NSIS installer — update `tauri.conf.json` bundle targets; configure installer pages (license, install dir, start menu shortcut, desktop shortcut); set `publisher` and `shortDescription`
- [ ] TASK-259: Configure Tauri updater — add `tauri-plugin-updater`; configure update endpoint (GitHub releases JSON); implement update check on app startup (setting `app.auto_update`, default: true); show non-blocking notification when update available
- [ ] TASK-260: React: Update notification banner — shown at top of main window when update is available; "Update Now" button triggers download + install; "Dismiss" hides until next check

### Code Signing

- [ ] TASK-261: Windows code signing — integrate `signtool` into CI release pipeline; configure certificate via environment variable `WINDOWS_CERTIFICATE` in CI secrets; sign .exe and .msi
- [ ] TASK-262: macOS code signing — integrate `codesign` and notarization into CI; configure Apple Developer certificate in CI secrets; sign .app and .dmg

### Data Management

- [ ] TASK-263: Rust: `commands/system.rs::export_all_data()` — exports full SQLite database as a backup file to user-chosen location (via `rfd` save dialog); includes app version in filename
- [ ] TASK-264: Rust: `commands/system.rs::import_data(path)` — imports a backup SQLite file; validates schema version compatibility; merges or replaces current data (user choice); requires app restart after import
- [ ] TASK-265: Rust: `commands/dictionary.rs::export_dictionary(format)` — exports dictionary entries and correction rules as JSON; `import_dictionary(path)` imports from JSON, skipping duplicates
- [ ] TASK-266: React: Settings page — "Data Management" section with "Export Backup", "Import Backup", "Export Dictionary", "Import Dictionary" buttons; confirmation dialogs for destructive operations
- [ ] TASK-267: React: Settings page — "About" section showing app version, build date, license info, link to repository
- [ ] TASK-268: DB migration 6 — add settings: `app.auto_update` (true), `app.last_update_check` (empty)

## QA / Acceptance

- [ ] TASK-268a: Verify Windows installer creates Start Menu and desktop shortcuts
- [ ] TASK-268b: Verify auto-updater detects a mock update endpoint
- [ ] TASK-268c: Verify database backup and restore round-trips correctly
- [ ] TASK-268d: Verify dictionary export/import preserves all entries and rules

---

## Acceptance Criteria

- Windows installer creates proper Start Menu and desktop shortcuts
- Auto-updater detects and installs new versions from GitHub releases
- Windows and macOS binaries are signed (no "unknown publisher" warnings)
- Users can backup and restore their entire database
- Dictionary can be exported and imported as JSON

---

## Technical Notes

- Tauri's built-in `tauri-plugin-updater` uses GitHub releases as the update endpoint — simple and free for open-source
- Code signing requires purchasing certificates (Windows: code signing cert, macOS: Apple Developer account)
- Database import validates `schema_migrations` table to ensure version compatibility before proceeding
- Dictionary import uses JSON format for portability; skips entries where `phrase` + `language` already exists
