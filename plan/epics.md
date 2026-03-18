# Epics — FlowDict / LocalVoice

> Master backlog index. Detailed tasks are in `plan/tasks/ms*.md`.
> Status values: `todo` · `in-progress` · `done`

## Overview

### v0.1 — MVP (complete)

| Epic | Title | Depends on | Status |
|------|-------|-----------|--------|
| [MS-01](tasks/ms01_foundation.md) | Foundation & Shell | — | `done` |
| [MS-02](tasks/ms02_recording.md) | Recording Core | MS-01 | `done` |
| [MS-03](tasks/ms03_transcription.md) | Local Transcription | MS-02 | `done` |
| [MS-04](tasks/ms04_output.md) | Output Workflow | MS-03 | `done` |
| [MS-05](tasks/ms05_history.md) | History | MS-04 | `done` |
| [MS-06](tasks/ms06_dashboard.md) | Dashboard | MS-05 | `done` |
| [MS-07](tasks/ms07_models.md) | Models | MS-03 | `done` |
| [MS-08](tasks/ms08_dictionary.md) | Dictionary v1 | MS-04 | `done` |
| [MS-09](tasks/ms09_ambiguity.md) | Ambiguity v1 | MS-08 | `done` |
| [MS-10](tasks/ms10_polish.md) | Polish | MS-07, MS-09 | `done` |

> **Note:** MS-07 can begin in parallel with MS-05/06 (only needs MS-03). MS-08 needs output (MS-04).

### v0.1.1 — Settings Fixes (Critical)

| Epic | Title | Depends on | Status |
|------|-------|-----------|--------|
| [MS-10a](tasks/ms10a_settings_fixes.md) | Backend for Existing Settings | MS-10 | `done` |

> **Critical:** 6 settings in the UI have no backend implementation. Must be fixed before v0.2.

### v0.2 — Enhanced UX

| Epic | Title | Depends on | Status |
|------|-------|-----------|--------|
| [MS-11](tasks/ms11_expanded_pill.md) | Expanded Pill & Animations | MS-10a | `done` |
| [MS-12](tasks/ms12_insert_silence.md) | Insert Flow & Silence Detection | MS-10a | `done` |
| [MS-13](tasks/ms13_theme_shortcuts.md) | Theme System & Custom Shortcuts | MS-10a | `todo` |

> **Note:** MS-11, MS-12, MS-13 are independent and can run in parallel (all depend only on MS-10a).

### v0.3 — Advanced Features

| Epic | Title | Depends on | Status |
|------|-------|-----------|--------|
| [MS-14](tasks/ms14_reprocess_pipeline.md) | Session Reprocessing & Pipeline Config | MS-12 | `todo` |
| [MS-15](tasks/ms15_dashboard_confidence.md) | Stronger Dashboard & Confidence Viz | MS-14 | `todo` |
| [MS-16](tasks/ms16_history_ptt_audio.md) | Advanced History, Push-to-Talk & Audio Playback | MS-14 | `todo` |

> **Note:** MS-15 and MS-16 are independent and can run in parallel (both depend on MS-14).

### v0.4 — Cross-platform & Quality

| Epic | Title | Depends on | Status |
|------|-------|-----------|--------|
| [MS-17](tasks/ms17_tests_ci_perf.md) | Test Suite, CI/CD & Performance | MS-16 | `todo` |
| [MS-18](tasks/ms18_cross_platform.md) | macOS & Linux Support | MS-17 | `todo` |

### v1.0 — Release

| Epic | Title | Depends on | Status |
|------|-------|-----------|--------|
| [MS-19](tasks/ms19_installer_signing.md) | Installer, Updater, Signing & Data Management | MS-18 | `todo` |
| [MS-20](tasks/ms20_a11y_multilang_final.md) | Accessibility, Multi-language & Final Polish | MS-19 | `todo` |

---

## Dependency Graph

```
v0.1 (MS-01–10) ── all done
     │
     └── MS-10a (Settings Fixes) ─── v0.1.1 (critical)
              │
              ├── MS-11 (Expanded Pill) ──────┐
              ├── MS-12 (Insert + Silence) ───┤── v0.2
              └── MS-13 (Theme + Shortcuts) ──┘
              │
              └── MS-14 (Reprocess + Pipeline) ─── v0.3
                       │
                       ├── MS-15 (Dashboard + Confidence)
                       └── MS-16 (History + PTT + Audio)
                                │
                                └── MS-17 (Tests + CI + Perf) ─── v0.4
                                         │
                                         └── MS-18 (macOS/Linux)
                                                  │
                                                  └── MS-19 (Installer + Signing) ─── v1.0
                                                           │
                                                           └── MS-20 (A11y + Polish + Release)
```

