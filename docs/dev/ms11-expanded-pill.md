# MS-11 — Expanded Pill View & Animations

## What Was Built

An expanded pill view that reveals transcript, controls, language switching, and quick actions when the user clicks the compact pill. Also added a canvas-based waveform visualizer during recording and smooth CSS transitions for all pill state changes.

## Key Decisions

- **Window resize, not a second window** — the pill expands in-place by resizing the Tauri window from 300×64 to 300×280. This preserves the single-window mental model and avoids focus/z-order issues between multiple overlays.
- **Canvas waveform over CSS bars** — used `<canvas>` with `requestAnimationFrame` for the audio-level waveform. Canvas gives smoother animation and finer control than animating individual DOM elements, with minimal overhead since the pill is always small.
- **Collapse on blur via `window` event** — the pill listens for the browser `blur` event to auto-collapse when the user clicks away. This is simpler than using Tauri's `onFocusChanged` and works reliably on Windows.
- **Success-to-idle auto-fade** — after 3 seconds in success state, the pill fades via an opacity transition and resets to idle on the frontend. The frontend manages this timer rather than the Rust backend to keep it purely visual.

## Architecture Notes

- `expand_pill` / `collapse_pill` Tauri commands live in `commands/window.rs` — they set both `set_size` and `set_max_size` to constrain the window properly.
- `ExpandedPill.tsx` reads settings via `useSettingsStore` to show the current language and allow quick-switching.
- The `Waveform.tsx` component reads `audioLevel` from `app-store` via a ref (not re-rendering on every level change) for performance.
- Quick actions ("Copy", "History", "Settings") open the main window for History/Settings — a future iteration could navigate to specific tabs.

## Known Limitations / Future Work

- History and Settings quick-action buttons both just open the main window — they don't navigate to a specific tab yet.
- The expanded pill height (280px) is fixed; if more content is added in the future, it may need to become dynamic.
- Waveform bar count and amplification factor are hardcoded — could be made configurable for accessibility.
