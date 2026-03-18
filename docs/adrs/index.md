# Architecture Decision Records (ADRs)

This directory contains ADRs documenting key architectural decisions for LocalVoice.

## What is an ADR?

An Architecture Decision Record (ADR) documents a significant architectural decision made during the project. Each ADR follows a standard template:

- **Status**: Proposed, Accepted, Deprecated, or Superseded
- **Context**: The situation that prompted the decision
- **Decision**: What was decided
- **Consequences**: Positive, negative, and trade-offs

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [001](./001-whispercpp-sidecar.md) | whisper.cpp as Sidecar Process | Accepted |
| [002](./002-dual-window-design.md) | Dual-Window Design (Pill + Main Window) | Accepted |
| [003](./003-sqlite-database.md) | SQLite for Local Data Persistence | Accepted |

## Adding New ADRs

When making significant architectural decisions:

1. Create a new file `0XX-<title>.md` in this directory
2. Use the standard ADR template
3. Update this index table
4. Link from relevant documentation in `docs/dev/` if applicable