---

## Task Summary

| Version | Milestones | Task Range | Task Count |
|---------|-----------|------------|------------|
| v0.1 (MVP) | MS-01 – MS-10 | TASK-001 – TASK-169 | 169 |
| v0.1.1 (fixes) | MS-10a | TASK-286 – TASK-305 | 20 |
| v0.2 | MS-11, MS-12, MS-13 | TASK-170 – TASK-200 | 31 |
| v0.3 | MS-14, MS-15, MS-16 | TASK-201 – TASK-231 | 31 |
| v0.4 | MS-17, MS-18 | TASK-232 – TASK-257 | 26 |
| v1.0 | MS-19, MS-20 | TASK-258 – TASK-285 | 28 |
| **Total** | **21 milestones** | **TASK-001 – TASK-305** | **305 tasks** |

---

## MS-01 — Foundation & Shell

**Goal:** Establish the technical foundation — Tauri shell, React/TypeScript frontend, Rust command bridge, SQLite bootstrap, tray, and default pill window.

**Acceptance Criteria:**
- App launches successfully into pill mode by default
- Pill is draggable and re-openable after hiding
- Full app window can be opened from tray and pill
- Database file is created and migrations run without manual steps
- No placeholder crashes during window lifecycle operations

---

## MS-02 — Recording Core

**Goal:** Enable microphone selection, recording start/stop, global shortcut handling, and real-time UI state transitions in the pill.

**Acceptance Criteria:**
- Global shortcut starts recording from idle
- Stopping a recording produces a valid audio artifact
- Pill visibly changes to Listening and then Processing
- Canceling a recording returns to idle without saving output

---

## MS-03 — Local Transcription

**Goal:** Deliver end-to-end local transcription via whisper.cpp sidecar with German and English language selection.

**Acceptance Criteria:**
- A recorded German clip can be transcribed locally
- A recorded English clip can be transcribed locally
- Transcription result returns structured text and metadata
- Failures from missing model or bad invocation are surfaced clearly

---

## MS-04 — Output Workflow

**Goal:** Make the transcription immediately useful by copying to clipboard and optionally auto-pasting.

**Acceptance Criteria:**
- Successful transcription copies text to clipboard by default
- Optional paste/insert mode can be toggled in settings
- Pill shows Success or Error outcome after output step

---

## MS-05 — History

**Goal:** Persist past dictation sessions and provide a browsable history with search, filters, and session detail.

**Acceptance Criteria:**
- Completed transcriptions appear in history automatically
- Search returns relevant prior sessions
- Deleting a session removes it from the list and database

---

## MS-06 — Dashboard

**Goal:** Surface useful usage metrics: total words, sessions, WPM, recording time, language usage, activity chart.

**Acceptance Criteria:**
- Dashboard displays non-zero values after sessions exist
- Word count, session count, total duration, and average WPM are correct
- Charts render from live DB data

---

## MS-07 — Models

**Goal:** Allow users to discover, download, install, select, and remove local whisper.cpp models.

**Acceptance Criteria:**
- User can download a supported model and use it afterward
- Installed model survives app restart
- User can delete an installed model safely
- Default model can be set separately for German and English

---

## MS-08 — Dictionary v1

**Goal:** Introduce manual dictionary entries and correction rules that automatically improve transcripts.

**Acceptance Criteria:**
- A configured correction rule changes future transcripts automatically
- Rules can be disabled without deletion
- Usage count increases when a rule is applied

---

## MS-09 — Ambiguity v1

**Goal:** Detect unclear or repeatedly problematic words heuristically and surface actionable suggestions.

**Acceptance Criteria:**
- Repeatedly problematic terms appear in the ambiguity list
- User can accept a suggestion and create a correction rule
- Dismissed items no longer reappear without new evidence

---

## MS-10 — Polish

**Goal:** Prepare the MVP for real users — onboarding, autostart, window persistence, error handling, release smoke test.

**Acceptance Criteria:**
- New users can launch, install a model, and complete first dictation without documentation
- App remembers pill/window positions across restarts
- Autostart works on Windows
- Common failure modes show actionable messages

---

## MS-10a — Backend Implementation for Existing Settings

**Goal:** Wire up all 6 frontend settings that have UI controls but no backend implementation: auto-punctuation, filler removal, silence timeout, push-to-talk, start hidden, default view mode.

