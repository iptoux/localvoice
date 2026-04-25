---
name: tauri-v2
description: >
  Expert reference and decision guide for building desktop apps with Tauri v2.
  Use this skill whenever the user is working on a Tauri v2 project — whether
  they are setting up a new app, adding a plugin, writing Rust commands, wiring
  frontend-to-backend communication, handling permissions/capabilities, choosing
  a persistence strategy, or debugging a Tauri-related issue.

  Trigger on: "tauri", "tauri v2", "tauri app", "tauri command", "invoke",
  "tauri plugin", "tauri config", "tauri.conf.json", "capabilities", "ACL",
  "#[tauri::command]", "tauri build", "tauri dev", "app handle", "WebviewWindow",
  "tauri store", "tauri stronghold", "tauri fs plugin", or any mention of
  building a cross-platform desktop app with a Rust backend and web frontend.

  Do NOT use for Tauri v1 projects (different API surface), pure Electron apps,
  or purely web-based projects that happen to use "windows" in their vocabulary.
---

# Tauri v2 — Agent Reference

This skill is the **single source of truth** for any agent working on a Tauri v2
project. When official Tauri docs, blog posts, Stack Overflow snippets, and your
training data conflict — **the official Tauri v2 documentation wins.**

Tauri.app docs index: https://tauri.app/
Detailed references are bundled in `references/`:
- `references/plugins.md` — all official plugins with use-cases and doc links
- `references/code-patterns.md` — concrete code patterns for common tasks
- `references/migration-v1-to-v2.md` — key v1→v2 differences to avoid mistakes

Read the relevant reference file before generating code when you are unsure.

---

## Source-of-truth priority

When generating code, architecture, or recommendations:

1. **Official Tauri v2 documentation** (tauri.app)
2. Existing code already present in the current repository
3. Official Tauri plugin documentation for the exact plugin in use
4. Rust/crate docs for the referenced Tauri APIs
5. Community examples — only when they demonstrably match v2 patterns

Never assume a v1 API works in v2 without verifying.

---

## Core agent rules

### 1. Prefer the smallest correct solution
Use **core Tauri APIs** when they solve the problem. Add a plugin only when the
feature is explicitly provided by a plugin or clearly reduces complexity.

### 2. Security first — capability-based model
Tauri v2 replaced the v1 `allowlist` with a **capability/ACL system**.
- Permissions are defined in `src-tauri/capabilities/*.json` (or `.toml`).
- Any plugin or command touching the OS, filesystem, shell, clipboard, shortcuts,
  process handling, or external resources is security-sensitive.
- Always prefer the **narrowest permission scope** possible.

### 3. Know the integration paths

| Direction | Mechanism |
|---|---|
| Frontend → Rust | `invoke()` / `#[tauri::command]` |
| Rust → Frontend | `app.emit()` / `window.emit()` / event listeners |
| Shared backend state | `app.manage(State)` + `State<T>` in commands |
| User-persistent data | Store / Stronghold / Filesystem plugin (see §Persistence) |

### 4. Configuration boundaries
- Static app configuration → `tauri.conf.json` / `Tauri.toml`
- Runtime business state → **not** in config files
- Platform-specific overrides → platform-specific config files (merged via JSON Merge Patch)

### 5. Persistence choice (ordered by data sensitivity)

| Data type | Use |
|---|---|
| Secrets, tokens, credentials | **Stronghold** |
| App settings, feature flags, preferences | **Store** |
| User files, exports, documents | **Filesystem plugin** |
| Static packaged assets shipped with the app | **Embedded resources** (`bundle.resources`) |

---

## Decision guide

### Frontend → Rust: Commands / invoke
Use when frontend code needs trusted native functionality in Rust.
- Prefer small, explicit commands with strongly typed payloads.
- Keep command surfaces narrow — avoid "do-everything" commands.
- Validate inputs in Rust.

**Example:**
```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Register in main.rs / lib.rs:
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error running app");
```

