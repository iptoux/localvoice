# Changelog

## MS-01 — Foundation & Shell (2026-03-17)

- App launches as a small floating pill by default (300×64, transparent, always-on-top)
- Full main window opens from the pill (double-click) or tray menu
- System tray icon with Open App / Settings / Quit
- SQLite database created automatically in the platform app-data directory on first launch
- Default settings seeded into the database
- Closing any window hides it instead of quitting — the app stays alive in the tray
