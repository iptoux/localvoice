I'll start by exploring the codebase to understand the existing structure and any existing conventions.

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: glob
Input: {
  "pattern": "**/*.md"
}

🔧 Tool: glob
Input: {
  "pattern": "**/Cargo.toml"
}
Let me explore the Rust source code and existing documentation to understand conventions:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\Cargo.toml"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\ms01-foundation.md"
}

🔧 Tool: glob
Input: {
  "pattern": "src-tauri/src/**/*.rs"
}
Let me examine the existing error handling code and some commands:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\errors\\mod.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\commands\\settings.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\commands\\recording.rs"
}
Let me also examine the frontend error handling:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\lib\\tauri.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\app-store.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx"
}
Let me check how errors are emitted to the frontend and look at a few more command files:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\transcription\\orchestrator.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\state\\app_state.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\commands\\history.rs"
}
Let me also check the frontend event handling and look for any existing patterns:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\main.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\settings-store.ts"
}

🔧 Tool: glob
Input: {
  "pattern": "docs/dev/*.md"
}
Let me look at how events are listened to on the frontend and check the types:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\types\\index.ts"
}

🔧 Tool: grep
Input: {
  "include": "*.tsx",
  "pattern": "listen\\("
}

🔧 Tool: grep
Input: {
  "include": "*.ts",
  "pattern": "recording-state-changed|on\\(['\\\"]"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\hooks\\use-throttled-event.ts"
}

🔧 Tool: glob
Input: {
  "pattern": "src/hooks/*.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\hooks\\use-text-processor.ts"
}

🔧 Tool: grep
Input: {
  "include": "*.tsx",
  "pattern": "catch|\\.catch|try.*catch"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\PillApp.tsx"
}
Now I have a complete understanding of the error handling patterns. Let me create the documentation:

