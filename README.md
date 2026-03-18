# LocalVoice

![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)
![Tauri v2](https://img.shields.io/badge/Tauri-v2-2B90B8)
![TypeScript](https://img.shields.io/badge/TypeScript-5.8-3178C6)
![Rust](https://img.shields.io/badge/Rust-1.77+-CE422B)

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
| MS-08 | Dictionary v1 | ✅ done |
| MS-09 | Ambiguity v1 | ✅ done |
| MS-10 | Polish | ✅ done |
| MS-11 | Expanded Pill & Animations | ✅ done |
| MS-12 | Improved Insert Flow | ✅ done |
| MS-13 | Theme & Custom Shortcuts | ✅ done |
| MS-14 | Session Reprocessing & Pipeline Config | ✅ done |
| MS-15 | Stronger Dashboard & Confidence Viz | ✅ done |

## Prerequisites

| Tool | Version |
| --- | --- |
| Node.js | ≥ 20 |
| npm | ≥ 10 |
| Rust | stable (≥ 1.77) |
| Tauri CLI | v2 (installed via npm) |

Windows: no additional system libraries needed — SQLite is bundled.

## Quick Setup

Run the bootstrap script to set up your development environment automatically:

**Windows (PowerShell):**
```powershell
.\scripts\bootstrap.ps1
```

**Unix/macOS (Bash):**
```bash
./scripts/bootstrap.sh
```

The bootstrap script will:
1. Check for Node.js (≥20), Rust (≥1.77), and pnpm
2. Install pnpm if not present
3. Install Node.js dependencies
4. Download whisper.cpp binaries (or skip with `--skip-whisper`)
5. Verify the Tauri CLI is installed
6. Check Rust compilation (or skip with `--skip-verification`)

## Manual Setup

If you prefer to set up manually:

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
