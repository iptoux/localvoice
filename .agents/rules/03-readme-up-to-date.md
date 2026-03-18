# Rule 03 — Keep README Up to Date

## Rule

The root `README.md` must always reflect the current state of the project. Whenever a milestone is completed, a new feature is implemented, a build step changes, or setup instructions change, update `README.md` in the same work session — not as a follow-up.

## Applies To

- Any change that affects how to install, build, run, or configure the app
- Completing a milestone (update the status/roadmap section)
- Adding, removing, or renaming CLI commands, environment variables, or config keys
- Changes to system requirements (OS, dependencies, Rust version, Node version)

## Required Behavior

- `README.md` must always contain accurate: project description, prerequisites, build & run instructions, and current milestone/release status
- When a milestone task file (`plan/tasks/ms0X_*.md`) is fully checked off, update the README roadmap or status section to mark it done
- If `README.md` does not exist yet, create it before completing the first milestone (MS-01)
- Keep the README concise — link to `docs/` for deep-dive content rather than duplicating it

## Never Do

- Complete implementation work that changes the build/run workflow without updating `README.md`
- Leave the milestone status in `README.md` stale after tasks are completed
- Duplicate large blocks of documentation from `docs/` into the README — link instead
