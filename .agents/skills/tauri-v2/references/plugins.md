# Tauri v2 — Official Plugin Reference

All plugins listed here are official `@tauri-apps/plugin-*` packages.
Install pattern: `npm install @tauri-apps/plugin-<name>` + `cargo add tauri-plugin-<name>`

---

## Store
**When:** Lightweight persisted key-value app data (preferences, feature flags, recent values, simple user settings).
**Not for:** Secrets (use Stronghold).
- JS: `import { Store } from '@tauri-apps/plugin-store'`
- Rust: values must be `serde_json::Value`-compatible
- Doc: https://tauri.app/plugin/store/

## Stronghold
**When:** Secrets, tokens, encryption material, credentials — anything sensitive.
**Not for:** Casual preferences or non-sensitive settings.
- Uses iota-stronghold under the hood; requires a password/key to unlock.
- Doc: https://tauri.app/plugin/stronghold/

## File System (fs)
**When:** Reading/writing user files, app-specific directories, recursive directory access.
**Security:** Highly permission-sensitive. Prefer narrowest scope. Separate app-internal storage from arbitrary user filesystem access.
- Doc: https://tauri.app/plugin/file-system/

## Updater
**When:** Self-updating desktop distributions; release channel update checks.
- Prefer HTTPS endpoints. Non-HTTPS is explicitly dangerous.
- Doc: https://tauri.app/plugin/updater/

## Window State
**When:** Persisting window size, position, and geometry across launches.
**Not for:** Arbitrary user settings unrelated to window behavior.
- Doc: https://tauri.app/plugin/window-state/

## Single Instance
**When:** Only one app instance should run; later launches should signal the existing instance (forward args, focus window).
- Note: packaging caveats for Snap/Flatpak on Linux.
- Doc: https://tauri.app/plugin/single-instance/

## Persisted Scope
**When:** Filesystem scopes or permission scope decisions must persist across launches.
- Treat as security-sensitive. Pairs well with the Filesystem plugin.
- Doc: https://tauri.app/plugin/persisted-scope/

## OS Information
**When:** Runtime OS info (version, arch, platform) for diagnostics, platform-aware logic, or support screens.
- Doc: https://tauri.app/plugin/os-info/

## Opener
**When:** Opening files, URLs, or paths with the system-default application.
- Validate inputs; keep narrowly scoped.
- Doc: https://tauri.app/plugin/opener/

## Logging
**When:** Centralized, structured application logging.
- Never log secrets or sensitive payloads.
- Doc: https://tauri.app/plugin/logging/

## Global Shortcut
**When:** Registering system-wide shortcuts for quick actions or summon/focus hotkeys.
- Permission-sensitive. Register only needed shortcuts.
- Doc: https://tauri.app/plugin/global-shortcut/

## Clipboard
**When:** Clipboard read/write (copy/paste helpers, rich desktop UX).
- Treat as privacy-sensitive.
- Doc: https://tauri.app/plugin/clipboard/

## CLI
**When:** Parsing command-line arguments at startup.
- Pairs well with Single Instance for forwarding args to a running instance.
- Doc: https://tauri.app/plugin/cli/

## Process
**When:** Inspecting or interacting with process-level information.
- Keep usage minimal and explicit.
- Doc: https://tauri.app/plugin/process/

## Shell
**When:** Truly needed to spawn commands or launch approved external binaries.
- High risk. Prefer allowlisted/narrowly scoped usage. Never broad execution.
- Doc: https://tauri.app/plugin/shell/

## WebSocket
**When:** WebSocket client functionality for real-time backend communication.
- Keep reconnection, error handling, and trust boundaries explicit.
- Doc: https://tauri.app/plugin/websocket/

---

## Core docs (not plugins)

- State Management: https://tauri.app/develop/state-management/
- Embedding Additional Files: https://tauri.app/develop/resources/
- Calling Rust from Frontend: https://tauri.app/develop/calling-rust/
- Calling Frontend from Rust: https://tauri.app/develop/calling-frontend/
- Configuration Files: https://tauri.app/develop/configuration-files/
- Capabilities / ACL: https://tauri.app/security/capabilities/
