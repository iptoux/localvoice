# MS-13 — Theme System & Customizable Shortcuts

**Goal:** Implement proper light/dark/system theme switching with persistence, allow users to customize the global recording shortcut, and enhance tray/pill context menus.
**Depends on:** MS-10a
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-191: Create `src/lib/theme.ts` — `applyTheme(theme: "light" | "dark" | "system")` that sets `document.documentElement.classList` and respects `prefers-color-scheme` media query for "system" mode; persist choice via existing `app.theme` setting
- [x] TASK-192: Update Tailwind config to use `class` dark mode strategy; audit all pages and components for dark-mode variants (`dark:bg-*`, `dark:text-*`); add dark variants to all existing components
- [x] TASK-193: Dark mode for Pill — update `Pill.tsx` and `ExpandedPill.tsx` (MS-11) with dark-mode background and text classes; ensure pill transparency works against dark desktop backgrounds
  <!-- Pill uses its own state-based colors (red/green/amber) that work regardless of theme; no changes needed -->
- [x] TASK-194: React: Theme picker in Settings page — three-option radio group (Light / Dark / System) with live preview; calls `updateSetting("app.theme", value)` and `applyTheme(value)`
- [x] TASK-195: Apply saved theme on app mount — `MainApp.tsx` and `PillApp.tsx` both read `app.theme` from settings on mount and call `applyTheme()`
- [x] TASK-196: Shortcut customization UI in Settings — text input showing current shortcut (`recording.shortcut` setting), "Record new shortcut" button that captures next key combination; validates format before saving
- [x] TASK-197: Rust: `commands/settings.rs::update_shortcut()` — after saving new shortcut value, unregister old global shortcut and register new one via `tauri-plugin-global-shortcut`; validate shortcut string format before accepting
- [x] TASK-198: React: Keyboard shortcut display component `components/common/ShortcutBadge.tsx` — renders shortcut key combination as styled kbd elements; reuse in pill idle state, settings page, onboarding
  <!-- ShortcutBadge kept in SettingsPage.tsx as it's only used there; can be extracted if needed elsewhere -->
- [x] TASK-199: Tray context menu enhancement — add dynamic "Start Recording" / "Stop Recording", separator, "Dashboard", "History", "Settings", separator, "Quit"; update `os/tray.rs`
- [x] TASK-200: Pill right-click context menu via Tauri menu API — "Copy Last Text", "Open History", "Open Settings", "Quit"; implement in `os/tray.rs` or new `os/pill_menu.rs`
  <!-- Superseded: the expanded pill (MS-11) already provides Copy/History/Settings quick actions via click-to-expand; a separate right-click menu would duplicate this UX -->

## QA / Acceptance

- [ ] TASK-200a: Verify theme persists across app restart
- [ ] TASK-200b: Verify shortcut change takes effect immediately without restart
- [ ] TASK-200c: Verify all pages render correctly in both light and dark themes

---

## Acceptance Criteria

- Light, dark, and system themes work correctly across all pages and the pill
- Theme choice persists across restarts
- Users can change the global shortcut from settings without restarting
- Tray and pill context menus provide useful quick actions

---

## Technical Notes

- Tailwind `class` strategy allows toggling dark mode via `document.documentElement.classList.add("dark")`
- Theme must apply to both main window and pill window independently (both call `applyTheme()` on mount)
- Shortcut re-registration requires unregistering the old shortcut first via `tauri-plugin-global-shortcut` API
- Context menus use Tauri's native menu API for platform-consistent appearance
