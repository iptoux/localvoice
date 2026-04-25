# Changelog

## Unreleased

- Added a plain-text `localvoice.log` file in the app data directory so crash reports can include logs even when the in-app viewer is unavailable.
- Moved audio-level UI event emission out of the real-time capture callback and uses the default Windows input stream configuration to reduce driver-level recording crashes.

## MS-18 — macOS & Linux Support (2026-03-23)

- macOS autostart via launchd plist (`~/Library/LaunchAgents/com.localvoice.app.plist`)
- Linux autostart via XDG `.desktop` file (`~/.config/autostart/localvoice.desktop`)
- macOS auto-insert: simulates `Cmd+V` via `osascript` (requires Accessibility permission)
- Linux auto-insert: detects X11 vs. Wayland at runtime; uses `xdotool` or `wtype`
- `windows-sys` dependency moved to Windows-only in `Cargo.toml`
- DLL resources split into `tauri.windows.conf.json` — macOS/Linux builds no longer fail due to missing `.dll` files
- CI matrix: `cargo test` now runs on `windows-latest`, `macos-latest`, and `ubuntu-latest`
- Release pipeline: new `build-macos` and `build-linux` jobs produce `.dmg` and `.deb`/`.AppImage` artifacts; whisper.cpp is built from source via `cmake`
- `bootstrap.sh` updated: detects macOS vs. Linux, downloads or builds the correct whisper-cli triple
- README updated with platform prerequisites, macOS Accessibility setup, and Linux `xdotool`/`wtype` instructions

## MS-17 — Test Suite, CI/CD & Performance (2026-03-23)

- 91 Rust unit/integration tests across all major backend modules
- 48 frontend component tests (Pill, SettingsPage, Dashboard) via Vitest + @testing-library/react
- GitHub Actions CI pipeline: cargo test + lint + frontend test + build on every PR
- SQLite log persistence: app logs now survive restarts via tokio background channel
- Vite code splitting with `manualChunks` for vendor chunks (recharts, lucide, i18n, UI libs)
- Deferred non-critical Tauri startup work (audio cleanup) to after first render

## MS-15 — Stronger Dashboard & Confidence Visualization (2026-03-18)

- Language breakdown donut chart on Dashboard
- Top correction rules frequency chart
- WPM trend line chart with date range filtering
- Confidence-colored segments in session detail (green/yellow/red)
- Word-level diff view comparing raw vs. cleaned transcript
- New backend stats: language breakdown, correction stats, WPM trend, daily comparison

## MS-14 — Session Reprocessing & Configurable Post-Processing (2026-03-18)

- Reprocess sessions with a different whisper model or language
- Optional audio file retention with configurable cleanup period
- Post-processing pipeline toggles (auto-cap, auto-punct, filler removal, dictionary auto-apply) in Settings
- "Original" tab in session detail drawer to compare pre/post-reprocess text
- Audio cleanup runs on startup, removing files older than retention period

## MS-13 — Theme System & Customizable Shortcuts (2026-03-18)

- **Theme switching**: Light, Dark, and System themes with live preview in Settings → Appearance
- **Theme persistence**: selected theme is saved and restored on app restart; "System" follows OS preference in real-time
- **Semantic colors**: all pages migrated from hardcoded dark colors to CSS-variable-based theme-aware classes
- **Shortcut customization**: new "Change" button in Settings → Recording to record a custom global shortcut; validates and re-registers immediately without restart
- **Enhanced tray menu**: Start/Stop Recording, Dashboard, History, Settings, Quit — with separators for grouping
- **Rust `update_shortcut` command**: validates shortcut format, unregisters old shortcut, registers new one atomically

## MS-12 — Improved Insert Flow (2026-03-18)

- **Direct SendInput**: replaced `enigo 0.1` with `windows-sys` `SendInput` for more reliable Ctrl+V simulation
- **Chunked insertion**: texts longer than 4 000 characters are split into sequential paste chunks to prevent buffer overflow in target apps
- **Configurable insert delay**: new "Insert delay" slider (50–500 ms) in Settings → Output; controls the pause before each Ctrl+V
- **Graceful fallback**: if auto-insert fails, text stays on clipboard with message "Text copied — paste manually"
- **Target app detection**: the foreground window title is detected before insertion and stored in session metadata (`output_target_app`)
- **DB migration 4**: seeds `output.insert_delay_ms` setting (default: 100 ms)

## MS-11 — Expanded Pill View & Animations (2026-03-18)

- **Expanded pill**: single-click the pill to reveal transcript preview, language/model badges, word count, language switcher, start/stop button, and quick actions (Copy, History, Settings)
- **Collapse on blur**: expanded pill automatically collapses when clicking elsewhere or clicking the pill bar again
- **Waveform visualization**: animated canvas-based audio bars replace the static "Listening…" text during recording, responding to real-time audio level
- **Smooth state transitions**: CSS `transition-all` with 300ms easing on the pill container for color and size changes
- **Success-to-idle fade**: after 3 seconds in success state, the pill fades back to idle with an opacity transition instead of an abrupt reset
- **Pill window resize**: Tauri commands `expand_pill` / `collapse_pill` resize the pill window between 300×64 and 300×280

## MS-10 — Polish (2026-03-17)

