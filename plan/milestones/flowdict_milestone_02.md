# Milestone 2 — Recording Core
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Enable microphone selection, recording start/stop, global shortcut handling, and real-time UI state transitions in the pill.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Microphone device enumeration
- Start/stop/cancel recording
- Global shortcut registration
- Temporary audio buffering or WAV output
- Recording timer in pill
- Listening/processing/idle state transitions

## Out of Scope
- Speech-to-text
- Model usage
- History browsing UI

## Deliverables
- Device list available in settings
- Recording can be started by shortcut
- Recording state reflected in pill UI
- Temporary audio artifact created after stop
- Basic audio error states surfaced to UI

## Primary User Stories
- As a user, I want to start recording instantly using a global shortcut.
- As a user, I want to see when the app is actively listening.
- As a user, I want recording to stop cleanly without reopening the main window.

## Functional Breakdown

### Frontend
- Connect recording controls to backend commands
- Show timer and state in pill
- Show selected microphone in settings
- Surface simple recording errors

### Backend / Core
- Implement audio capture service using selected device
- Write temp WAV or PCM pipeline
- Register and manage global hotkey
- Broadcast recording state events to frontend

## Dependencies
- Requires successful completion of Milestone 1 or equivalent foundation work.

## Technical Notes
- Capture audio in a way that can later be reused by the transcription orchestrator without major refactoring.
- Emit recording state changes as events instead of polling where possible.

## Acceptance Criteria
- Global shortcut starts recording from idle.
- Stopping a recording produces a valid audio artifact.
- Pill visibly changes to listening and then processing.
- Canceling a recording returns to idle without saving output.

## Risks
- Audio device compatibility issues
- Shortcut collisions with other apps

## Mitigations
- Provide device fallback and clear errors
- Allow shortcut rebinding early in settings

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
