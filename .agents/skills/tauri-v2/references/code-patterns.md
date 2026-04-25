# Tauri v2 — Common Code Patterns

Quick reference for the most frequently needed Tauri v2 patterns.

---

## Project structure

```
my-app/
├── src/                    # Frontend (React/Vue/Svelte/Vanilla)
├── src-tauri/
│   ├── src/
│   │   ├── main.rs         # Entry point (calls lib::run)
│   │   └── lib.rs          # App builder + command registration
│   ├── capabilities/
│   │   └── default.json    # Permission grants (v2 ACL)
│   ├── icons/
│   ├── tauri.conf.json     # App config (identifier, bundle, build)
│   └── Cargo.toml
├── package.json
└── vite.config.ts (or equivalent)
```

---

## Creating a new project

```bash
npm create tauri-app@latest
# Follow the prompts: choose frontend framework, package manager, etc.
cd my-app
npm install
npm run tauri dev
```

---

## Adding a Rust command

```rust
// src-tauri/src/lib.rs
#[tauri::command]
fn read_config(app: tauri::AppHandle) -> Result<String, String> {
    // use app to access managed state, emit events, etc.
    Ok("config data".to_string())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_config])
        .run(tauri::generate_context!())
        .expect("error running app");
}
```

```ts
// Frontend
import { invoke } from '@tauri-apps/api/core';
const config = await invoke<string>('read_config');
```

---

## Managed state

```rust
use std::sync::Mutex;

struct AppState {
    count: Mutex<u32>,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState { count: Mutex::new(0) })
        .invoke_handler(tauri::generate_handler![increment])
        .run(tauri::generate_context!())
        .expect("error");
}

#[tauri::command]
fn increment(state: tauri::State<AppState>) -> u32 {
    let mut c = state.count.lock().unwrap();
    *c += 1;
    *c
}
```

---

## Emitting events from Rust

```rust
// From a command
#[tauri::command]
async fn start_task(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        for i in 0..100 {
            app.emit("task-progress", i).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}
```

```ts
// Frontend listener
import { listen } from '@tauri-apps/api/event';
const unlisten = await listen<number>('task-progress', (event) => {
    console.log('Progress:', event.payload);
});
// Clean up when done:
unlisten();
```

---

## Using the Store plugin

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-store = "2"
```

```rust
// Register plugin
.plugin(tauri_plugin_store::Builder::default().build())
```

```ts
// Frontend
import { load } from '@tauri-apps/plugin-store';

const store = await load('settings.json', { autoSave: true });
await store.set('theme', 'dark');
const theme = await store.get<string>('theme');
```

Capability needed: `"store:default"` or specific `"store:allow-get"`, `"store:allow-set"`

---

## Reading/writing files

```ts
import { readTextFile, writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

const content = await readTextFile('config.json', {
    baseDir: BaseDirectory.AppConfig,
});

await writeTextFile('output.txt', 'Hello!', {
    baseDir: BaseDirectory.Document,
});
```

Capability needed:
```json
"permissions": ["fs:allow-read-text-file", "fs:allow-write-text-file"]
```
Add scope for directories if needed:
```json
{ "identifier": "fs:scope-app-config-recursive", "allow": [{ "path": "$APPCONFIG/**" }] }
```

---

## Opening a URL / file in default app

```ts
import { openUrl, openPath } from '@tauri-apps/plugin-opener';

await openUrl('https://tauri.app');
await openPath('/path/to/file.pdf');
```

Capability: `"opener:default"` or `"opener:allow-open-url"`

---

## Capability file template

`src-tauri/capabilities/default.json`:
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default app permissions",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "store:default",
    "fs:allow-read-text-file",
    "fs:allow-write-text-file"
  ]
}
```

Run `npm run tauri build -- --no-bundle` or `cargo tauri build` after adding
permissions to regenerate the schema.

---

## tauri.conf.json key fields (v2)

```json
{
  "productName": "My App",
  "version": "0.1.0",
  "identifier": "com.example.myapp",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "My App",
        "width": 1024,
        "height": 768
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": ["icons/32x32.png", "icons/icon.ico", "icons/icon.icns"]
  }
}
```
