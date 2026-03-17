# MS-01 — Foundation & Shell

## What Was Built

The full technical scaffold for LocalVoice:

- Tauri v2 project bootstrapped with React + TypeScript + Vite
- Two-window setup: a small floating pill (300×64, transparent, always-on-top, no decorations) and a hidden full main window (1100×720)
- SQLite database opened in the platform app-data directory on first launch; all tables created via a versioned migration runner
- Shared `AppState` registered via `tauri::Builder::manage()` — holds the DB connection and recording-state placeholders
- Tauri commands: `get_settings`, `update_setting`, `reset_settings`, `show_pill`, `hide_pill`, `open_main_window`, `set_pill_position`
- System tray with Open App / Settings / Quit; left-click toggles pill visibility
- React frontend: window-label detection routes to `PillApp` or `MainApp` from a single `main.tsx`
- Pill component with drag region and state colour coding
- App shell with sidebar navigation (Dashboard, History, Dictionary, Models, Settings — all placeholder pages)
- Zustand store stubs: `app-store` (recording state), `settings-store` (load/update from DB)
- Typed `invoke()` wrappers in `src/lib/tauri.ts`

## Key Decisions

- **`rusqlite` (bundled) over `sqlx`** — Avoids async complexity and a separate SQLite system library. The bundled feature compiles SQLite directly into the binary; no external dependency at runtime.
- **Versioned migration table (`schema_migrations`)** — Simple integer-version approach: query `MAX(version)`, run all entries with a higher version number. No external migration framework needed for the current scope.
- **Single `index.html`, window detection at runtime** — Both windows share one Vite build. `getCurrentWindow().label` decides whether to mount `PillApp` or `MainApp`. Avoids maintaining two separate HTML entry points.
- **Close → hide, not quit** — The `on_window_event` handler intercepts `CloseRequested` and hides the window instead. The app stays alive in the tray.
- **`data-tauri-drag-region` on every child element** — Required on Windows because child elements can intercept `mousedown` before it bubbles to the drag-region parent. See [Bug fix below](#known-bugs-fixed).

## Architecture Notes

```
src-tauri/src/
  lib.rs                ← setup hook (DB → AppState → tray), command registration
  db/
    mod.rs              ← open() — WAL mode, FK enforcement, returns Arc<Mutex<Connection>>
    migrations.rs       ← MIGRATIONS static array, run() checks schema_migrations table
    repositories/
      settings_repo.rs  ← get_all(), upsert()
  state/
    app_state.rs        ← AppState { db, active_session_id, is_recording }
  commands/
    settings.rs         ← get_settings, update_setting, reset_settings
    window.rs           ← show_pill, hide_pill, open_main_window, set_pill_position
  os/
    tray.rs             ← TrayIconBuilder, menu event handler, left-click toggle
  errors/
    mod.rs              ← AppError wrapper, CmdResult<T> alias

src/
  main.tsx              ← getCurrentWindow().label → PillApp | MainApp
  PillApp.tsx           ← pill entry point
  MainApp.tsx           ← BrowserRouter + Sidebar + page routes
  components/pill/Pill.tsx
  components/layout/Sidebar.tsx
  stores/app-store.ts
  stores/settings-store.ts
  lib/tauri.ts          ← typed invoke() wrappers
  types/index.ts
```

## Known Bugs Fixed

### `data-tauri-drag-region` not working on Windows

Two fixes required — documented in detail in [plan/learnings/tauri-drag-region-windows.md](../../plan/learnings/tauri-drag-region-windows.md):

1. `core:window:allow-start-dragging` must be added to `capabilities/default.json` — `core:default` does not include it.
2. `data-tauri-drag-region` must be placed on every visible child element inside the drag area, not only the outer container.

### Scrollbar visible on transparent pill window

The Webview shows a native scrollbar whenever any content overflows by even 1 px (subpixel rendering, rounding). Fix: `overflow: hidden` on both `html` and `body`, plus on the `#root` div.

See [plan/learnings/tauri-pill-scrollbar.md](../../plan/learnings/tauri-pill-scrollbar.md).

## Known Limitations / Future Work

- `active_session_id` and `is_recording` in `AppState` are placeholders — wired up in MS-02
- `reset_settings` is a no-op stub — full seed-reset implemented in MS-10
- Pill position is not persisted between launches — MS-10
- No window state persistence (size/position of main window) — MS-10
- Placeholder pages have no real content — filled in MS-05 through MS-09
