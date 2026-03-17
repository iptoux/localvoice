# Rule 06 — Document Learnings from Fixes and Changes

## Rule

Every bugfix, code correction, or non-trivial change that reveals a non-obvious insight must be documented as an individual file in `C:\Users\Maik Roland Damm\.claude\learnings\`. This builds a persistent knowledge base that prevents repeating the same mistakes across sessions and projects.

## Triggers

Write a learning file whenever:
- A bug is fixed that had a non-obvious root cause
- Code had to be changed because an initial approach was wrong or incomplete
- A library, API, or framework behaved unexpectedly
- A build, config, or tooling issue was resolved after investigation
- A performance or architecture decision was reconsidered based on evidence
- A Tauri, Rust, whisper.cpp, or platform quirk was discovered

## File Naming Convention

```
YYYY-MM-DD_<short-slug>.md
```

Examples:
```
2026-03-17_tauri-sidecar-stdout-buffering.md
2026-03-17_cpal-device-id-not-persistent-across-reboots.md
2026-03-17_whisper-cpp-json-output-flag-missing-on-windows.md
```

## File Template

```markdown
# <Short descriptive title>

**Date:** YYYY-MM-DD
**Area:** <e.g. Tauri / Rust / Audio / Transcription / Frontend / Build / DB>
**Milestone:** <e.g. MS-03>

## What Happened

<Describe the bug, unexpected behavior, or situation that triggered the learning>

## Root Cause

<What was actually wrong or misunderstood>

## Fix / Solution

<What was changed and why it works>

## Learning / Rule of Thumb

<The transferable insight — what to remember or check first next time>

## References

- <link to relevant docs, issue, commit, or SO answer if applicable>
```

## Required Behavior

- Create the learning file **in the same session** the fix or insight occurs — not retroactively
- One file per distinct learning — do not bundle unrelated fixes into one file
- The "Learning / Rule of Thumb" section must be actionable: a future reader should know what to do differently
- After writing the file, no further action is required (no index update needed — the folder is browsed by date)

## Never Do

- Skip writing a learning file because the fix "was obvious" or "was small"
- Bundle multiple unrelated learnings into one file
- Write the learning only as a code comment — it must live in the learnings folder to be findable across sessions
