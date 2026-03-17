# Milestone 10 — Polish
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Prepare the MVP for real users with better error handling, onboarding, persistence polish, autostart, remembered window positions, and overall stability improvements.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Error handling improvements
- Loading and empty states
- Remembered window/pill position
- Autostart support
- First-run onboarding for model install
- Migration and smoke tests

## Out of Scope
- Cross-device sync
- Enterprise deployment tooling

## Deliverables
- First-run flow
- Autostart toggle
- Persisted window geometry
- Improved UX for errors and loading
- Release checklist and smoke coverage

## Primary User Stories
- As a user, I want the app to feel stable and polished in daily use.
- As a user, I want first-run setup to guide me toward a usable model quickly.

## Functional Breakdown

### Frontend
- Add onboarding and empty states
- Improve error surfaces across views
- Persist and restore UI layout state

### Backend / Core
- Implement autostart integration
- Strengthen migration/test paths
- Harden file and model edge cases

## Dependencies
- Requires successful completion of Milestone 9 or equivalent foundation work.
- Requires a stable database schema and migration approach.
- Requires recording pipeline and command/event flow to be available.

## Technical Notes
- Use a release checklist so polish work stays bounded.
- Document platform-specific behavior for autostart, tray, and insertion quirks.

## Acceptance Criteria
- New users can launch, install a model, and complete first dictation without external documentation.
- App remembers pill/window positions across restarts.
- Autostart setting works on supported target platform.
- Common failure modes show actionable messages.

## Risks
- Polish work can expand endlessly
- OS autostart behavior varies

## Mitigations
- Define a fixed MVP polish checklist
- Ship Windows-first autostart and abstract for later platforms

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
