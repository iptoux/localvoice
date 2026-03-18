# Error Handling Conventions

This document defines the project's conventions for Rust error handling and how frontend code should handle error events from Tauri.

## Table of Contents

1. [Rust Backend Error Types](#rust-backend-error-types)
2. [Result Type Aliases](#result-type-aliases)
3. [Creating Errors](#creating-errors)
4. [Propagating Errors](#propagating-errors)
5. [Frontend Event Handling](#frontend-event-handling)
6. [Frontend Command Error Handling](#frontend-command-error-handling)
7. [User-Friendly Error Messages](#user-friendly-error-messages)

---

## Rust Backend Error Types

All error handling is centralized in `src-tauri/src/errors/mod.rs`.

### AppError

```rust
#[derive(Debug, Serialize)]
pub struct AppError(pub String);
```

`AppError` is a simple wrapper around a `String`. It serializes to a plain string so the frontend receives a descriptive error message when a command fails.

### CmdResult

```rust
pub type CmdResult<T> = Result<T, AppError>;
```

All Tauri commands return `CmdResult<T>`.

---

## Creating Errors

### From strings

```rust
// Using .into() for implicit conversion
return Err("Recording already in progress".into());

// Explicit construction
return Err(AppError("Invalid shortcut format".to_string()));

// With format!
return Err(format!("Failed to register shortcut '{}': {}", shortcut, e).into());
```

### From library errors

The `From` trait implementations handle common error types:

```rust
// rusqlite errors automatically convert
fn get_settings(state: State<AppState>) -> CmdResult<HashMap<String, String>> {
    settings_repo::get_all(&state.db).map_err(Into::into)
}
```

For errors without automatic conversion, use `.map_err()`:

```rust
return std::fs::write(&path, content)
    .map_err(|e| AppError(format!("Failed to write export file: {e}")))?;
```

---

## Propagating Errors

### Standard propagation with `?`

```rust
#[tauri::command]
pub fn get_session(state: State<AppState>, session_id: String) -> CmdResult<SessionWithSegments> {
    // ? operator works because of From implementations
    sessions_repo::get_session(&state.db, &session_id)
}
```

### Explicit conversion with `.map_err(Into::into)`

```rust
#[tauri::command]
pub fn get_settings(state: State<AppState>) -> CmdResult<HashMap<String, String>> {
    settings_repo::get_all(&state.db).map_err(Into::into)
}
```

### Inline error creation

```rust
normalized
    .parse::<tauri_plugin_global_shortcut::Shortcut>()
    .map_err(|e| format!("Invalid shortcut '{shortcut}': {e}"))?;
```

---

## Frontend Event Handling

### Error State via Events (Recommended)

For operations that emit state changes (like recording), errors are propagated via the `recording-state-changed` event, not via command return values:

```rust
// In Rust - commands return () and errors are emitted as events
emit_recording_state(&app, RecordingState::Error, Some(error_message));
```

```typescript
// In React (PillApp.tsx)
useTauriEvent<RecordingStatePayload>("recording-state-changed", (event) => {
  setRecordingState(event.payload.state);
  setRecordingError(event.payload.error ?? null);
});
```

The `RecordingStatePayload` type includes an optional error field:

```typescript
interface RecordingStatePayload {
  state: RecordingState;
  error?: string;
}
```

### Listening to Other Error Events

```typescript
import { useTauriEvent } from "./hooks/use-throttled-event";

// For transcription completion
useTauriEvent<TranscriptionResult>("transcription-completed", (event) => {
  setLastTranscription(event.payload);
});

// For output results (includes success/failure)
useTauriEvent<OutputResultPayload>("output-result", (event) => {
  setLastOutputResult(event.payload);
  if (!event.payload.success) {
    console.error("Output failed:", event.payload.error);
  }
});
```

---

## Frontend Command Error Handling

### Simple fire-and-forget (non-critical)

```typescript
// UI updates via events, command errors are logged
stopRecording().catch(console.error);
```

### With UI feedback (critical operations)

```typescript
// History.tsx pattern
try {
  const result = await deleteSession(sessionId);
  // Success handling
} catch (e) {
  // Show error to user
  console.error("Failed to delete session:", e);
}
```

### With optional fallback

```typescript
// SettingsPage.tsx pattern
getAutostart().then(setAutostartState).catch(() => {
  // Default state on error
});
```

---

## User-Friendly Error Messages

The backend provides a `user_friendly_message()` function that maps raw error strings to short, user-friendly messages suitable for display in the UI:

```rust
use crate::errors::user_friendly_message;

// In orchestrator.rs
Err(e) => {
    let friendly = user_friendly_message(&e.to_string());
    emit_recording_state(&app, RecordingState::Error, Some(friendly));
}
```

### Mapping Rules

| Error Pattern | User Message |
|-------------|--------------|
| "no model" / "model not found" | "No model installed. Open Models to download one." |
| whisper-cli / whisper sidecar | "Transcription failed. Make sure a model is installed and working." |
| audio / microphone / cpal | "Microphone not accessible. Check your audio settings." |
| disk / no space / storage | "Not enough disk space to record audio." |
| (other) | Truncated to 120 chars |

The frontend receives these pre-formatted messages in `recordingStatePayload.error`.

---

## Summary of Patterns

| Scenario | Backend Pattern | Frontend Pattern |
|----------|----------------|------------------|
| Command returning data | `CmdResult<T>` with `?` | `try/catch` or `.then()/.catch()` |
| State-changing async ops | Emit `recording-state-changed` event | Listen via `useTauriEvent` |
| Non-critical UI updates | Silent `.catch()` | `.catch(console.error)` |
| Displayable errors | Use `user_friendly_message()` | Show in `error` state of pill |

---

## Adding New Error Types

When adding support for new error sources:

1. **For library errors**: Ensure the type implements `Into<AppError>` or add a `From` implementation in `errors/mod.rs`
2. **For user-facing errors**: Consider using `user_friendly_message()` for display
3. **For async operations**: Prefer event-based error reporting over command return values
4. **For validation errors**: Return early with a descriptive error message

Example adding a new `From` implementation:

```rust
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError(e.to_string())
    }
}
```
