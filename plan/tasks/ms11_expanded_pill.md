# MS-11 — Expanded Pill View & Animations

**Goal:** Implement the PRD-specified expanded pill view (compact overlay showing transcript, controls, language, model, quick actions) and smooth pill state animations including waveform visualization during recording.
**Depends on:** MS-10a
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-170: Define `ExpandedPillState` type and add `isExpanded` flag to `app-store.ts`; expose `togglePillExpanded` action
- [ ] TASK-171: Tauri command `expand_pill` in `commands/window.rs` — resizes pill window from compact (300x64) to expanded (300x280); `collapse_pill` reverses; update `tauri.conf.json` maxHeight constraint
- [ ] TASK-172: Frontend wrappers `expandPill()` / `collapsePill()` in `lib/tauri.ts`; called from pill click handler (single-click = expand, double-click = open main window unchanged)
- [ ] TASK-173: React: `components/pill/ExpandedPill.tsx` — shows latest transcript text (scrollable, max 4 lines), language badge, model badge, word count
- [ ] TASK-174: React: Add Start/Stop button to `ExpandedPill` — calls `startRecording()` / `stopRecording()` from `lib/tauri.ts`; disabled during processing state
- [ ] TASK-175: React: Language quick-switch in `ExpandedPill` — dropdown switching between DE/EN; calls `updateSetting("transcription.default_language", lang)`
- [ ] TASK-176: React: Quick actions row in `ExpandedPill` — "Copy again" (re-copies last cleaned text), "Open History", "Open Settings" buttons
- [ ] TASK-177: Pill collapse-on-blur — when pill window loses focus while expanded, auto-collapse back to compact mode; uses Tauri `window.onFocusChanged`
- [ ] TASK-178: CSS transition animations for pill state changes — use `transition-all` with 200ms duration for background color; add a subtle pulse keyframe for `listening` state
- [ ] TASK-179: Waveform visualization component `components/pill/Waveform.tsx` — renders RMS audio level (from existing `audio-level` event) as animated bar/wave graphic using CSS transforms; replaces plain "Listening..." text in compact pill during recording
- [ ] TASK-180: Smooth success-to-idle transition — after success state, auto-fade back to idle after 3 seconds with opacity transition (currently uses `setTimeout` with no animation)

## QA / Acceptance

- [ ] TASK-180a: Verify expanded pill shows correct transcript, language, and model after transcription
- [ ] TASK-180b: Verify collapse-on-blur works and expanded pill does not stay open permanently
- [ ] TASK-180c: Verify waveform responds to actual audio level changes during recording

---

## Acceptance Criteria

- Single-clicking the pill expands it to show transcript, controls, and quick actions
- Expanded pill collapses on blur or second click
- Waveform animates in response to audio level during recording
- State transitions are visually smooth with no abrupt jumps

---

## Technical Notes

- Pill expansion uses window resize (not a separate window) to keep the single-pill mental model
- The `maxHeight` constraint in `tauri.conf.json` must be updated or removed to allow resize
- Waveform uses the existing `audio-level` event already emitted by the recording pipeline
- Transition animations are CSS-only to avoid JS overhead in the always-visible pill
