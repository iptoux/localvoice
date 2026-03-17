# Milestone 6 — Dashboard
# Context

This document is a standalone delivery brief for one milestone of the FlowDict project: an offline-first desktop dictation application built with Tauri v2, TypeScript, Rust, and whisper.cpp. The product defaults to a small floating pill that shows the current voice state and expands into richer views when needed.

## Goal

Surface useful usage metrics and trends such as total words, sessions, WPM, recording time, language usage, and activity over time.
## Why This Milestone Matters

This milestone unlocks a concrete product capability and reduces downstream implementation risk by establishing interfaces, behaviors, and constraints needed by later milestones.
## In Scope
- Aggregate stats computation
- Dashboard cards
- Simple time-series chart
- Language usage summary
- Basic correction and ambiguity placeholders if data exists

## Out of Scope
- Complex analytics segmentation
- Custom report builder

## Deliverables
- Dashboard overview cards
- Words-over-time chart
- Average WPM metric
- Total recording duration metric

## Primary User Stories
- As a user, I want to see how much I have used the app.
- As a user, I want to understand my dictation speed and trends over time.

## Functional Breakdown

### Frontend
- Build dashboard cards and chart layout
- Add selectable time range filters if simple

### Backend / Core
- Create aggregation queries for sessions
- Compute WPM and totals
- Return chart-ready series

## Dependencies
- Requires successful completion of Milestone 5 or equivalent foundation work.
- Requires a stable database schema and migration approach.
- Requires recording pipeline and command/event flow to be available.

## Technical Notes
- Define formulas for word count, duration, and WPM centrally to avoid inconsistencies.
- Prefer pre-aggregated query helpers over duplicating logic in frontend code.

## Acceptance Criteria
- Dashboard displays non-zero values after sessions exist.
- Word count, session count, total duration, and average WPM are correct within expected tolerance.
- Charts render from live DB data.

## Risks
- WPM may be noisy on short sessions
- Stats queries may drift from source logic

## Mitigations
- Define one canonical stats service
- Document metric formulas clearly

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
