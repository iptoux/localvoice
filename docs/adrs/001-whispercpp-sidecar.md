# ADR-001: whisper.cpp as Sidecar Process

## Status
Accepted

## Context
The application requires local speech-to-text transcription. whisper.cpp provides high-quality local transcription but can be integrated in two ways:
- **FFI/Bindings**: Direct Rust bindings to the whisper.cpp C++ library using `bindgen` or `cxx`
- **Sidecar Process**: Spawn whisper.cpp as a separate child process and communicate via stdin/stdout

## Decision
Use whisper.cpp as a **sidecar process** (child process), not as an FFI binding.

## Consequences

### Positive
- **Build simplicity**: No complex C++ compilation or binding generation during Rust build
- **Isolation**: Transcription crashes do not crash the main Tauri application
- **Flexibility**: Easy to swap, upgrade, or debug whisper.cpp independently
- **Language agnostic**: Can use any whisper.cpp binary regardless of how it was compiled
- **Simpler dependencies**: No need for C++ toolchain integration in Rust compilation

### Negative
- **Latency**: IPC overhead for stdin/stdout communication (minimal for batch transcription)
- **Process management**: Need to handle child process lifecycle, spawn, and cleanup
- **No real-time**: Cannot stream audio to whisper for partial results (acceptable for MVP)

### Trade-offs
- Accept slight IPC overhead in exchange for significantly simpler build system
- Forgo real-time streaming transcription initially (future enhancement)