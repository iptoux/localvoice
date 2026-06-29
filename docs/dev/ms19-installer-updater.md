# MS-19 Installer and Updater

LocalVoice uses the official Tauri v2 updater plugin for self-updates from GitHub Releases.

## Runtime Flow

- `tauri-plugin-updater` is initialized in the Rust Tauri builder.
- Release builds check `https://github.com/iptoux/localvoice/releases/latest/download/latest.json`.
- Automatic checks run on startup only outside debug builds and only when `app.auto_update` is not `false`.
- The backend emits `update-available`, `update-download-progress`, and `update-error`.
- The main window displays an update banner. Installation only starts when the user clicks **Update Now**.
- `install_pending_update` downloads, installs, and restarts the app.

## Signing

The updater public key is committed in `src-tauri/tauri.conf.json`. The private key must never be committed.

Required release secrets:

| Secret | Purpose |
|---|---|
| `TAURI_SIGNING_PRIVATE_KEY` | Private updater signing key |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Optional key password |

If the private key is lost, existing installations cannot verify future update bundles signed with a different key.

## Local Builds

Use the PowerShell release helper for local Windows installer builds without publishing a GitHub release:

```powershell
.\scripts\create-release.ps1 -LocalBuild
```

`-LocalBuild` skips GitHub authentication, clean working tree checks, tag creation, and release publishing. It also disables `bundle.createUpdaterArtifacts` for that build, so no updater signing environment variable is required.

To test signed updater artifacts locally, run:

```powershell
.\scripts\create-release.ps1 -LocalBuild -UpdaterArtifacts
```

When updater artifacts are requested, the script uses `TAURI_SIGNING_PRIVATE_KEY` if it is already set. Otherwise it loads the private key from `%USERPROFILE%\.tauri\localvoice-updater.key` and clears the process environment variable before exiting.

## GitHub Release Manifest

The release workflow uploads normal installers, generated `.sig` files, and `latest.json`. The manifest contains signature file contents, not URLs to `.sig` files.

Supported updater platforms:

| Platform key | Asset |
|---|---|
| `windows-x86_64` | NSIS `*-setup.exe` |
| `linux-x86_64` | `.AppImage` |
| `darwin-aarch64` | `.app.tar.gz` |

Draft releases can be inspected manually, but the updater only sees the latest published non-prerelease GitHub release.
