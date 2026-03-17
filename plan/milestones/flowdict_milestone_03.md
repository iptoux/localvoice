# Milestone 3 — Local Transcription
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Deliver end-to-end local transcription via whisper.cpp sidecar, with German and English language selection and parsed transcript output.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- whisper.cpp sidecar integration
- Language selection for DE and EN
- Model invocation parameters
- Parsing stdout/json output
- Raw transcript generation
- Minimal cleaned transcript

## Out of Scope
- Model downloads UI
- Advanced post-processing
- Automatic dictionary learning

## Deliverables
- Transcription pipeline from recorded audio to text
- Support for German and English selection
- Parsed transcript segments
- Basic error handling for model execution

## Primary User Stories
- As a user, I want my speech transcribed locally without cloud services.
- As a user, I want to choose German or English before speaking.
- As a user, I want to see the final transcript after processing.

## Functional Breakdown

### Frontend
- Add language selection in pill expanded view and settings
- Show processing and result states
- Render raw and cleaned transcript preview

### Backend / Core
- Invoke whisper.cpp sidecar with configured model
- Parse segment output
- Build transcription orchestrator
- Return structured transcription result

## Dependencies
- Requires successful completion of Milestone 2 or equivalent foundation work.
- Requires recording pipeline and command/event flow to be available.

## Technical Notes
- Use whisper.cpp as a sidecar process for the MVP to reduce build complexity.
- Return structured transcript payloads with segments to support history and future ambiguity detection.

## Acceptance Criteria
- A recorded German clip can be transcribed locally.
- A recorded English clip can be transcribed locally.
- Transcription result returns structured text and metadata.
- Failures from missing model or bad invocation are surfaced clearly.

## Risks
- Packaging whisper.cpp sidecar reliably
- Model/runtime incompatibilities

## Mitigations
- Start with fixed supported models and pinned versions
- Use sidecar process isolation rather than direct FFI for MVP

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
