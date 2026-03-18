
🔧 Tool: task
Input: {
  "description": "Explore Tauri window config",
  "prompt": "Explore the codebase to understand:\n1. How Tauri windows are configured (tauri.conf.json, window creation code)\n2. How settings/models are stored (SQLite schema, settings commands)\n3. How the app detects first run or checks for existing data\n4. Current window visibility behavior\n\nFocus on:\n- src-tauri/tauri.conf.json\n- src-tauri/src/commands/ directory (especially settings, window commands)\n- src-tauri/src/main.rs\n- src/ directory for any window management logic\n\nReturn a summary of findings including relevant file paths and code patterns.",
  "subagent_type": "explore"
}
## Specification: Show Main Window on First Run or When No Model Exists

### 1. Problem
The main window remains hidden by default, requiring users to manually open it. On first run or when no model exists, users should be automatically shown the main window (onboarding/settings) instead of being left with only the pill window.

### 2. Solution
Extend the startup window visibility logic in `lib.rs` to check for first run (no installed models) and missing default model configuration, then show the main window instead of hiding it.

### 3. Acceptance Criteria
- **GIVEN** the app starts with no installed models, **WHEN** the app initializes, **THEN** the main window is shown automatically
- **GIVEN** the app starts with installed models but no default model set, **WHEN** the app initializes, **THEN** the main window is shown automatically
- **GIVEN** the app starts with a default model configured and not first run, **WHEN** the app initializes, **THEN** the default mode (pill or main) is respected
- **GIVEN** the main window is shown automatically, **WHEN** it appears, **THEN** it is properly focused and brought to the foreground
- **GIVEN** `app.start_hidden` is true, **WHEN** the app initializes, **THEN** all windows remain hidden regardless of model state

### 4. Files to Modify
| File | Purpose | Action |
|------|---------|--------|
| `src-tauri/src/lib.rs` | Startup logic for window visibility | Modify |
| `src-tauri/src/commands/system.rs` | Add command to check if default model exists | Modify |

### 5. Implementation Tasks
```tasks
- [ ] T001: Add `has_default_model()` command to system.rs | File: src-tauri/src/commands/system.rs
- [ ] T002: Modify startup logic in lib.rs to show main window on first run or missing model | File: src-tauri/src/lib.rs
- [ ] T003: Ensure proper window focus when auto-showing main window | File: src-tauri/src/commands/window.rs
```

### 6. Verification
- Start app with no models → main window appears
- Start app with models but no default → main window appears  
- Start app with default model set → default mode respected
- Test `app.start_hidden` still works correctly

[SPEC_GENERATED] Please review the specification above. Reply with 'approved' to proceed or provide feedback for revisions.[TASK_START] T001: Add `has_default_model()` command to system.rs[TASK_COMPLETE] T001: Added `has_default_model()` command to system.rs that checks if a default model exists for the configured language (handles "auto" by checking both de/en). Also exported to TypeScript wrapper in `src/lib/tauri.ts`.[TASK_START] T002: Modify startup logic in lib.rs to show main window on first run or missing model[TASK_COMPLETE] T002: Modified startup logic in lib.rs to check for first run (no installed models) and missing default model, showing main window with focus when either condition is true (respecting `start_hidden` setting).[TASK_START] T003: Ensure proper window focus when auto-showing main windowThe `open_main_window` command already has `set_focus()` when the window exists, but missing it when creating a new window. Let me fix that:[TASK_COMPLETE] T003: Added `set_focus()` call when creating a new main window in `open_main_window` command. The startup logic already had focus set when auto-showing.<summary>
## Summary: Show Main Window on First Run or When No Model Exists

### Changes Implemented
- Added `has_default_model()` command to check if a default model exists for the configured language (handles "auto" by checking both de/en fallback)
- Modified startup logic in `lib.rs` to detect first run (no installed models) and missing default model configuration
- Main window is now auto-shown with proper focus when either condition is true, respecting the `app.start_hidden` setting
- Added `set_focus()` call when creating a new main window in `open_main_window` command
- Exported new `has_default_model` function to TypeScript wrapper

### Files Modified
- `src-tauri/src/commands/system.rs` - Added `has_default_model()` command
- `src-tauri/src/lib.rs` - Extended startup logic for first run/model detection
- `src-tauri/src/commands/window.rs` - Added focus call when creating new window
- `src/lib/tauri.ts` - Exported TypeScript wrapper for `has_default_model`

### Notes for Developer
- The feature respects `app.start_hidden` setting - if true, all windows remain hidden regardless of model state
- When auto-showing the main window, both `show()` and `set_focus()` are called to ensure proper foregrounding
- Log output indicates "Starting in main window mode (onboarding)" when auto-showing due to first run or missing model
</summary>