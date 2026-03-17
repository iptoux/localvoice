# LocalVoice

Offline-first desktop voice dictation. Record with a global shortcut, transcribe locally via whisper.cpp, output to clipboard or active app — no cloud, no telemetry.

## Status

| Milestone | Description | Status |
| --- | --- | --- |
| MS-01 | Foundation & Shell | ✅ done |
| MS-02 | Recording Core | ✅ done |
| MS-03 | Local Transcription | ✅ done |
| MS-04 | Output Workflow | ✅ done |
| MS-05 | History | ✅ done |
| MS-06 | Dashboard | ✅ done |
| MS-07 | Models | ✅ done |
| MS-08 | Dictionary v1 | todo |
| MS-09 | Ambiguity v1 | todo |
| MS-10 | Polish | todo |

## Prerequisites

| Tool | Version |
| --- | --- |
| Node.js | ≥ 20 |
| npm | ≥ 10 |
| Rust | stable (≥ 1.77) |
| Tauri CLI | v2 (installed via npm) |

Windows: no additional system libraries needed — SQLite is bundled.

## Setup

```bash
# Install frontend dependencies
pnpm install

# Run in development (hot-reload frontend + Rust watch)
pnpm tauri dev

# Production build
pnpm tauri build
```

## Project Layout

```text
src/                  React/TypeScript frontend
src-tauri/            Rust backend (Tauri v2)
  src/
    commands/         Tauri #[tauri::command] handlers
    db/               SQLite layer (migrations, repositories)
    state/            AppState shared across commands
    os/               tray, hotkeys, clipboard, text_insertion
    errors/           AppError / CmdResult types
plan/                 PRD, milestones, task lists, learnings
docs/
  user/               User-facing guides
  dev/                Developer/architecture docs
```

See [docs/dev/index.md](docs/dev/index.md) for the full developer reference.
