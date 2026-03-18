Let me check the current state of the implementation to continue from where we left off.

🔧 Tool: glob
Input: {
  "pattern": "**/*event*"
}

🔧 Tool: glob
Input: {
  "pattern": "**/contracts/**"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\plan\\tasks"
}

🔧 Tool: grep
Input: {
  "pattern": "event.*contract|contract.*registry|EventContract"
}

🔧 Tool: glob
Input: {
  "pattern": "**/src/**/*.ts"
}

🔧 Tool: glob
Input: {
  "pattern": "**/src-tauri/src/**/*.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\lib\\tauri.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\hooks\\use-throttled-event.ts"
}

🔧 Tool: grep
Input: {
  "include": "*.ts",
  "pattern": "emit\\(|listen\\("
}

🔧 Tool: grep
Input: {
  "include": "*.rs",
  "pattern": "emit\\("
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\types\\index.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\state\\app_state.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\models\\downloader.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\transcription\\orchestrator.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\commands\\recording.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\commands\\history.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\audio\\capture.rs",
  "limit": 400
}

🔧 Tool: grep
Input: {
  "include": "*.md",
  "pattern": "feature-1773848074442-tet3lqw02"
}

🔧 Tool: glob
Input: {
  "pattern": "**/contracts/**"
}

🔧 Tool: glob
Input: {
  "pattern": "**/*contract*"
}

🔧 Tool: glob
Input: {
  "pattern": "src/lib/**/*.ts"
}

🔧 Tool: grep
Input: {
  "include": "*.ts",
  "pattern": "RecordingStatePayload|OutputResult|TranscriptionResult|DownloadProgress"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\app-store.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\models-store.ts"
}
Now I understand the codebase. I'll create the Event Contract Registry as a centralized source of truth for all Tauri events.

🔧 Tool: write
Input: {
  "content": "import type {\n  DownloadProgress as TDownloadProgress,\n  OutputResult,\n  RecordingState,\n  RecordingStatePayload,\n  TranscriptionResult,\n} from \"../types\";\n\n/**\n * Centralized registry of all Tauri event channels used in LocalVoice.\n *\n * This file is the single source of truth for:\n * - Event channel names\n * - TypeScript payload types\n * - Usage examples\n *\n * ## Adding a new event\n * 1. Add the event definition here with channel name, payload type, and documentation.\n * 2. Export the channel name constant for use in components.\n * 3. Use `useThrottledEvent` or `useTauriEvent` hooks with the event type.\n */\nexport const EventChannels = {\n  /** Pill recording state transitions (idle → listening → processing → success/error). */\n  RECORDING_STATE_CHANGED: \"recording-state-changed\",\n\n  /** Real-time RMS audio level during recording (0-1 range, throttled ~80ms). */\n  AUDIO_LEVEL: \"audio-level\",\n\n  /** Result of the output step (clipboard write or text insertion). */\n  OUTPUT_RESULT: \"output-result\",\n\n  /** Fired when a full transcription pipeline completes successfully or with error. */\n  TRANSCRIPTION_COMPLETED: \"transcription-completed\",\n\n  /** Fired when silence timeout is detected during recording. */\n  SILENCE_DETECTED: \"silence-detected\",\n\n  /** Fired after a session is re-transcribed with updated language/model. */\n  SESSION_REPROCESSED: \"session-reprocessed\",\n\n  /** Download progress for model installations (throttled to 1% changes). */\n  MODEL_DOWNLOAD_PROGRESS: \"model-download-progress\",\n\n  /** Frontend-to-frontend navigation event (pill → main window). */\n  NAVIGATE_TO: \"navigate-to\",\n} as const;\n\nexport type EventChannel = (typeof EventChannels)[keyof typeof EventChannels];\n\n/**\n * Payload types for all registered events.\n * Use these types with `useThrottledEvent<T>` or `listen<T>()`.\n */\nexport interface EventPayloads {\n  [EventChannels.RECORDING_STATE_CHANGED]: RecordingStatePayload;\n  [EventChannels.AUDIO_LEVEL]: number;\n  [EventChannels.OUTPUT_RESULT]: OutputResult;\n  [EventChannels.TRANSCRIPTION_COMPLETED]: TranscriptionResult;\n  [EventChannels.SILENCE_DETECTED]: void;\n  [EventChannels.SESSION_REPROCESSED]: string;\n  [EventChannels.MODEL_DOWNLOAD_PROGRESS]: TDownloadProgress;\n  [EventChannels.NAVIGATE_TO]: string;\n}\n\n/**\n * Type-safe event listener that infers the payload type from the channel name.\n *\n * @example\n * ```ts\n * import { listenEvent, EventChannels } from \"./lib/events\";\n *\n * // Automatically infers payload as RecordingStatePayload\n * listenEvent(EventChannels.RECORDING_STATE_CHANGED, (payload) => {\n *   console.log(\"Recording state:\", payload.state);\n * });\n * ```\n */\nexport async function listenEvent<C extends EventChannel>(\n  channel: C,\n  handler: (payload: EventPayloads[C]) => void\n): Promise<() => void> {\n  const { listen } = await import(\"@tauri-apps/api/event\");\n  const unlisten = await listen<EventPayloads[C]>(channel, (event) => {\n    handler(event.payload);\n  });\n  return unlisten;\n}\n\n/**\n * Type-safe throttled event listener for high-frequency events.\n * Use for audio-level, download-progress, etc.\n *\n * @example\n * ```ts\n * import { listenThrottledEvent, EventChannels } from \"./lib/events\";\n *\n * listenThrottledEvent(EventChannels.AUDIO_LEVEL, (rms) => {\n *   setAudioLevel(rms);\n * });\n * ```\n */\nexport async function listenThrottledEvent<C extends EventChannel>(\n  channel: C,\n  handler: (payload: EventPayloads[C]) => void\n): Promise<() => void> {\n  const { listen } = await import(\"@tauri-apps/api/event\");\n  const { rafThrottle } = await import(\"./raf-throttle\");\n\n  const throttled = rafThrottle<EventPayloads[C]>((value) => {\n    handler(value);\n  });\n\n  const unlisten = await listen<EventPayloads[C]>(channel, (event) => {\n    throttled.update(event.payload);\n  });\n\n  return () => {\n    throttled.cancel();\n    unlisten();\n  };\n}\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\lib\\events.ts"
}