- **Onboarding**: first-time users with no model installed see a welcome overlay guiding them to the Models page
- **Autostart**: Settings → System toggle registers/removes LocalVoice from Windows startup (registry)
- **Window persistence**: pill position and main window size/position are saved automatically and restored on next launch
- **Native notifications**: errors during transcription emit an OS toast notification with a plain-language message (opt-out); success notifications are opt-in (Settings → Notifications)
- **Friendly error messages**: raw Rust errors are mapped to user-readable text before display in the pill and notifications
- **In-app Logs page**: shows all warn/error entries captured in the current session; filterable by level; exportable as JSON
- **Error boundary**: uncaught React errors show a "Try again" screen instead of a blank window
- **Settings**: autostart toggle, error notification toggle, success notification toggle; `Reset to defaults` button now functional
- **Loading states**: Models page shows loading indicator on initial fetch; shared Spinner component added

## MS-09 — Ambiguity v1 (2026-03-17)

- New **Suggestions** tab in the Dictionary page — shows low-confidence phrases that whisper repeatedly struggles with
- Phrases are automatically tracked after each transcription; they surface once ≥ 3 occurrences with average confidence below 60%
- If a matching correction rule already exists, a suggested replacement is shown instantly
- **Accept**: one click creates a correction rule (mode: suggested, auto-apply: on) and removes the suggestion
- **Edit**: inline input to type a custom replacement before accepting
- **Dismiss**: hides the suggestion; it reappears automatically if whisper continues to struggle (+5 new occurrences)
- DB migration v2 adds `dismissed_at_occurrences` column to `ambiguous_terms` for the re-surface logic

## MS-08 — Dictionary v1 (2026-03-17)

- New **Dictionary** page with Rules tab (default) and Terms tab
- Correction rules replace misheard words automatically during every transcription (case-insensitive, global match)
- Rules can be enabled/disabled individually with a toggle — no deletion required
- Usage counter increments each time a rule fires; rules sorted by usage count
- Language-specific rules (DE / EN) apply alongside universal (all-language) rules
- Term entries (name, acronym, product, custom) available for reference and future features
- Add/Edit/Delete via modal forms; active toggle inline on rule rows

## MS-07 — Models (2026-03-17)

- New **Models** page: browse all available whisper.cpp models with name, size, and language scope
- Download button per model with live progress bar (percent + bytes transferred)
- SHA-256 checksum verification after download — partial/corrupt files are rejected and removed
- Delete installed models from disk with a confirmation prompt
- Separate default model selectors for German (DE) and English (EN)
- Transcription now resolves the model from the DB default for the active language, falling back to the legacy settings path and then auto-scan
- Models stored at `{app_data_dir}/models/` — survives app restarts via DB install record

## MS-06 — Dashboard (2026-03-17)

- Dashboard page now shows live metrics: Total Words, Sessions, Avg WPM, Recording Time
- Words-over-time line chart (daily buckets for 7d/30d, weekly for all time)
- Language breakdown horizontal bar chart with per-language colour coding
- Date range selector: Last 7 days / Last 30 days / All time — updates all metrics and charts instantly
- All aggregations computed via pre-aggregated SQL queries (no in-memory loading of sessions)

## MS-05 — History (2026-03-17)

- All transcription sessions are now persisted to SQLite automatically
- New **History** page: browsable list of past sessions with date, language, word count, and text preview
- Debounced full-text search across transcript content
- Filter by language and date range
- Session detail drawer: cleaned vs raw transcript tabs, optional segment list with timestamps and confidence scores
- Copy cleaned/raw text to clipboard directly from the detail drawer
- Delete individual sessions (with confirmation step)
- Export sessions to plain text or JSON via native save-file dialog
- Pagination (50 sessions per page) with Previous / Next controls

## MS-04 — Output Workflow (2026-03-17)

- Transcription text is automatically copied to the clipboard after every successful recording
- New **Auto-insert** output mode: text is pasted into the focused application via Ctrl+V immediately after transcription; previous clipboard content is restored afterwards
- Output mode toggle added to Settings (Clipboard / Auto-insert)
- Pill Success state now shows a **"Copied"** or **"Inserted"** badge alongside the transcript preview
- Pill auto-returns to Idle after ~2 s (success) or ~3 s (error) — no manual dismiss needed
- `output-result` event emitted after each output step with mode, success flag, and optional error

## MS-03 — Local Transcription (2026-03-17)

- Automatic offline transcription via whisper.cpp CLI sidecar after every recording
- German and English language selection in Settings (all whisper.cpp languages supported)
- Pill shows transcript preview text in Success state
- Transcription errors shown clearly in the pill Error state
- Whitespace normalisation and automatic sentence capitalisation post-processing
- Model and binary paths configurable via Settings, environment variables, or auto-discovery
- `transcribe_last_recording` command allows manual re-transcription from the frontend

## MS-02 — Recording Core (2026-03-17)

- Global shortcut (`Ctrl+Shift+Space` by default) starts and stops recording from anywhere
- Pill transitions through all five states: Idle → Listening → Processing → Success / Error
- Elapsed recording timer shown while listening
- Microphone selector in Settings (reads all available input devices from the OS)
- Audio captured as 16 kHz 16-bit mono WAV for local transcription (MS-03)
- Real-time audio level events emitted to the frontend during recording
- Cancel recording discards audio without leaving temp files

## MS-01 — Foundation & Shell (2026-03-17)

- App launches as a small floating pill by default (300×64, transparent, always-on-top)
- Full main window opens from the pill (double-click) or tray menu
- System tray icon with Open App / Settings / Quit
- SQLite database created automatically in the platform app-data directory on first launch
- Default settings seeded into the database
- Closing any window hides it instead of quitting — the app stays alive in the tray
