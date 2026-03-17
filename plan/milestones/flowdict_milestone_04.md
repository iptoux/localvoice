# Milestone 4 — Output Workflow
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Make the transcription immediately useful by copying it to the clipboard and optionally inserting/pasting it into the active application.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Clipboard output mode
- Optional auto-paste/insert fallback
- Success/error state feedback in pill
- Last transcript preview in expanded pill

## Out of Scope
- Per-app profiles
- Rich formatting insertion

## Deliverables
- Clipboard output on successful transcription
- Configurable output mode
- Success and failure feedback state
- Expanded view shows latest output

## Primary User Stories
- As a user, I want my dictation result to be usable immediately.
- As a user, I want to choose whether the result is copied or inserted.

## Functional Breakdown

### Frontend
- Add output mode setting
- Show result summary after successful processing
- Add quick copy/retry controls

### Backend / Core
- Implement clipboard writer
- Implement best-effort insert/paste fallback
- Return output success metadata

## Dependencies
- Requires successful completion of Milestone 3 or equivalent foundation work.
- Requires recording pipeline and command/event flow to be available.

## Technical Notes
- Clipboard should be the default output mode because it is the most reliable cross-app option.
- Treat direct insertion as best-effort and easy to disable.

## Acceptance Criteria
- Successful transcription copies text to clipboard by default.
- Optional paste/insert mode can be toggled.
- Pill shows success or error outcome after output step.

## Risks
- OS-level text insertion is unreliable
- Target app focus may change during processing

## Mitigations
- Use clipboard-first default
- Treat insertion as optional best-effort enhancement

## Suggested Task Buckets

### Engineering
- Implement core milestone functionality
- Add logging around critical flows
- Add smoke coverage for the happy path

### Product / UX
- Validate the interaction model against the pill-first concept
- Review language and labels for clarity
- Keep progressive disclosure intact so advanced UI does not crowd the default pill workflow

### QA
- Validate expected user path end-to-end
- Validate cancellation and failure states
- Validate persistence across app restarts where relevant

## Definition of Done

This milestone is done when all deliverables are implemented, the acceptance criteria are satisfied on the primary target platform, major failure states are handled gracefully, and the work is stable enough for the next milestone to build on without reworking the milestone’s core interfaces.
