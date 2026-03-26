<div align="center">

<img src="docs/localvoice_appicon_transparent.png" alt="LocalVoice" width="120"/>

# LocalVoice

**Offline-first desktop voice dictation — no cloud, no telemetry, just your voice.**

Record with a global shortcut, transcribe locally with [whisper.cpp](https://github.com/ggerganov/whisper.cpp), and send text straight to your clipboard or active app.

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-2B90B8)](https://tauri.app)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.8-3178C6)](https://www.typescriptlang.org)
[![Rust](https://img.shields.io/badge/Rust-1.77+-CE422B)](https://www.rust-lang.org)

</div>

---

## What is LocalVoice?

LocalVoice is a lightweight desktop app that turns your voice into text — entirely on your machine. There's no account to create, no audio sent to a server, and no subscription. Just press a shortcut, speak, and your words appear wherever your cursor is.

It's built for developers, writers, and anyone who wants fast, private voice input as part of their daily workflow.

---

## Key Features

- **Global hotkey recording** — start and stop dictation from anywhere on your desktop
- **Push-to-talk mode** — hold the shortcut to record, release to stop; configurable per session
- **100% local transcription** — powered by whisper.cpp; your audio never leaves your machine
- **Multiple Whisper models** — download and switch between models per language
- **Smart output** — insert directly into the active app, copy to clipboard, or preview first
- **Custom dictionary** — teach LocalVoice your vocabulary, acronyms, and corrections
- **Filler word removal** — automatically strips "um", "uh", and other fillers
- **Ambiguity detection** — flags low-confidence phrases for your review
- **Session history** — browse, search, and filter past transcriptions with bulk select, delete, and export
- **Audio playback** — replay the original recording directly from the session detail view
- **Pagination & filters** — configurable page size (25/50/100), "has audio" toggle, and date quick-presets
- **CSV / JSON / TXT export** — export single or multiple sessions in your preferred format
- **Dashboard & analytics** — WPM trends, language breakdown, correction metrics
- **Session reprocessing** — re-run post-processing on past sessions with updated rules
- **Compact pill UI** — a small floating window that stays out of your way
- **Themes & shortcuts** — light, dark, or system theme; fully configurable hotkeys
- **No telemetry** — zero data collection, ever

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop framework | [Tauri v2](https://tauri.app) |
| Frontend | React 19, TypeScript 5.8, Vite |
| Styling | Tailwind CSS v4, shadcn/ui, Radix UI |
| State management | Zustand |
| Backend | Rust (stable ≥ 1.77) |
| Database | SQLite (bundled via rusqlite) |
| Transcription | whisper.cpp (local sidecar binary) |
| Audio capture | cpal, hound |

---

## Screenshots

<table>
  <tr>
    <td align="center">
      <img src="docs/screenshots/dashboard.png" alt="Dashboard — usage stats, WPM chart, language breakdown" width="480"/>
      <br/><sub>Dashboard — usage stats, WPM trend &amp; language breakdown</sub>
    </td>
    <td align="center">
      <img src="docs/screenshots/models-light.png" alt="Model Manager — download and manage whisper.cpp models (light theme)" width="480"/>
      <br/><sub>Model Manager — download whisper.cpp models (dark theme)</sub>
    </td>
  </tr>
  <tr>
    <td align="center">
      <img src="docs/screenshots/models-dark.png" alt="Model Manager — installed models with per-language defaults (dark theme)" width="480"/>
      <br/><sub>Model Manager — per-language defaults &amp; installed models (light theme)</sub>
    </td>
    <td align="center">
      <img src="docs/screenshots/logs.png" alt="Log Viewer — filterable in-app debug logs with export" width="480"/>
      <br/><sub>Log Viewer — filterable in-app debug logs with JSON export</sub>
    </td>
  </tr>
</table>

---

## Platform Support

| Platform | Status | Notes |
|---|---|---|
| **Windows 10/11 x64** | First-class | Fully supported, signed installers |
| **macOS Apple Silicon (arm64)** | Supported | Unsigned builds; Accessibility permission required for auto-insert |
| **macOS Intel (x86_64)** | Supported | Same as Apple Silicon |
| **Linux x86_64** | Supported | Requires `xdotool` (X11) or `wtype` (Wayland) for auto-insert; `libappindicator` for tray |

---

## Installation

> **Security notices by platform**
>
> - **Windows:** You may see a SmartScreen warning ("unknown publisher"). Click **"More info" → "Run anyway"**. Release binaries are Authenticode-signed via SignPath.
> - **macOS:** The app is not notarized. Right-click the `.app` → Open → Open to bypass Gatekeeper. Auto-insert (paste) requires granting Accessibility permission in System Settings → Privacy & Security → Accessibility.
> - **Linux:** No code signing. Mark the binary executable and run directly. Tray icon requires `libayatana-appindicator3` or `libappindicator3`.

### Prerequisites

| Tool | Version |
|---|---|
| Node.js | ≥ 20 |
| pnpm | ≥ 9 |
| Rust | stable ≥ 1.77 |

**macOS additional requirements:**
- Xcode Command Line Tools: `xcode-select --install`
- cmake (for building whisper.cpp): `brew install cmake`

**Linux additional requirements:**
```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev libasound2-dev \
  cmake build-essential
# For auto-insert (X11):
sudo apt-get install -y xdotool
# For auto-insert (Wayland):
sudo apt-get install -y wtype
```

### Quick Setup (recommended)

Clone the repo and run the bootstrap script — it handles dependencies, whisper binaries, and build verification automatically.

**Windows (PowerShell):**
```powershell
git clone https://github.com/your-username/localvoice.git
cd localvoice
.\scripts\bootstrap.ps1
```

**macOS / Linux:**
```bash
git clone https://github.com/your-username/localvoice.git
cd localvoice
./scripts/bootstrap.sh
```

The script will:
1. Check for Node.js, Rust, and pnpm (and install pnpm if missing)
2. Check Linux system packages (Linux only)
3. Install frontend dependencies
4. Download or build whisper.cpp binaries for your platform (skip with `--skip-whisper`)
5. Verify the Tauri CLI is available
6. Run a Rust compilation check (skip with `--skip-verification`)

### Manual Setup

```bash
# Install frontend dependencies
pnpm install

# Start the dev server (hot-reload frontend + Rust watch)
pnpm tauri dev

# Production build
pnpm tauri build
```

### whisper.cpp Binaries (required)

The bootstrap script handles this automatically. For manual setup, you need to place a platform-appropriate binary in `src-tauri/binaries/`:

**Windows:**

1. Download `whisper-bin-win-x64.zip` from the [whisper.cpp v1.7.1 release](https://github.com/ggerganov/whisper.cpp/releases/tag/v1.7.1)
2. Extract and copy files:

| File | Destination |
|---|---|
| `Release/whisper-cli.exe` | `src-tauri/binaries/whisper-cli-x86_64-pc-windows-msvc.exe` |
| `Release/ggml.dll` | `src-tauri/ggml.dll` |
| `Release/ggml-base.dll` | `src-tauri/ggml-base.dll` |
| `Release/ggml-cpu.dll` | `src-tauri/ggml-cpu.dll` |
| `Release/whisper.dll` | `src-tauri/whisper.dll` |
| `Release/SDL2.dll` | `src-tauri/SDL2.dll` |

**macOS (Apple Silicon):**

```bash
git clone --depth 1 --branch v1.7.1 https://github.com/ggerganov/whisper.cpp
cd whisper.cpp && cmake -B build -DCMAKE_BUILD_TYPE=Release -DWHISPER_BUILD_EXAMPLES=ON
cmake --build build --target whisper-cli -j$(sysctl -n hw.logicalcpu)
cp build/bin/whisper-cli ../src-tauri/binaries/whisper-cli-aarch64-apple-darwin
```

**macOS (Intel):** same steps, use target name `whisper-cli-x86_64-apple-darwin`.

**Linux:**

```bash
git clone --depth 1 --branch v1.7.1 https://github.com/ggerganov/whisper.cpp
cd whisper.cpp && cmake -B build -DCMAKE_BUILD_TYPE=Release -DWHISPER_BUILD_EXAMPLES=ON
cmake --build build --target whisper-cli -j$(nproc)
cp build/bin/whisper-cli ../src-tauri/binaries/whisper-cli-x86_64-unknown-linux-gnu
```

> All binary files are excluded from version control (`.gitignore`). Every contributor must provide them manually or run the bootstrap script.

### Platform-specific notes

**macOS — Auto-insert (paste):**
`LocalVoice.app` must be granted Accessibility permission:
System Settings → Privacy & Security → Accessibility → enable LocalVoice.

**macOS — Autostart:**
The autostart toggle writes a launchd plist to `~/Library/LaunchAgents/com.localvoice.app.plist`.

**Linux — Auto-insert (paste):**
Install the tool matching your display server:
- X11: `sudo apt-get install xdotool`
- Wayland: `sudo apt-get install wtype`

**Linux — Tray icon:**
Requires `libayatana-appindicator3` or `libappindicator3`. Install via:
```bash
sudo apt-get install libayatana-appindicator3-dev
```

**Linux — Autostart:**
The autostart toggle writes an XDG `.desktop` file to `~/.config/autostart/localvoice.desktop`.

---

## Configuration

LocalVoice stores all settings in a local SQLite database — no config files to edit by hand. Everything is configurable through the app's Settings page:

| Setting | Description |
|---|---|
| Recording shortcut | Global hotkey to start/stop recording |
| Output mode | Insert to active app, clipboard, or preview |
| Default language | Language used for transcription |
| Active Whisper model | Per-language model selection |
| Theme | System, light, or dark |
| Filler words | Language-specific list of words to strip |
| Audio retention | Whether to keep raw audio after transcription |
| Logging | Enable/disable in-app debug logging |

---

## Usage

1. Launch LocalVoice — a small pill window appears on screen.
2. Press your configured shortcut (default: customizable in Settings) to start recording.
3. Speak. The pill animates to show it's listening.
4. Press the shortcut again (or let silence detection stop it automatically).
5. LocalVoice transcribes locally and sends the text to your active app or clipboard.

You can open the full dashboard at any time to browse history, manage models, edit your dictionary, review ambiguous phrases, and view usage stats.

---

## Project Structure

```
src/                  React/TypeScript frontend
src-tauri/
  src/
    commands/         Tauri IPC command handlers
    db/               SQLite layer (migrations, repositories)
    audio/            Recording and device management
    transcription/    whisper.cpp sidecar protocol
    postprocess/      Text cleaning, filler removal, corrections
    dictionary/       Custom vocabulary and correction rules
    os/               Tray, hotkeys, clipboard, text insertion
    state/            AppState shared across commands
    errors/           AppError / CmdResult types
docs/
  user/               User-facing guides
  dev/                Developer and architecture docs
scripts/              Bootstrap and utility scripts
```

Full developer reference: [docs/dev/index.md](docs/dev/index.md)

---

## Contributing

Contributions are welcome. Here's how to get involved:

**Reporting bugs**
Open an issue with a clear description, steps to reproduce, your OS, and the LocalVoice version. Attach logs from the in-app log viewer if relevant.

**Suggesting features**
Open a discussion or issue describing the use case and why it would be valuable. Check existing issues first to avoid duplicates.

**Submitting a pull request**
1. Fork the repo and create a branch from `main`: `git checkout -b feat/your-feature`
2. Follow the existing code style — Rust uses standard `rustfmt`, TypeScript uses the project's ESLint config
3. Keep PRs focused — one feature or fix per PR
4. Update or add documentation if your change affects user-facing behavior
5. Open the PR with a clear description of what changed and why

**Good first issues**
Look for issues tagged `good first issue` — these are scoped and well-documented entry points.

---

## Code signing policy

Free code signing provided by SignPath.io, certificate by SignPath Foundation.

Official Windows release artifacts for LocalVoice are built from this public repository and submitted for signing only from the project's release workflow.

Committers and reviewers:
- @iptoux

Approvers:
- @iptoux

Only release artifacts produced from the LocalVoice repository are submitted for signing.
Third-party upstream binaries included in release packages remain attributed to their original upstream projects and are not individually signed under the LocalVoice project certificate.

---

## License

MIT — see [LICENSE](LICENSE) for the full text.

---

<div align="center">
Built with <a href="https://tauri.app">Tauri</a> · Transcription by <a href="https://github.com/ggerganov/whisper.cpp">whisper.cpp</a>
</div>