🔧 Tool: write
Input: {
  "content": "# Error Handling Conventions\n\nThis document defines the project's conventions for Rust error handling and how frontend code should handle error events from Tauri.\n\n## Table of Contents\n\n1. [Rust Backend Error Types](#rust-backend-error-types)\n2. [Result Type Aliases](#result-type-aliases)\n3. [Creating Errors](#creating-errors)\n4. [Propagating Errors](#propagating-errors)\n5. [Frontend Event Handling](#frontend-event-handling)\n6. [Frontend Command Error Handling](#frontend-command-error-handling)\n7. [User-Friendly Error Messages](#user-friendly-error-messages)\n\n---\n\n## Rust Backend Error Types\n\nAll error handling is centralized in `src-tauri/src/errors/mod.rs`.\n\n### AppError\n\n```rust\n#[derive(Debug, Serialize)]\npub struct AppError(pub String);\n```\n\n`AppError` is a simple wrapper around a `String`. It serializes to a plain string so the frontend receives a descriptive error message when a command fails.\n\n### CmdResult\n\n```rust\npub type CmdResult<T> = Result<T, AppError>;\n```\n\nAll Tauri commands return `CmdResult<T>`.\n\n---\n\n## Creating Errors\n\n### From strings\n\n```rust\n// Using .into() for implicit conversion\nreturn Err(\"Recording already in progress\".into());\n\n// Explicit construction\nreturn Err(AppError(\"Invalid shortcut format\".to_string()));\n\n// With format!\nreturn Err(format!(\"Failed to register shortcut '{}': {}\", shortcut, e).into());\n```\n\n### From library errors\n\nThe `From` trait implementations handle common error types:\n\n```rust\n// rusqlite errors automatically convert\nfn get_settings(state: State<AppState>) -> CmdResult<HashMap<String, String>> {\n    settings_repo::get_all(&state.db).map_err(Into::into)\n}\n```\n\nFor errors without automatic conversion, use `.map_err()`:\n\n```rust\nreturn std::fs::write(&path, content)\n    .map_err(|e| AppError(format!(\"Failed to write export file: {e}\")))?;\n```\n\n---\n\n## Propagating Errors\n\n### Standard propagation with `?`\n\n```rust\n#[tauri::command]\npub fn get_session(state: State<AppState>, session_id: String) -> CmdResult<SessionWithSegments> {\n    // ? operator works because of From implementations\n    sessions_repo::get_session(&state.db, &session_id)\n}\n```\n\n### Explicit conversion with `.map_err(Into::into)`\n\n```rust\n#[tauri::command]\npub fn get_settings(state: State<AppState>) -> CmdResult<HashMap<String, String>> {\n    settings_repo::get_all(&state.db).map_err(Into::into)\n}\n```\n\n### Inline error creation\n\n```rust\nnormalized\n    .parse::<tauri_plugin_global_shortcut::Shortcut>()\n    .map_err(|e| format!(\"Invalid shortcut '{shortcut}': {e}\"))?;\n```\n\n---\n\n## Frontend Event Handling\n\n### Error State via Events (Recommended)\n\nFor operations that emit state changes (like recording), errors are propagated via the `recording-state-changed` event, not via command return values:\n\n```rust\n// In Rust - commands return () and errors are emitted as events\nemit_recording_state(&app, RecordingState::Error, Some(error_message));\n```\n\n```typescript\n// In React (PillApp.tsx)\nuseTauriEvent<RecordingStatePayload>(\"recording-state-changed\", (event) => {\n  setRecordingState(event.payload.state);\n  setRecordingError(event.payload.error ?? null);\n});\n```\n\nThe `RecordingStatePayload` type includes an optional error field:\n\n```typescript\ninterface RecordingStatePayload {\n  state: RecordingState;\n  error?: string;\n}\n```\n\n### Listening to Other Error Events\n\n```typescript\nimport { useTauriEvent } from \"./hooks/use-throttled-event\";\n\n// For transcription completion\nuseTauriEvent<TranscriptionResult>(\"transcription-completed\", (event) => {\n  setLastTranscription(event.payload);\n});\n\n// For output results (includes success/failure)\nuseTauriEvent<OutputResultPayload>(\"output-result\", (event) => {\n  setLastOutputResult(event.payload);\n  if (!event.payload.success) {\n    console.error(\"Output failed:\", event.payload.error);\n  }\n});\n```\n\n---\n\n## Frontend Command Error Handling\n\n### Simple fire-and-forget (non-critical)\n\n```typescript\n// UI updates via events, command errors are logged\nstopRecording().catch(console.error);\n```\n\n### With UI feedback (critical operations)\n\n```typescript\n// History.tsx pattern\ntry {\n  const result = await deleteSession(sessionId);\n  // Success handling\n} catch (e) {\n  // Show error to user\n  console.error(\"Failed to delete session:\", e);\n}\n```\n\n### With optional fallback\n\n```typescript\n// SettingsPage.tsx pattern\ngetAutostart().then(setAutostartState).catch(() => {\n  // Default state on error\n});\n```\n\n---\n\n## User-Friendly Error Messages\n\nThe backend provides a `user_friendly_message()` function that maps raw error strings to short, user-friendly messages suitable for display in the UI:\n\n```rust\nuse crate::errors::user_friendly_message;\n\n// In orchestrator.rs\nErr(e) => {\n    let friendly = user_friendly_message(&e.to_string());\n    emit_recording_state(&app, RecordingState::Error, Some(friendly));\n}\n```\n\n### Mapping Rules\n\n| Error Pattern | User Message |\n|-------------|--------------|\n| \"no model\" / \"model not found\" | \"No model installed. Open Models to download one.\" |\n| whisper-cli / whisper sidecar | \"Transcription failed. Make sure a model is installed and working.\" |\n| audio / microphone / cpal | \"Microphone not accessible. Check your audio settings.\" |\n| disk / no space / storage | \"Not enough disk space to record audio.\" |\n| (other) | Truncated to 120 chars |\n\nThe frontend receives these pre-formatted messages in `recordingStatePayload.error`.\n\n---\n\n## Summary of Patterns\n\n| Scenario | Backend Pattern | Frontend Pattern |\n|----------|----------------|------------------|\n| Command returning data | `CmdResult<T>` with `?` | `try/catch` or `.then()/.catch()` |\n| State-changing async ops | Emit `recording-state-changed` event | Listen via `useTauriEvent` |\n| Non-critical UI updates | Silent `.catch()` | `.catch(console.error)` |\n| Displayable errors | Use `user_friendly_message()` | Show in `error` state of pill |\n\n---\n\n## Adding New Error Types\n\nWhen adding support for new error sources:\n\n1. **For library errors**: Ensure the type implements `Into<AppError>` or add a `From` implementation in `errors/mod.rs`\n2. **For user-facing errors**: Consider using `user_friendly_message()` for display\n3. **For async operations**: Prefer event-based error reporting over command return values\n4. **For validation errors**: Return early with a descriptive error message\n\nExample adding a new `From` implementation:\n\n```rust\nimpl From<std::io::Error> for AppError {\n    fn from(e: std::io::Error) -> Self {\n        AppError(e.to_string())\n    }\n}\n```\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\error-handling.md"
}
Now let me update the dev documentation index to include this new file:

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md",
  "newString": "## Reference\n\n- [architecture.md](architecture.md) — high-level system overview *(todo)*\n- [database-schema.md](database-schema.md) — full schema with field descriptions *(todo)*\n- [error-handling.md](error-handling.md) — Rust error conventions and frontend event patterns\n- [tauri-commands.md](tauri-commands.md) — all Tauri command signatures *(todo)*\n- [transcription-pipeline.md](transcription-pipeline.md) — whisper.cpp sidecar pipeline *(todo)*",
  "oldString": "## Reference\n\n- [architecture.md](architecture.md) — high-level system overview *(todo)*\n- [database-schema.md](database-schema.md) — full schema with field descriptions *(todo)*\n- [tauri-commands.md](tauri-commands.md) — all Tauri command signatures *(todo)*\n- [transcription-pipeline.md](transcription-pipeline.md) — whisper.cpp sidecar pipeline *(todo)*"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md",
  "newString": "## Reference\n\n- [architecture.md](architecture.md) — high-level system overview *(todo)*\n- [database-schema.md](database-schema.md) — full schema with field descriptions *(todo)*\n- [error-handling.md](error-handling.md) — Rust error conventions and frontend event patterns\n- [tauri-commands.md](tauri-commands.md) — all Tauri command signatures *(todo)*\n- [transcription-pipeline.md](transcription-pipeline.md) — whisper.cpp sidecar, protocol, debugging *(done)*",
  "oldString": "## Reference\n\n- [architecture.md](architecture.md) — high-level system overview *(todo)*\n- [database-schema.md](database-schema.md) — full schema with field descriptions *(todo)*\n- [tauri-commands.md](tauri-commands.md) — all Tauri command signatures *(todo)*\n- [transcription-pipeline.md](transcription-pipeline.md) — whisper.cpp sidecar, protocol, debugging *(done)*"
}
Let me verify the documentation was created correctly and check for any lint issues:

🔧 Tool: bash
Input: {
  "command": "cd /c/Users/Maik\\ Roland\\ Damm/Entwicklung/localvoice && npm run lint 2>&1 | head -50",
  "description": "Run lint to verify project health"
}

🔧 Tool: bash
Input: {
  "command": "npm run lint",
  "description": "Run lint to verify project health",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "npm run",
  "description": "List available npm scripts",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\error-handling.md"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\index.md"
}

🔧 Tool: bash
Input: {
  "command": "npm run build",
  "description": "Build project to verify TypeScript compiles",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}
The build succeeded. Now let me create a Playwright test to verify the documentation was created correctly:

🔧 Tool: bash
Input: {
  "command": "npx playwright --version 2>&1 || echo \"Playwright not installed\"",
  "description": "Check if Playwright is available",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: write
Input: {
  "content": "import { test, expect } from \"@playwright/test\";\nimport * as fs from \"fs\";\nimport * as path from \"path\";\n\ntest.describe(\"Error Handling Documentation\", () => {\n  test(\"error-handling.md exists and contains required sections\", () => {\n    const docsDir = path.join(process.cwd(), \"docs\", \"dev\");\n    const docPath = path.join(docsDir, \"error-handling.md\");\n\n    expect(fs.existsSync(docPath), \"error-handling.md should exist\").toBe(true);\n\n    const content = fs.readFileSync(docPath, \"utf-8\");\n\n    expect(content).toContain(\"# Error Handling Conventions\");\n    expect(content).toContain(\"## Rust Backend Error Types\");\n    expect(content).toContain(\"AppError\");\n    expect(content).toContain(\"CmdResult\");\n    expect(content).toContain(\"## Creating Errors\");\n    expect(content).toContain(\"## Propagating Errors\");\n    expect(content).toContain(\"## Frontend Event Handling\");\n    expect(content).toContain(\"## Frontend Command Error Handling\");\n    expect(content).toContain(\"## User-Friendly Error Messages\");\n    expect(content).toContain(\"useTauriEvent\");\n    expect(content).toContain(\"recording-state-changed\");\n  });\n\n  test(\"error-handling.md is linked from index\", () => {\n    const indexPath = path.join(process.cwd(), \"docs\", \"dev\", \"index.md\");\n    const indexContent = fs.readFileSync(indexPath, \"utf-8\");\n\n    expect(indexContent).toContain(\"error-handling.md\");\n    expect(indexContent).toContain(\"Rust error conventions and frontend event patterns\");\n  });\n});\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\tests\\error-handling-docs.spec.ts"
}

🔧 Tool: bash
Input: {
  "command": "npm ls @playwright/test 2>&1 || npm ls playwright 2>&1",
  "description": "Check Playwright package installation",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "rm tests/error-handling-docs.spec.ts",
  "description": "Remove temporary test file",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: bash
Input: {
  "command": "ls docs/dev/*.md",
  "description": "Verify documentation files exist",
  "workdir": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}
<summary>
## Summary: Document the project's conventions for Rust error handling

### Changes Implemented
- Created `docs/dev/error-handling.md` with comprehensive documentation covering:
  - `AppError` struct and `CmdResult<T>` type alias patterns
  - Creating errors from strings and library errors
  - Propagating errors with `?` operator and `.map_err(Into::into)`
  - Frontend event handling via `recording-state-changed` and `useTauriEvent` hook
  - Frontend command error handling patterns (fire-and-forget, try/catch, optional fallback)
  - User-friendly error message mapping via `user_friendly_message()`
  - Pattern summary table
  - Guide for adding new error types
- Updated `docs/dev/index.md` to link to the new documentation

### Files Modified
- `docs/dev/error-handling.md` (created)
- `docs/dev/index.md` (updated reference section)

### Notes for Developer
- Documentation is based on patterns observed in existing codebase (`src-tauri/src/errors/mod.rs`, `commands/*.rs`, `PillApp.tsx`)
- Frontend event handling uses the `useTauriEvent` and `useThrottledEvent` hooks from `src/hooks/use-throttled-event.ts`
- Errors for async state-changing operations (recording/transcription) are propagated via events, not command return values
- TypeScript build passes successfully
</summary>