```ts
// Frontend (TypeScript)
import { invoke } from '@tauri-apps/api/core';
const message = await invoke<string>('greet', { name: 'Maik' });
```

---

### Rust → Frontend: Events / emit
Use when the backend must push progress, status, or lifecycle changes to the UI.

```rust
// Emit from a command or background task
app_handle.emit("download-progress", payload)?;
```

```ts
// Frontend listener
import { listen } from '@tauri-apps/api/event';
const unlisten = await listen<ProgressPayload>('download-progress', (event) => {
    console.log(event.payload);
});
```

Use events for notifications and streaming updates — not as a replacement for
request/response commands.

---

### Shared Rust state
Use `app.manage()` when Rust-side shared application state must be accessed from
commands or from code with a `Manager` context.

```rust
struct AppState { counter: Mutex<u32> }

app.manage(AppState { counter: Mutex::new(0) });

#[tauri::command]
fn increment(state: State<AppState>) -> u32 {
    let mut c = state.counter.lock().unwrap();
    *c += 1;
    *c
}
```

A normal `std::sync::Mutex` is usually fine; use an async mutex only when the
guard must live across `await` points.

---

## Capabilities / Permissions (v2 ACL system)

In Tauri v2, permissions are granted through **capability files** instead of the
v1 `allowlist`. This is a hard breaking change.

Capability files live in `src-tauri/capabilities/`. Example:
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default permissions",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:allow-read-text-file",
    "store:allow-get"
  ]
}
```

Rules:
- Only grant permissions that are actually used.
- Scope filesystem permissions to specific directories where possible.
- Check plugin docs for the exact permission identifier strings.

---

## Quick selection matrix

| Need | Use |
|---|---|
| Shared runtime Rust state | State Management |
| Frontend → native logic | Commands / invoke |
| Backend → UI notifications | Events / emit |
| Packaged static files | Embedded Resources |
| App settings / preferences | Store plugin |
| Secrets / credentials | Stronghold plugin |
| Real files / directories | Filesystem plugin |
| Open URL / file in default app | Opener plugin |
| Enforce single running instance | Single Instance plugin |
| Persist window position/size | Window State plugin |
| App auto-updates | Updater plugin |
| System-wide hotkeys | Global Shortcut plugin |
| Structured app logs | Logging plugin |
| Parse CLI arguments | CLI plugin |
| Spawn external processes | Shell plugin |
| WebSocket client | WebSocket plugin |
| OS info at runtime | OS Information plugin |
| Clipboard access | Clipboard plugin |
| Persist granted scopes | Persisted Scope plugin |

See `references/plugins.md` for full details, install instructions, and doc links.

---

## Anti-patterns to avoid

- Using **Store** for secrets that belong in **Stronghold**
- Using **embedded resources** for data the user edits at runtime
- Putting runtime state into `tauri.conf.json`
- Using **events** as a substitute for request/response commands
- Adding a plugin when core Tauri APIs already solve the problem
- Requesting overly broad filesystem or shell permissions
- **Reusing Tauri v1 code without verifying v2 compatibility** — the APIs changed
  significantly (allowlist → capabilities, `tauri::api` → plugins, etc.)
- Logging secrets or sensitive payloads

---

## How to look up current docs

If you need the authoritative, up-to-date API detail for a specific feature:
1. Check `references/plugins.md` for the doc URL of the relevant plugin.
2. Fetch the doc page with `WebFetch` if available and if the domain is accessible.
3. If blocked, state clearly what you found in your training knowledge and flag
   that the user should verify against https://tauri.app for the exact current API.

---

## Agent behavior checklist

When working on a Tauri v2 codebase:

1. Prefer official Tauri v2 patterns.
2. Check whether the feature belongs to core Tauri or a plugin.
3. Mention required permissions/capabilities for every security-sensitive feature.
4. Keep frontend/backend boundaries explicit and intentional.
5. Choose persistence based on data sensitivity (see §Persistence).
6. Never invent APIs not in the official documentation.
7. Never silently mix v1 and v2 syntax.
8. When uncertain about an API detail — say so and point to the doc URL.
