# Milestone 9 — Ambiguity v1
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Detect unclear or repeatedly problematic words heuristically and turn them into actionable suggestions for the user.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Heuristic ambiguity detection
- Ambiguous terms store
- Suggestion review UI
- Accept/dismiss workflow
- Conversion into correction rules

## Out of Scope
- True semantic disambiguation
- LLM-based rewriting

## Deliverables
- Ambiguous terms list
- Suggestion acceptance flow
- Dismiss flow
- Link from ambiguity item to new correction rule

## Primary User Stories
- As a user, I want the app to highlight words it often gets wrong.
- As a user, I want one-click conversion from suggestion to correction rule.

## Functional Breakdown

### Frontend
- Build ambiguity review section
- Show frequency and confidence hints
- Add accept and dismiss actions

### Backend / Core
- Collect low-confidence or repeat-correction candidates
- Persist ambiguous term stats
- Generate suggested targets when possible

## Dependencies
- Requires successful completion of Milestone 8 or equivalent foundation work.
- Requires a stable database schema and migration approach.
- Requires recording pipeline and command/event flow to be available.

## Technical Notes
- Start with conservative heuristics to avoid overwhelming the user with false positives.
- Use repeated correction patterns as a stronger signal than confidence alone.

## Acceptance Criteria
- Repeatedly problematic terms appear in the ambiguity list.
- User can accept a suggestion and create a correction rule.
- Dismissed items no longer reappear immediately without new evidence.

## Risks
- Confidence data may be limited
- Heuristics may produce false positives

## Mitigations
- Combine confidence with repeated user corrections
- Start conservative and expose thresholds later

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
