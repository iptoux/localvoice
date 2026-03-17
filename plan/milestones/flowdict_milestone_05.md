# Milestone 5 — History
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Persist past dictation sessions and provide a browsable history with details, search, filtering, copy, and deletion.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Session persistence
- History list view
- Session detail view
- Search and basic filters
- Copy/delete actions

## Out of Scope
- Advanced export formats beyond simple baseline
- Full reprocessing analytics

## Deliverables
- Sessions table populated after every completed transcription
- History list UI
- Search by text
- Filter by language/date/model
- Session detail page with raw and cleaned transcript

## Primary User Stories
- As a user, I want to find past dictations later.
- As a user, I want to compare raw and cleaned transcripts.
- As a user, I want to remove unwanted history entries.

## Functional Breakdown

### Frontend
- Build history list and detail pages
- Implement search/filter controls
- Add copy and delete actions

### Backend / Core
- Persist sessions and segments
- Query sessions with filters
- Delete session records safely

## Dependencies
- Requires successful completion of Milestone 4 or equivalent foundation work.
- Requires a stable database schema and migration approach.
- Requires recording pipeline and command/event flow to be available.

## Technical Notes
- Persist both raw and cleaned transcript text so reprocessing remains possible later.
- Design history queries with pagination in mind even if the first UI is simple.

## Acceptance Criteria
- Completed transcriptions appear in history automatically.
- Search returns relevant prior sessions.
- Deleting a session removes it from the list and database.

## Risks
- History volume may affect UI performance
- Schema may need future evolution

## Mitigations
- Paginate or virtualize list if needed
- Use migrations from the start

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
