
🔧 Tool: glob
Input: {
  "pattern": "docs/adrs/**/*.md"
}

🔧 Tool: glob
Input: {
  "pattern": "docs/**/*"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\plan\\flowdict_prd.md"
}

🔧 Tool: bash
Input: {
  "command": "mkdir -p \"C:/Users/Maik Roland Damm/Entwicklung/localvoice/docs/adrs\"",
  "description": "Create docs/adrs directory"
}

🔧 Tool: bash
Input: {
  "command": "mkdir \"C:/Users/Maik Roland Damm/Entwicklung/localvoice/docs/adrs\"",
  "description": "Create docs/adrs directory"
}

🔧 Tool: glob
Input: {
  "pattern": "docs/adrs/*"
}

🔧 Tool: write
Input: {
  "content": "# ADR-001: whisper.cpp as Sidecar Process\n\n## Status\nAccepted\n\n## Context\nThe application requires local speech-to-text transcription. whisper.cpp provides high-quality local transcription but can be integrated in two ways:\n- **FFI/Bindings**: Direct Rust bindings to the whisper.cpp C++ library using `bindgen` or `cxx`\n- **Sidecar Process**: Spawn whisper.cpp as a separate child process and communicate via stdin/stdout\n\n## Decision\nUse whisper.cpp as a **sidecar process** (child process), not as an FFI binding.\n\n## Consequences\n\n### Positive\n- **Build simplicity**: No complex C++ compilation or binding generation during Rust build\n- **Isolation**: Transcription crashes do not crash the main Tauri application\n- **Flexibility**: Easy to swap, upgrade, or debug whisper.cpp independently\n- **Language agnostic**: Can use any whisper.cpp binary regardless of how it was compiled\n- **Simpler dependencies**: No need for C++ toolchain integration in Rust compilation\n\n### Negative\n- **Latency**: IPC overhead for stdin/stdout communication (minimal for batch transcription)\n- **Process management**: Need to handle child process lifecycle, spawn, and cleanup\n- **No real-time**: Cannot stream audio to whisper for partial results (acceptable for MVP)\n\n### Trade-offs\n- Accept slight IPC overhead in exchange for significantly simpler build system\n- Forgo real-time streaming transcription initially (future enhancement)",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\adrs\\001-whispercpp-sidecar.md"
}

🔧 Tool: write
Input: {
  "content": "# ADR-002: Dual-Window Design (Pill + Main Window)\n\n## Status\nAccepted\n\n## Context\nThe application serves two distinct use cases:\n1. **Quick voice input**: Fast, always-accessible recording trigger with minimal UI\n2. **Full application**: Dashboard, history, dictionary, models, and settings management\n\nA single window would require either:\n- Being too large by default (unobtrusive goal violated)\n- Being too small to contain all features (insufficient functionality)\n\n## Decision\nImplement two separate windows with distinct purposes:\n\n1. **Pill Window** (default, always-on-top)\n   - Small floating capsule UI\n   - Shows current voice state (idle/listening/processing/success/error)\n   - Always quickly accessible\n   - Minimal visual footprint on desktop\n   - Draggable, position persisted\n\n2. **Main Window** (on-demand)\n   - Full application interface\n   - Sidebar navigation: Dashboard, History, Dictionary, Models, Settings\n   - Standard window controls (close, minimize, maximize)\n   - Size and position persisted\n\n## Consequences\n\n### Positive\n- **Minimal default footprint**: User's desktop is not cluttered when app is idle\n- **Clear state visibility**: Pill provides instant visual feedback on recording status\n- **Progressive disclosure**: Full features available when needed, hidden when not\n- **Multiple navigation paths**: Context menu, pill click, and tray icon provide access\n- **Flexibility**: Pill can be moved anywhere; main window has standard behavior\n\n### Negative\n- **Two window states to manage**: Position, visibility, and lifecycle complexity\n- **Communication between windows**: Frontend must coordinate state across windows\n- **Platform-specific window behavior**: Different handling for always-on-top across OSes\n\n### Trade-offs\n- Accept window management complexity in exchange for superior UX\n- Use Tauri v2's multiwindow capabilities to handle the complexity",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\adrs\\002-dual-window-design.md"
}

