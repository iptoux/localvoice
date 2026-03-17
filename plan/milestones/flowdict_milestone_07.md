# Milestone 7 — Models
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Allow users to discover, download, install, select, and remove local transcription models, including defaults for German and English.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Available model registry
- Download manager
- Progress reporting
- Install/delete flow
- Default model per language

## Out of Scope
- Automatic background updates
- Exotic custom model import

## Deliverables
- Models page
- Available vs installed model states
- Download progress display
- Default DE and EN model selection

## Primary User Stories
- As a user, I want to download a local model from inside the app.
- As a user, I want to choose different defaults for German and English.

## Functional Breakdown

### Frontend
- Create models UI with size and status
- Show progress and actions
- Add default model selectors

### Backend / Core
- Implement model registry metadata
- Download files with temp + verify flow
- Persist installations and defaults

## Dependencies
- Requires successful completion of Milestone 6 or equivalent foundation work.
- Requires a stable database schema and migration approach.
- Requires recording pipeline and command/event flow to be available.

## Technical Notes
- Store model metadata separately from download state where practical.
- Activate models only after verification succeeds.

## Acceptance Criteria
- User can download a supported model and use it afterward.
- Installed model survives app restart.
- User can delete an installed model safely.
- Default model can be set separately for German and English.

## Risks
- Large downloads may fail or be interrupted
- Checksum and version mismatches

## Mitigations
- Use temp files and resume-friendly logic where possible
- Verify before activation

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
