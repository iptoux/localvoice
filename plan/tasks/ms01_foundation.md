# MS-01 — Foundation & Shell

**Goal:** Establish the technical foundation — Tauri v2 shell, React/TypeScript frontend, Rust command bridge, SQLite bootstrap, tray, and default pill window.
**Depends on:** —
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-001: Initialize Tauri v2 project with `create-tauri-app` (React + TypeScript + Vite template)
- [x] TASK-002: Set up folder structure per PRD §19 — `src/{components,features,stores,pages,lib,types}` and `src-tauri/src/{commands,audio,transcription,postprocess,dictionary,history,stats,models,os,db,settings,state,errors}`
- [x] TASK-003: Configure pill window in `tauri.conf.json` — size ~300×64, no decorations, transparent, always-on-top, skip taskbar
- [x] TASK-004: Configure main app window in `tauri.conf.json` — hidden by default, 1100×720, with decorations
- [x] TASK-005: Add SQLite dependencies to `Cargo.toml` (`sqlx` with `sqlite` + `runtime-tokio` features, or `rusqlite`)
- [x] TASK-006: Implement `db/mod.rs` — open/create SQLite DB file in platform app-data dir on first launch; return shared pool/connection wrapped in `Arc<Mutex<_>>`
- [x] TASK-007: Write initial migration SQL covering all PRD §9 tables: `sessions`, `session_segments`, `dictionary_entries`, `correction_rules`, `ambiguous_terms`, `model_installations`, `settings`
- [x] TASK-008: Implement `db/migrations.rs` — run pending migrations on startup using versioned SQL or embedded strings
- [x] TASK-009: Implement `state/app_state.rs` — `AppState` struct (db pool, recording handle placeholder, active session id), register with `tauri::Builder::manage()`
- [x] TASK-010: Implement `commands/settings.rs` — `get_settings()` returns all rows from `settings` table; `update_setting(key, value)` upserts a row
- [x] TASK-011: Implement `commands/window.rs` — `show_pill()`, `hide_pill()`, `open_main_window()`, `set_pill_position(x, y)`
- [x] TASK-012: Implement system tray in `main.rs` / `os/tray.rs` — menu items: Open App, Settings, Quit; clicking tray icon shows pill
- [x] TASK-013: React: scaffold Pill component — static, idle state, draggable via Tauri drag-region; click opens main window
- [x] TASK-014: React: scaffold app shell — sidebar with links for Dashboard, History, Dictionary, Models, Settings (all placeholder pages)
- [x] TASK-015: React: configure Zustand store stubs (`stores/app-store.ts`, `stores/settings-store.ts`)

## Product/UX Tasks

- [x] TASK-016: Validate pill size and position on Windows — confirm it sits above the taskbar and is visible without obscuring typical workflows

## QA / Acceptance

- [x] TASK-017: Verify app launches to pill by default; DB file is created in app-data dir; migrations run without errors
- [x] TASK-018: Verify main window opens from tray and from pill click; closing main window does not quit the app
- [x] TASK-019: Verify pill is draggable and remembers no position yet (will persist in MS-10)

---

## Acceptance Criteria

- App launches successfully into pill mode by default
- Pill is draggable and re-openable after hiding
- Full app window can be opened from tray and pill
- Database file is created and migrations run without manual steps
- No placeholder crashes during window lifecycle operations

---

## Technical Notes

- Prefer a single source of truth for window state so the pill and full app cannot drift
- Keep routing and feature folders in place even if views are still placeholders
- Use `tauri::WebviewWindowBuilder` for programmatic window creation if needed