**Acceptance Criteria:**
- Every toggle and dropdown in the Settings page has a measurable effect on app behavior
- Filler words are removed for both German and English when enabled
- Silence detection auto-stops recording after configured timeout
- Push-to-talk starts on key-down and stops on key-up
- Start hidden and default view mode control window visibility on launch

---

## MS-11 — Expanded Pill View & Animations

**Goal:** Implement the PRD-specified expanded pill view with transcript, controls, quick actions, and smooth state animations with waveform visualization.

**Acceptance Criteria:**
- Single-clicking the pill expands it to show transcript, controls, and quick actions
- Expanded pill collapses on blur or second click
- Waveform animates in response to audio level during recording
- State transitions are visually smooth with no abrupt jumps

---

## MS-12 — Improved Insert Flow & Silence Detection

**Goal:** Replace the clipboard+paste hack with more robust text insertion and add automatic recording stop after configurable silence timeout.

**Acceptance Criteria:**
- Text insertion works reliably in common apps (Notepad, VS Code, browser fields)
- Insert delay is configurable and prevents text from being lost
- Silence detection auto-stops recording after the configured timeout
- Failed insertion gracefully falls back to clipboard with clear feedback

---

## MS-13 — Theme System & Customizable Shortcuts

**Goal:** Implement proper light/dark/system theme switching and allow users to customize the global recording shortcut.

**Acceptance Criteria:**
- Light, dark, and system themes work correctly across all pages and the pill
- Theme choice persists across restarts
- Users can change the global shortcut from settings without restarting
- Tray and pill context menus provide useful quick actions

---

## MS-14 — Session Reprocessing & Configurable Post-Processing

**Goal:** Allow re-running transcription on existing sessions with a different model or settings, and let users toggle individual post-processing pipeline steps.

**Acceptance Criteria:**
- Users can reprocess a session with a different model and see updated text
- Original raw text is preserved for comparison
- Audio files are kept when enabled and automatically cleaned after retention period
- Individual post-processing steps can be toggled independently

---

## MS-15 — Stronger Dashboard & Confidence Visualization

**Goal:** Expand dashboard with language breakdown, correction stats, WPM trends, and add per-segment confidence display in history.

**Acceptance Criteria:**
- Dashboard shows language breakdown, correction frequency, and WPM trend charts
- All charts respond to date range filtering
- Session detail displays confidence per segment with color coding
- Raw vs. cleaned transcript comparison highlights differences

---

## MS-16 — Advanced History, Push-to-Talk & Audio Playback

**Goal:** Enhance history with bulk operations and pagination, add push-to-talk mode, and enable optional session audio playback.

**Acceptance Criteria:**
- Bulk delete and export work for multiple selected sessions
- Pagination handles large history gracefully with configurable page size
- Push-to-talk works as hold-to-record when enabled
- Audio playback works for sessions with retained audio files

---

## MS-17 — Test Suite, CI/CD & Performance

**Goal:** Establish comprehensive tests, automated CI/CD pipelines, and optimize startup and runtime performance.

**Acceptance Criteria:**
- At least 80% of Rust business logic modules have unit tests
- Frontend has component tests for critical UI components
- CI pipeline runs on every PR and blocks merge on failure
- App startup time is under 2 seconds on a mid-range machine

---

## MS-18 — macOS & Linux Support

**Goal:** Make LocalVoice build and run correctly on macOS and Linux with platform-specific implementations.

**Acceptance Criteria:**
- App builds and launches on macOS (Apple Silicon + Intel) and Linux (Ubuntu 22.04+)
- Core workflow (record, transcribe, copy/insert) works on all three platforms
- Autostart works per platform
- CI builds artifacts for all three platforms

---

## MS-19 — Installer, Updater, Signing & Data Management

**Goal:** Prepare for public release with proper installers, auto-update, code signing, and backup/restore.

**Acceptance Criteria:**
- Windows installer creates Start Menu and desktop shortcuts
- Auto-updater detects and installs new versions from GitHub releases
- Binaries are signed (no "unknown publisher" warnings)
- Users can backup/restore their database and export/import dictionaries

---

## MS-20 — Accessibility, Multi-language & Final Polish

**Goal:** Add keyboard navigation, screen reader support, expand language support, and apply final polish for v1.0.

**Acceptance Criteria:**
- All elements are keyboard-navigable and screen-reader-accessible
- At least 6 languages supported (DE, EN, FR, ES, IT, PT) plus auto-detect
- Complete user and developer documentation in `docs/`
- Smoke test passes end-to-end on all supported platforms
- v1.0 release is tagged and published with signed installers