🔧 Tool: edit
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\lib\\events.ts",
  "newString": "import type {\n  DownloadProgress as TDownloadProgress,\n  OutputResult,\n  RecordingStatePayload,\n  TranscriptionResult,\n} from \"../types\";",
  "oldString": "import type {\n  DownloadProgress as TDownloadProgress,\n  OutputResult,\n  RecordingState,\n  RecordingStatePayload,\n  TranscriptionResult,\n} from \"../types\";"
}

🔧 Tool: bash
Input: {
  "command": "npm run lint 2>&1 | head -50",
  "description": "Run TypeScript lint check"
}

🔧 Tool: bash
Input: {
  "command": "npm run 2>&1",
  "description": "List available npm scripts"
}

🔧 Tool: bash
Input: {
  "command": "npx tsc --noEmit 2>&1",
  "description": "Run TypeScript type check"
}

🔧 Tool: bash
Input: {
  "command": "npx tsc --noEmit src/lib/events.ts 2>&1",
  "description": "Type check the new events file"
}
The Event Contract Registry has been created. Let me verify it works correctly:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\lib\\events.ts"
}
<summary>
## Summary: Frontend-Backend Event Contract Registry

### Changes Implemented
- Created centralized `src/lib/events.ts` with:
  - `EventChannels` const object with all 8 Tauri event channel names (recording-state-changed, audio-level, output-result, transcription-completed, silence-detected, session-reprocessed, model-download-progress, navigate-to)
  - `EventPayloads` interface mapping each channel to its TypeScript payload type
  - `EventChannel` type for type-safe channel name inference
  - `listenEvent()` helper for low-frequency events with automatic type inference
  - `listenThrottledEvent()` helper for high-frequency events (audio level, download progress) with RAF throttling

### Files Modified
- `src/lib/events.ts` (new file - 120 lines)

### Notes for Developer
- All event payloads reference existing types from `src/types/index.ts`
- Channel names match exactly what Rust emits in: `app_state.rs:58`, `audio/capture.rs:354`, `transcription/orchestrator.rs:240,387`, `commands/recording.rs:76`, `commands/history.rs:88`, `models/downloader.rs:79`
- Pre-existing TypeScript errors in `src/i18n/index.ts` and `src/mocks/tauri.ts` are unrelated to this feature
</summary>