# Tauri v1 → v2 Migration Reference

This file documents the most important breaking changes between Tauri v1 and v2
so agents can avoid silently mixing old patterns into a v2 codebase.

Official migration guide: https://tauri.app/start/migrate/from-tauri-1/

---

## Security model: allowlist → capabilities/ACL

**v1:** `tauri.conf.json` had an `allowlist` object to enable APIs.
```json
// v1 — DO NOT use in v2
{
  "tauri": {
    "allowlist": {
      "fs": { "readFile": true, "scope": ["$APPDATA/*"] },
      "shell": { "open": true }
    }
  }
}
```

**v2:** Permissions live in `src-tauri/capabilities/*.json` files.
```json
// v2
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:allow-read-text-file",
    "shell:allow-open"
  ]
}
```

---

## Frontend imports changed

**v1:** `@tauri-apps/api` was monolithic.
```ts
// v1 — DO NOT use in v2
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import { readTextFile } from '@tauri-apps/api/fs';
```

**v2:** Core APIs are in `@tauri-apps/api/core` and `@tauri-apps/api/event`;
filesystem and other OS APIs moved to separate plugin packages.
```ts
// v2
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';
import { readTextFile } from '@tauri-apps/plugin-fs'; // plugin!
```

---

## Built-in APIs → plugins

Many built-in v1 APIs became separate plugins in v2 that must be explicitly added:

| v1 built-in | v2 plugin |
|---|---|
| `@tauri-apps/api/fs` | `@tauri-apps/plugin-fs` |
| `@tauri-apps/api/shell` | `@tauri-apps/plugin-shell` |
| `@tauri-apps/api/dialog` | `@tauri-apps/plugin-dialog` |
| `@tauri-apps/api/notification` | `@tauri-apps/plugin-notification` |
| `@tauri-apps/api/clipboard` | `@tauri-apps/plugin-clipboard-manager` |
| `@tauri-apps/api/os` | `@tauri-apps/plugin-os` |
| `@tauri-apps/api/process` | `@tauri-apps/plugin-process` |
| `@tauri-apps/api/updater` | `@tauri-apps/plugin-updater` |
| `@tauri-apps/api/globalShortcut` | `@tauri-apps/plugin-global-shortcut` |

---

## App entry point

**v1:**
```rust
// src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**v2:** The recommended pattern adds a `lib.rs` with a `run()` function so both
desktop and mobile targets can use it:
```rust
// src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// src-tauri/src/main.rs
fn main() {
    app_lib::run();
}
```

---

## Config file changes

- `tauri.conf.json` structure reorganized; some fields renamed or moved.
- `bundle.identifier` is now required and influences plugin naming.
- The `build` section moved; `distDir` → `frontendDist`, `devPath` → `devUrl`.
- Platform-specific config files are merged via JSON Merge Patch semantics.

---

## Plugin registration (Rust side)

**v2 pattern:**
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![my_command])
    .run(tauri::generate_context!())
    .expect("error running app");
```

---

## Event API changes

`app.emit_all()` (v1) → `app.emit()` (v2)
`window.emit()` still works but targets only that window.
`app.emit_to()` targets specific windows/webviews by label.

---

## When in doubt

Always check: https://tauri.app/start/migrate/from-tauri-1/
