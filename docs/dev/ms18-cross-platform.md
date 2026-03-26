# MS-18 — macOS & Linux Support

## What Was Built

Full macOS and Linux support for all OS-integration layers: autostart, text insertion (auto-paste), Cargo dependency hygiene, Tauri config platform split, CI/CD matrix, and bootstrap script.

## Key Decisions

- **Autostart**: Each platform uses its native mechanism. Windows keeps the registry approach; macOS uses a launchd plist in `~/Library/LaunchAgents/`; Linux uses an XDG `.desktop` file in `~/.config/autostart/`. The debug build guard (`#[cfg(debug_assertions)]`) is shared across all platforms via a single early return.

- **Text insertion**: Rather than introducing a new Rust crate, platform-native CLI tools are invoked as child processes. macOS uses `osascript` to simulate `Cmd+V` via System Events. Linux detects the display server at runtime (`WAYLAND_DISPLAY` / `XDG_SESSION_TYPE`) and dispatches to `wtype` (Wayland) or `xdotool` (X11). If the tool is not installed, a warning is logged and the text stays on the clipboard for manual paste — same graceful fallback as before.

- **`windows-sys` moved to Windows-only dep**: This crate is only referenced inside `#[cfg(target_os = "windows")]` blocks. Moving it to `[target.'cfg(target_os = "windows")'.dependencies]` in `Cargo.toml` prevents it from being compiled/linked on macOS and Linux, reducing build time and avoiding potential cross-compilation issues.

- **`tauri.windows.conf.json` for DLL resources**: The Windows DLL bundle resources (`ggml.dll`, `whisper.dll`, `SDL2.dll`, etc.) were removed from the base `tauri.conf.json` and placed in `src-tauri/tauri.windows.conf.json`. Tauri v2 merges platform-specific override files automatically, so macOS and Linux builds no longer fail because the DLL files don't exist.

- **whisper.cpp built from source in macOS/Linux CI**: There are no official pre-built macOS binaries for whisper.cpp at the standard release URL. The release and CI workflows build whisper.cpp from source using `cmake`, which adds ~5–10 minutes to macOS/Linux pipeline runs but guarantees a native binary for the correct architecture.

- **CI matrix for Rust tests**: The `rust-test` job in `ci.yml` now runs a `matrix` across `windows-latest`, `macos-latest`, and `ubuntu-latest`. Ubuntu installs the required system packages (`libwebkit2gtk-4.1-dev`, `libasound2-dev`, etc.) before running `cargo test`.

- **`foreground_window.rs` unchanged**: The non-Windows stub already returns `None`, which is acceptable for macOS and Linux. Future work can add `xdotool getactivewindow getwindowname` for Linux and `lsappinfo` / AppleScript for macOS.

## Architecture Notes

```
src-tauri/src/os/
  autostart.rs        Windows: winreg | macOS: launchd plist | Linux: XDG .desktop
  text_insertion.rs   Windows: SendInput | macOS: osascript | Linux: xdotool/wtype
  foreground_window.rs Windows: GetForegroundWindow | non-Windows: None (stub)

src-tauri/
  tauri.conf.json          Base config — no platform-specific resources
  tauri.windows.conf.json  Merged on Windows — adds ggml.dll/whisper.dll/SDL2.dll

.github/workflows/
  ci.yml      rust-test matrix: windows-latest, macos-latest, ubuntu-latest
  release.yml build-windows + sign + build-macos + build-linux + release
```

## macOS Runtime Requirements

| Feature | Requirement |
|---|---|
| Auto-insert (paste) | Accessibility permission granted to LocalVoice.app |
| Tray icon | Tauri tray-icon feature (bundled) |
| Autostart | No extra permission; writes launchd plist to user LaunchAgents |

## Linux Runtime Requirements

| Feature | Package |
|---|---|
| Auto-insert (X11) | `xdotool` |
| Auto-insert (Wayland) | `wtype` |
| Tray icon | `libayatana-appindicator3` or `libappindicator3` |
| Global shortcuts | Works via `tauri-plugin-global-shortcut` (X11); Wayland may require compositor support |

## Known Limitations / Future Work

- **macOS notarization**: Release `.dmg` builds are not notarized. Users must right-click → Open to bypass Gatekeeper. Notarization requires an Apple Developer account.
- **macOS universal binary**: The CI currently builds only `aarch64-apple-darwin`. An Intel build or universal binary (`lipo`) would need a separate step.
- **Linux Wayland global shortcuts**: `tauri-plugin-global-shortcut` may not work on all Wayland compositors. Users may need to configure shortcuts manually.
- **Linux AppImage/RPM**: Depending on the Ubuntu runner environment, only `.deb` may be produced. RPM/AppImage require additional packages.
- **`foreground_window` on macOS/Linux**: Currently returns `None` — the session `output_target_app` field will be empty on these platforms.
