# MS-18 — macOS & Linux Support

**Goal:** Make LocalVoice build and run correctly on macOS and Linux, handling platform-specific differences in audio capture, text insertion, hotkeys, autostart, and windowing.
**Depends on:** MS-17
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-248: Platform-conditional compilation audit — review all `#[cfg(target_os = "windows")]` blocks; create corresponding `#[cfg(target_os = "macos")]` and `#[cfg(target_os = "linux")]` implementations or stubs for: `os/autostart.rs`, `os/text_insertion.rs`, `os/hotkeys.rs`, `os/tray.rs`
- [ ] TASK-249: macOS autostart — implement via `launchd` plist in `~/Library/LaunchAgents/`; wrap in `os/autostart.rs` under `#[cfg(target_os = "macos")]`
- [ ] TASK-250: Linux autostart — implement via XDG autostart `.desktop` file in `~/.config/autostart/`; wrap in `os/autostart.rs` under `#[cfg(target_os = "linux")]`
- [ ] TASK-251: macOS text insertion — implement via clipboard + Cmd+V approach (Cmd instead of Ctrl); use `CGEventCreateKeyboardEvent` or `osascript` fallback
- [ ] TASK-252: Linux text insertion — implement via `xdotool` (X11) or `wtype` (Wayland) for clipboard + Ctrl+V; detect X11 vs. Wayland at runtime
- [ ] TASK-253: Build whisper-cli sidecar for macOS (arm64 + x86_64) and Linux (x86_64); update `tauri.conf.json` `externalBin` with platform triples; bundle platform-specific libraries
- [ ] TASK-254: macOS: Build and test with `.dmg` bundle target; verify transparency, always-on-top, and tray icon work; fix platform-specific window behavior
- [ ] TASK-255: Linux: Test on Ubuntu 22.04+ and Fedora 38+; verify tray icon (requires `libappindicator`), pill transparency, global shortcuts; document required system packages
- [ ] TASK-256: CI: Add macOS and Linux build targets to GitHub Actions release pipeline; matrix build across `windows-latest`, `macos-latest`, `ubuntu-latest`
- [ ] TASK-257: Update `README.md` with platform-specific install instructions and known limitations per OS

## QA / Acceptance

- [ ] TASK-257a: Verify core workflow (record → transcribe → copy) on macOS
- [ ] TASK-257b: Verify core workflow (record → transcribe → copy) on Ubuntu 22.04
- [ ] TASK-257c: Verify autostart on each platform
- [ ] TASK-257d: Verify CI produces valid artifacts for all three platforms

---

## Acceptance Criteria

- App builds and launches on macOS (Apple Silicon + Intel) and Linux (Ubuntu 22.04+)
- Core workflow (record, transcribe, copy/insert) works on all three platforms
- Autostart works per platform
- CI builds artifacts for all three platforms
- README documents platform-specific requirements

---

## Technical Notes

- macOS and Linux are treated as **second-class initially** — getting builds working and core features functional; platform-specific polish can be iterated post-v1.0
- `cpal` audio capture works cross-platform out of the box; platform differences are mainly in OS integration (autostart, text insertion, tray)
- Linux tray icon depends on `libappindicator3` (or `libayatana-appindicator`) being installed
- Wayland global shortcuts may require `xdg-desktop-portal` or `wlr-protocols` depending on compositor
- whisper-cli sidecar must be compiled separately per platform triple (or use pre-built binaries from upstream)
