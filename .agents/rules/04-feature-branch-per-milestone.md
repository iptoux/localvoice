# Rule 04 — Feature Branch per Milestone / Plan

## Rule

Every milestone (MS-01 through MS-10) and every significant planned feature must be implemented on its own dedicated Git branch. Work is never committed directly to `main`.

## Branch Naming Convention

```
ms/01-foundation
ms/02-recording
ms/03-transcription
ms/04-output
ms/05-history
ms/06-dashboard
ms/07-models
ms/08-dictionary
ms/09-ambiguity
ms/10-polish
```

For unplanned sub-features or hotfixes branched off a milestone branch:
```
feat/<short-description>
fix/<short-description>
```

## Required Behavior

- Before starting any milestone, confirm the correct branch is checked out (or create it from `main`)
- All commits for a milestone live on its branch until the milestone acceptance criteria are met
- Only after all tasks in `plan/tasks/ms0X_*.md` are checked off and the milestone passes QA may the branch be merged into `main`
- Branch must be merged via a pull request (or explicit user instruction) — never force-pushed or merged without review
- After a successful merge, the milestone branch may be deleted; `main` then reflects the latest stable state

## Never Do

- Commit milestone or feature work directly to `main`
- Mix work from two different milestones on a single branch
- Merge a milestone branch before all acceptance criteria tasks are checked off
- Skip creating a branch because "it's just a small change" — every planned task belongs to its milestone branch
