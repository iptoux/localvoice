# ADR-002: Dual-Window Design (Pill + Main Window)

## Status
Accepted

## Context
The application serves two distinct use cases:
1. **Quick voice input**: Fast, always-accessible recording trigger with minimal UI
2. **Full application**: Dashboard, history, dictionary, models, and settings management

A single window would require either:
- Being too large by default (unobtrusive goal violated)
- Being too small to contain all features (insufficient functionality)

## Decision
Implement two separate windows with distinct purposes:

1. **Pill Window** (default, always-on-top)
   - Small floating capsule UI
   - Shows current voice state (idle/listening/processing/success/error)
   - Always quickly accessible
   - Minimal visual footprint on desktop
   - Draggable, position persisted

2. **Main Window** (on-demand)
   - Full application interface
   - Sidebar navigation: Dashboard, History, Dictionary, Models, Settings
   - Standard window controls (close, minimize, maximize)
   - Size and position persisted

## Consequences

### Positive
- **Minimal default footprint**: User's desktop is not cluttered when app is idle
- **Clear state visibility**: Pill provides instant visual feedback on recording status
- **Progressive disclosure**: Full features available when needed, hidden when not
- **Multiple navigation paths**: Context menu, pill click, and tray icon provide access
- **Flexibility**: Pill can be moved anywhere; main window has standard behavior

### Negative
- **Two window states to manage**: Position, visibility, and lifecycle complexity
- **Communication between windows**: Frontend must coordinate state across windows
- **Platform-specific window behavior**: Different handling for always-on-top across OSes

### Trade-offs
- Accept window management complexity in exchange for superior UX
- Use Tauri v2's multiwindow capabilities to handle the complexity