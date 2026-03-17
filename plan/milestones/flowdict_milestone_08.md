# Milestone 8 — Dictionary v1
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Introduce manual dictionary entries and correction rules that can automatically improve transcripts over time.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Dictionary entries CRUD
- Correction rules CRUD
- Automatic replacement during post-processing
- Rule usage tracking

## Out of Scope
- Automatic suggestion engine
- Context-aware language models

## Deliverables
- Dictionary page
- Correction rules page/section
- Rule application in transcription pipeline
- Usage counters

## Primary User Stories
- As a user, I want to teach the app my vocabulary.
- As a user, I want repeated transcription mistakes corrected automatically.

## Functional Breakdown

### Frontend
- Build dictionary management UI
- Build correction rule editor
- Show enable/disable toggles and usage counts

### Backend / Core
- Persist dictionary entries and rules
- Apply rules in post-processing pipeline
- Increment usage stats when rules fire

## Dependencies
- Requires successful completion of Milestone 7 or equivalent foundation work.
- Requires a stable database schema and migration approach.
- Requires recording pipeline and command/event flow to be available.

## Technical Notes
- Run rule application after normalization but before ambiguity capture so replacements affect final transcript quality.
- Track usage counts for future UX around most valuable rules.

## Acceptance Criteria
- A configured correction rule changes future transcripts automatically.
- Rules can be disabled without deletion.
- Usage count increases when a rule is applied.

## Risks
- Over-aggressive replacements may harm transcript quality
- Rule conflicts can create unexpected output

## Mitigations
- Support active/inactive toggles and per-language scope
- Apply deterministic rule order and log matches

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
