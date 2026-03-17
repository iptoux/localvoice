# Changelog

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
