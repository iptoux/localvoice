# MS-13 — Theme System & Customizable Shortcuts

## What Was Built

Full light/dark/system theme switching for the main window and pill, with persistence across restarts. Customizable global recording shortcut with live re-registration. Enhanced tray menu with recording toggle and navigation items.

## Key Decisions

- **CSS variables via Tailwind v4 `@custom-variant`** — the codebase already had `@custom-variant dark (&:is(.dark *))` and light/dark CSS variable blocks in `index.css`. Theme switching toggles the `.dark` class on `<html>`, which swaps all `--background`, `--foreground`, `--card`, etc. variables. No Tailwind config changes needed.
- **Semantic color classes** — replaced ~150 hardcoded `bg-neutral-*` / `text-neutral-*` classes across all pages with semantic equivalents (`bg-background`, `bg-card`, `text-foreground`, `text-muted-foreground`, `border-border`). This makes all pages automatically theme-aware.
- **Pill keeps state-based colors** — the pill uses its own color scheme (red for listening, amber for processing, green for success) that works regardless of theme. No pill-specific theme changes were needed.
- **Shortcut recorder in-page** — built a `ShortcutRecorder` component that captures keyboard events, shows a preview, and lets the user confirm or cancel. The Rust `update_shortcut` command validates the format, unregisters all shortcuts, and registers the new one atomically.
- **TASK-200 (pill context menu) superseded** — the expanded pill (MS-11) already provides Copy/History/Settings quick actions. A duplicate right-click menu was deemed unnecessary.

## Architecture Notes

- `src/lib/theme.ts` exports `applyTheme()` and `watchSystemTheme()`. Both `MainApp.tsx` and `PillApp.tsx` call these on mount, reading the persisted `app.theme` setting.
- `watchSystemTheme()` returns a cleanup function that listens for `prefers-color-scheme` media query changes and re-applies the theme when set to "system".
- `commands/settings.rs::update_shortcut()` parses the shortcut string to validate format, calls `unregister_all()`, persists to DB, then `register()` the new shortcut. This ensures the old shortcut is always cleaned up.
- Tray menu now uses `start_recording_internal` / `stop_recording_internal` directly (the same functions used by the hotkey handler), avoiding the Tauri command layer.

## Known Limitations / Future Work

- Theme change in settings applies instantly to the current window but the pill window only picks up the new theme on next mount (restart or re-show). A cross-window event could sync this live.
- The shortcut recorder captures browser-level key events; some OS-reserved combinations (like Win+L) can't be captured.
- Tray menu item text is static ("Start Recording") — it doesn't dynamically update to "Stop Recording" during a recording session.
