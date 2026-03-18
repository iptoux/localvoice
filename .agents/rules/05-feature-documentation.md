# Rule 05 — Comprehensive Feature Documentation in /docs

## Rule

Every completed feature or milestone must be documented under `docs/` before it is considered done. Documentation is split into user-facing and developer-facing files, cross-linked, and kept up to date as the feature evolves.

## Directory Structure

```
docs/
  user/
    index.md                  ← user doc landing page, links to all features
    getting-started.md
    recording.md
    history.md
    dictionary.md
    models.md
    settings.md
    …
  dev/
    index.md                  ← developer doc landing page
    architecture.md           ← high-level system overview
    database-schema.md        ← full schema with field descriptions
    tauri-commands.md         ← all Tauri command signatures and payloads
    transcription-pipeline.md ← step-by-step pipeline description
    ms01-foundation.md        ← one file per milestone: decisions, gotchas, patterns
    ms02-recording.md
    …
  changelog.md                ← one entry per merged milestone / release
```

## Required Behavior

- When a milestone branch is ready to merge, create or update the corresponding `docs/dev/ms0X-*.md` file covering: what was built, key implementation decisions, non-obvious patterns, known limitations
- Create or update the relevant `docs/user/*.md` file(s) for any user-visible feature added in the milestone
- Every new `docs/` file must be linked from its section index (`docs/user/index.md` or `docs/dev/index.md`)
- Cross-link related files where relevant (e.g. `docs/user/dictionary.md` links to `docs/dev/ms08-dictionary.md`)
- `docs/changelog.md` gets one new entry per merged milestone using the format: `## MS-0X — Title (YYYY-MM-DD)` followed by a bullet list of user-visible changes
- Documentation files must be in English (per Rule 02)
- Prefer multiple focused files over one large file — split by feature, not by audience alone

## File Template: docs/dev/ms0X-*.md

```markdown
# MS-0X — <Title>

## What Was Built
<brief summary>

## Key Decisions
- <decision and reason>

## Architecture Notes
<relevant patterns, module interactions>

## Known Limitations / Future Work
<anything deferred>
```

## File Template: docs/user/*.md

```markdown
# <Feature Name>

## What It Does
<one paragraph>

## How to Use It
<step-by-step>

## Related
- [<other doc>](<relative link>)
```

## Never Do

- Merge a milestone branch without creating or updating the relevant `docs/` files
- Write documentation only in `CLAUDE.md` or `README.md` for content that belongs in `docs/`
- Create undiscoverable documentation files (every file must be linked from an index)
- Let `docs/changelog.md` fall behind merged milestones
