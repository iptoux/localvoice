# Platform Support

LocalVoice runs on Windows, macOS, and Linux. Core features (record, transcribe, copy/insert) work on all platforms. Some platform-specific features require extra setup.

## What It Does

LocalVoice adapts its OS integration to each platform:

- **Autostart**: registers itself with the OS login mechanism on each platform (registry on Windows, launchd on macOS, XDG autostart on Linux)
- **Auto-insert**: pastes transcribed text into the focused application using the platform's native paste mechanism
- **Tray icon**: uses the system tray on all platforms (requires `libappindicator` on Linux)
- **Global shortcut**: registers a system-wide hotkey for recording

## Platform Status

| Platform | Supported | Installer |
|---|---|---|
| Windows 10/11 (x64) | ✓ Full support | `.msi` or `-setup.exe` |
| macOS Apple Silicon (arm64) | ✓ Supported | `.dmg` |
| macOS Intel (x86_64) | ✓ Supported | `.dmg` |
| Ubuntu 22.04+ (x64) | ✓ Supported | `.deb` / `.AppImage` |
| Fedora 38+ (x64) | ✓ Supported | `.rpm` / `.AppImage` |

## Windows

No extra setup required. The installer includes all required libraries.

**SmartScreen warning:** If Windows shows "Windows protected your PC", click **More info → Run anyway**. Release builds are signed with an Authenticode certificate.

## macOS

### First Launch

macOS blocks unsigned apps by default. To open LocalVoice:
1. Right-click `LocalVoice.app` → **Open**
2. Click **Open** in the security dialog

### Auto-Insert (Paste)

The auto-insert feature requires Accessibility permission:

1. Open **System Settings → Privacy & Security → Accessibility**
2. Click the lock icon and authenticate
3. Toggle **LocalVoice** to enabled

Without this permission, transcribed text is still copied to your clipboard — you can paste it manually with `⌘V`.

### Autostart

The **Launch at login** toggle in Settings writes a launchd plist to:
```
~/Library/LaunchAgents/com.localvoice.app.plist
```

### System Requirements

- macOS 12 (Monterey) or later recommended
- Xcode Command Line Tools (for building from source)

## Linux

### System Dependencies

Install the required packages for your distribution:

**Debian / Ubuntu:**
```bash
sudo apt-get install -y libayatana-appindicator3-dev
# For auto-insert on X11:
sudo apt-get install -y xdotool
# For auto-insert on Wayland:
sudo apt-get install -y wtype
```

**Fedora / RHEL:**
```bash
sudo dnf install -y libappindicator-gtk3
# For auto-insert on X11:
sudo dnf install -y xdotool
```

### Auto-Insert (Paste)

LocalVoice detects your display server automatically:
- **X11**: uses `xdotool key ctrl+v`
- **Wayland**: uses `wtype -k ctrl+v`

If the tool is not installed, transcribed text is still copied to your clipboard for manual paste (`Ctrl+V`).

### Tray Icon

The tray icon requires `libayatana-appindicator3` (Debian/Ubuntu) or `libappindicator3`. If neither is installed, the tray icon will not appear, but the app still runs.

### Autostart

The **Launch at login** toggle writes an XDG autostart file to:
```
~/.config/autostart/localvoice.desktop
```
This works on GNOME, KDE, XFCE, and other XDG-compliant desktop environments.

### Wayland Global Shortcuts

Global shortcuts (record hotkey) may not work on all Wayland compositors. If your shortcut does not register:
- Try running under XWayland
- Check your compositor's keybinding settings
- Use the tray icon to start/stop recording as a workaround

## Related

- [Developer docs — MS-18 Cross-Platform](../dev/ms18-cross-platform.md)
- [Output & Auto-Insert](text-insertion.md)
- [Settings](../dev/index.md)
