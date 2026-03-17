# Milestone 1 — Foundation & Shell
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Establish the technical foundation of the app, including the Tauri shell, React/TypeScript frontend, Rust backend bridge, SQLite bootstrap, tray integration, and the default pill-shaped main window.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Tauri v2 project setup
- React + TypeScript frontend bootstrap
- Rust command bridge between frontend and backend
- SQLite initialization and migrations
- Tray icon and basic app lifecycle
- Default pill window as the main visible surface
- Full window open/close behavior
- Theme baseline and app-level settings bootstrap

## Out of Scope
- Audio recording
- Speech transcription
- Model downloads
- History persistence beyond DB bootstrap
- Dictionary logic

## Deliverables
- Compilable desktop app for target dev platform
- Pill window shown by default on launch
- Main window can be opened from pill or tray
- SQLite database created automatically
- Initial migrations system in place
- Basic settings storage works
- Tray menu with Open, Settings, Quit

## Primary User Stories
- As a user, I want the app to launch into a small pill instead of a large window.
- As a user, I want to open the full app only when needed.
- As a developer, I want a clean project structure so later milestones can build on it safely.

## Functional Breakdown

### Frontend
- Create app shell and routing skeleton
- Implement pill component with idle placeholder state
- Implement full-window layout with empty sections
- Create settings store for UI state
- Add command wrappers for Tauri backend calls

### Backend / Core
- Initialize Tauri app and command registration
- Implement window creation and visibility control
- Implement tray integration
- Initialize SQLite and migrations
- Create settings service with defaults

## Dependencies
- None. This is the foundation milestone.

## Technical Notes
- Prefer a single source of truth for window state so the pill and full app cannot drift.
- Keep routing and feature folders in place even if views are still placeholders.

## Acceptance Criteria
- App launches successfully into pill mode by default.
- Pill is draggable and re-openable after hiding.
- Full app window can be opened from tray and pill.
- Database file is created and migrations run without manual steps.
- No placeholder crashes during window lifecycle operations.

## Risks
- Cross-platform window behavior can differ.
- Tray behavior differs by OS.

## Mitigations
- Target Windows first and abstract window ops.
- Keep tray menu minimal and centralized.

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