🔧 Tool: write
Input: {
  "content": "# ADR-003: SQLite for Local Data Persistence\n\n## Status\nAccepted\n\n## Context\nThe application requires local storage for:\n- Session history (transcripts, metadata)\n- Dictionary entries and correction rules\n- Ambiguous terms tracking\n- Model installations registry\n- User settings\n\nOptions considered:\n- **SQLite**: Single-file relational database\n- **JSON files**: Flat file storage\n- **Embedded KV store**: RocksDB, sled\n- **Full database server**: PostgreSQL (rejected: requires external setup)\n\n## Decision\nUse **SQLite** as the single local database for all persistence needs.\n\n## Consequences\n\n### Positive\n- **Single file**: All data in one `localvoice.db` file, easy to backup/export\n- **No setup**: Embedded, no server installation required\n- **Portable**: Works across Windows, macOS, Linux\n- **Mature**: Excellent Rust support via `rusqlite` or `sqlx`\n- **Relational**: Proper schema with foreign keys, indexes, queries\n- **ACID**: Transactions ensure data integrity\n- **Queryable**: Complex queries for history search, stats aggregation\n\n### Negative\n- **Single file limitations**: Not ideal for huge datasets (acceptable for personal use)\n- **Concurrency**: Write locking (mitigated by single-user app)\n- **No network**: Cannot share data between machines (intentional: offline-first)\n\n### Trade-offs\n- Accept single-file limitations in exchange for zero-configuration deployment\n- Use proper schema design to handle scale up to thousands of sessions",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\adrs\\003-sqlite-database.md"
}

🔧 Tool: write
Input: {
  "content": "# Architecture Decision Records (ADRs)\n\nThis directory contains ADRs documenting key architectural decisions for LocalVoice.\n\n## What is an ADR?\n\nAn Architecture Decision Record (ADR) documents a significant architectural decision made during the project. Each ADR follows a standard template:\n\n- **Status**: Proposed, Accepted, Deprecated, or Superseded\n- **Context**: The situation that prompted the decision\n- **Decision**: What was decided\n- **Consequences**: Positive, negative, and trade-offs\n\n## Index\n\n| ADR | Title | Status |\n|-----|-------|--------|\n| [001](./001-whispercpp-sidecar.md) | whisper.cpp as Sidecar Process | Accepted |\n| [002](./002-dual-window-design.md) | Dual-Window Design (Pill + Main Window) | Accepted |\n| [003](./003-sqlite-database.md) | SQLite for Local Data Persistence | Accepted |\n\n## Adding New ADRs\n\nWhen making significant architectural decisions:\n\n1. Create a new file `0XX-<title>.md` in this directory\n2. Use the standard ADR template\n3. Update this index table\n4. Link from relevant documentation in `docs/dev/` if applicable",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\adrs\\index.md"
}

🔧 Tool: glob
Input: {
  "pattern": "docs/adrs/**/*"
}


<summary>
## Summary: Architecture Decision Records (ADRs)

### Changes Implemented
- Created `docs/adrs/` directory with 4 markdown files:
  - `001-whispercpp-sidecar.md` - Documents using whisper.cpp as a sidecar process instead of FFI bindings
  - `002-dual-window-design.md` - Documents the dual-window design (pill + main window)
  - `003-sqlite-database.md` - Documents using SQLite as the single local database
  - `index.md` - Index file with ADR list and guidance for adding new ADRs

### Files Modified
- Created: `docs/adrs/index.md`
- Created: `docs/adrs/001-whispercpp-sidecar.md`
- Created: `docs/adrs/002-dual-window-design.md`
- Created: `docs/adrs/003-sqlite-database.md`

### Notes for Developer
- Each ADR follows the standard template with Context, Decision, and Consequences sections
- ADRs are numbered sequentially (001, 002, 003)
- Index file provides guidance for adding future ADRs
</summary>