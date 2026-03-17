# Epics — FlowDict / LocalVoice

> Master backlog index. Detailed tasks are in `plan/tasks/ms0X_*.md`.
> Status values: `todo` · `in-progress` · `done`

## Overview

| Epic | Title | Depends on | Status |
|------|-------|-----------|--------|
| [MS-01](tasks/ms01_foundation.md) | Foundation & Shell | — | `todo` |
| [MS-02](tasks/ms02_recording.md) | Recording Core | MS-01 | `todo` |
| [MS-03](tasks/ms03_transcription.md) | Local Transcription | MS-02 | `todo` |
| [MS-04](tasks/ms04_output.md) | Output Workflow | MS-03 | `todo` |
| [MS-05](tasks/ms05_history.md) | History | MS-04 | `todo` |
| [MS-06](tasks/ms06_dashboard.md) | Dashboard | MS-05 | `todo` |
| [MS-07](tasks/ms07_models.md) | Models | MS-03 | `todo` |
| [MS-08](tasks/ms08_dictionary.md) | Dictionary v1 | MS-04 | `todo` |
| [MS-09](tasks/ms09_ambiguity.md) | Ambiguity v1 | MS-08 | `todo` |
| [MS-10](tasks/ms10_polish.md) | Polish | MS-07, MS-09 | `todo` |

> **Note:** MS-07 can begin in parallel with MS-05/06 (only needs MS-03). MS-08 needs output (MS-04).

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
