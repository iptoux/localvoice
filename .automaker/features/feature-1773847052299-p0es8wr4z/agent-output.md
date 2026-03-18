I'll explore the codebase to understand the architecture before creating the diagram.

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\plan\\flowdict_prd.md"
}

🔧 Tool: glob
Input: {
  "pattern": "src-tauri/src/**/*.rs"
}

🔧 Tool: glob
Input: {
  "pattern": "src/**/*.{ts,tsx}"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\state\\app_state.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\state\\recording_state.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\stores\\app-store.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\lib\\tauri.ts"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\commands\\recording.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src-tauri\\src\\transcription\\orchestrator.rs"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\components\\pill\\Pill.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\MainApp.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\PillApp.tsx"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\src\\types\\index.ts"
}

🔧 Tool: glob
Input: {
  "pattern": "docs/**/*"
}
Now I have a complete picture of the state flow. Let me create the Mermaid diagram in `docs/dev/`.

🔧 Tool: write
Input: {
  "content": "# State Flow Architecture\n\nThis document shows how state flows between Rust AppState, Tauri events/commands, Zustand stores, and React components.\n\n## Overview\n\nLocalVoice uses a dual-language architecture with two state synchronization patterns:\n\n| Pattern | Mechanism | Direction | Latency |\n|---------|-----------|-----------|---------|\n| **Synchronous** | `invoke()` command | Frontend → Rust | ~ms (blocking) |\n| **Asynchronous** | Tauri `emit()` event | Rust → Frontend | ~ms (non-blocking) |\n\n## State Flow Diagram\n\n```mermaid\nflowchart TB\n    subgraph Rust[\"Rust Backend (src-tauri/src)\"]\n        AppState[\"AppState<br/>(Arc&lt;Mutex&lt;...&gt;&gt;)\"]\n        \n        subgraph AppState_fields[\"AppState Fields\"]\n            RS[\"recording_state: RecordingState<br/>(Idle|Listening|Processing|Success|Error)\"]\n            AR[\"active_recording: Option&lt;ActiveRecording&gt;<br/>(cpal Stream + samples)\"]\n            WAV[\"last_wav_path: Option&lt;String&gt;\"]\n            TR[\"last_transcription: Option&lt;TranscriptionResult&gt;\"]\n            RSA[\"recording_started_at: Option&lt;DateTime&gt;\"]\n        end\n        \n        subgraph Commands[\"Tauri Commands (#[tauri::command])\"]\n            REC[\"recording.rs<br/>start_recording / stop_recording / cancel_recording\"]\n            TRANS[\"transcription.rs<br/>transcribe_last_recording / reprocess_session\"]\n            HIST[\"history.rs<br/>list_sessions / get_session / delete_session\"]\n            DICT[\"dictionary.rs<br/>list_correction_rules / create_correction_rule / ...\"]\n            MOD[\"models.rs<br/>list_available_models / download_model / ...\"]\n            SETS[\"settings.rs<br/>get_settings / update_setting / ...\"]\n            SYST[\"system.rs / window.rs / filler_words.rs / stats.rs\"]\n        end\n        \n        subgraph BackendModules[\"Internal Modules (not exposed)\"]\n            AUD[\"audio/capture.rs<br/>cpal microphone capture\"]\n            WHIS[\"transcription/whisper_sidecar.rs<br/>whisper.cpp sidecar process\"]\n            POST[\"postprocess/pipeline.rs<br/>dictionary rules, fillers, normalization\"]\n            DB[\"db/repositories/*<br/>SQLite persistence\"]\n        end\n        \n        Events[\"emit_recording_state()<br/>emit() → Tauri Events\"]\n    end\n    \n    subgraph TauriEvents[\"Tauri Event Bus\"]\n        RSE[\"recording-state-changed<br/>RecordingStatePayload\"]\n        TL[\"audio-level (throttled)<br/>number (0-1 RMS)\"]\n        TC[\"transcription-completed<br/>TranscriptionResult\"]\n        OR[\"output-result<br/>OutputResultPayload\"]\n        THM[\"theme-changed<br/>Theme\"]\n        NAV[\"navigate-to<br/>string\"]\n        SD[\"silence-detected<br/>unit\"]\n    end\n    \n    subgraph Frontend[\"React Frontend (src/)\"]\n        subgraph Stores[\"Zustand Stores\"]\n            APP[\"app-store.ts<br/>recordingState, audioLevel, lastTranscription,<br/>lastOutputResult, isPillExpanded\"]\n            SETS_F[\"settings-store.ts<br/>settings cache\"]\n            HIST_F[\"history-store.ts<br/>sessions list + detail\"]\n            DICT_F[\"dictionary-store.ts<br/>entries, rules, ambiguous terms\"]\n            MOD_F[\"models-store.ts<br/>available + installed models\"]\n            DASH[\"dashboard-store.ts<br/>stats, timeseries, language breakdown\"]\n            FILL[\"filler-words-store.ts<br/>filler words + stats\"]\n            AMB[\"ambiguity-store.ts<br/>ambiguous terms\"]\n        end\n        \n        subgraph Components[\"React Components\"]\n            PILL[\"Pill.tsx<br/>listens to recording-state-changed,<br/>audio-level, transcription-completed\"]\n            EXP[\"ExpandedPill.tsx<br/>latest transcript, quick actions\"]\n            MAIN[\"MainApp.tsx<br/>listen to navigate-to, theme-changed\"]\n            HIST_P[\"History.tsx<br/>listSessions, getSessionDetail, deleteSession\"]\n            DICT_P[\"Dictionary.tsx<br/>dictionary CRUD via invoke()\"]\n            MOD_P[\"Models.tsx<br/>model list + download via invoke()\"]\n            SET_P[\"SettingsPage.tsx<br/>settings via invoke(), listInputDevices\"]\n            DASH_P[\"Dashboard.tsx<br/>getDashboardStats, getUsageTimeseries\"]\n            SB[\"Sidebar.tsx<br/>navigation\"]\n        end\n    end\n    \n    subgraph External[\"External Systems\"]\n        MIC[\"Microphone<br/>(cpal)\"]\n        WHISPER[\"whisper.cpp<br/>(sidecar process)\"]\n        FS[\"Filesystem<br/>(WAV, models, DB)\"]\n        CB[\"System Clipboard\"]\n    end\n\n    %% Recording Flow\n    REC -->|start| AUD\n    REC -->|set| RS\n    REC -->|set| AR\n    REC -->|emit| RSE\n    AUD -->|audio samples| AR\n    AUD -->|audio-level| TL\n    \n    REC -->|stop| WAV\n    REC -->|Processing| RSE\n    \n    %% Transcription Flow\n    TRANS -->|read| WAV\n    TRANS -->|invoke| WHISPER\n    WHISPER -->|stdout JSON| TRANS\n    TRANS -->|run| POST\n    POST -->|apply rules| DB\n    POST -->|store| TR\n    TRANS -->|emit| TC\n    TRANS -->|emit| RSE\n    POST -->|write| CB\n    \n    %% AppState mutations\n    Events -->|update| RS\n    REC -->|modify| AppState\n    TRANS -->|modify| AppState\n    \n    %% Event bus to frontend\n    RSE -->|listen| APP\n    TL -->|throttled listen| APP\n    TC -->|listen| APP\n    OR -->|listen| APP\n    THM -->|listen| MAIN\n    NAV -->|listen| MAIN\n    \n    %% Stores to components\n    APP -->|useAppStore| PILL\n    APP -->|useAppStore| EXP\n    APP -->|useAppStore| SET_P\n    \n    %% Invoke commands (sync)\n    PILL -->|\"invoke(startRecording)\"| REC\n    PILL -->|\"invoke(stopRecording)\"| REC\n    PILL -->|\"invoke(cancelRecording)\"| REC\n    \n    SET_P -->|\"invoke(listInputDevices)\"| REC\n    SET_P -->|\"invoke(getSettings)\"| SETS\n    SET_P -->|\"invoke(updateSetting)\"| SETS\n    \n    HIST_P -->|\"invoke(listSessions)\"| HIST\n    HIST_P -->|\"invoke(getSessionDetail)\"| HIST\n    HIST_P -->|\"invoke(deleteSession)\"| HIST\n    \n    DICT_P -->|\"invoke(listDictionaryEntries)\"| DICT\n    DICT_P -->|\"invoke(createCorrectionRule)\"| DICT\n    DICT_P -->|\"invoke(updateCorrectionRule)\"| DICT\n    \n    MOD_P -->|\"invoke(listAvailableModels)\"| MOD\n    MOD_P -->|\"invoke(downloadModel)\"| MOD\n    MOD_P -->|\"invoke(deleteModel)\"| MOD\n    \n    DASH_P -->|\"invoke(getDashboardStats)\"| SYST\n    DASH_P -->|\"invoke(getUsageTimeseries)\"| SYST\n    \n    %% External interactions\n    AUD -->|input| MIC\n    WHISPER -->|read WAV| FS\n    REC -->|write temp WAV| FS\n    POST -->|write| CB\n    \n    %% Legend\n    subgraph Legend[\"Legend\"]\n        L1[\"invoke() — synchronous command\"]\n        L2[\"emit() → listen — asynchronous event\"]\n        L3[\"direct state access\"]\n    end\n```\n\n## Recording State Transitions\n\n```mermaid\nstateDiagram-v2\n    [*] --> Idle: App Start\n    Idle --> Listening: startRecording()\n    Listening --> Processing: stopRecording()\n    Listening --> Idle: cancelRecording() / silence timeout\n    Processing --> Success: transcription complete\n    Processing --> Error: transcription failed\n    Success --> Idle: 2s auto-reset\n    Error --> Idle: 3s auto-reset\n    \n    note right of Listening: cpal stream active<br/>audio-level events emitted\n    note right of Processing: whisper.cpp running<br/>background thread\n    note right of Success: clipboard written<br/>session persisted to DB\n```\n\n## Sync vs Async Communication Patterns\n\n### Synchronous (invoke commands) — Query & Control\n\nUsed when the frontend needs to:\n- Trigger an action and wait for result\n- Query persistent data (settings, history, models)\n\n```\nReact Component → invoke(command) → Rust Command Handler\n                                              ↓\n                                    SQLite / System Call\n                                              ↓\nReact Component ← Promise resolved ← Return value\n```\n\n**Example:** `startRecording()` → `stopRecording()` → WAV path returned\n\n### Asynchronous (emit events) — Reactive State\n\nUsed when Rust needs to push updates to the UI:\n- High-frequency updates (audio level)\n- State transitions initiated by backend\n- Background task completion\n\n```\nRust Backend: emit(event, payload) → Tauri Event Bus → All Windows\n\nEach Window:\n  listen(event, handler) → update Zustand store → React re-render\n```\n\n**Example:** `emit_recording_state(app, Listening)` → Pill updates immediately\n\n## Audio Level Flow (High-Frequency)\n\n```mermaid\nsequenceDiagram\n    participant AUD as audio/capture.rs<br/>(cpal callback)\n    participant APP as AppState\n    participant BUS as Tauri Event Bus\n    participant PILL as PillApp.tsx\n    participant ZUST as app-store.ts\n    participant UI as Pill.tsx\n    \n    loop Every audio buffer (~10ms)\n        AUD->>AUD: compute RMS level\n        AUD->>APP: app.emit(\"audio-level\", level)\n        Note over APP: Throttled in frontend<br/>via useThrottledEvent\n        BUS->>PILL: audio-level event\n        PILL->>ZUST: setAudioLevel(level)\n        ZUST->>UI: re-render Waveform\n    end\n```\n\n## Transcription Flow (Background Task)\n\n```mermaid\nsequenceDiagram\n    participant UI as Pill / invoke()\n    participant REC as recording.rs\n    participant CAP as capture.rs\n    participant ORCH as orchestrator.rs\n    participant WHIS as whisper_sidecar.rs\n    participant POST as postprocess/pipeline.rs\n    participant DB as SQLite\n    participant CLIP as clipboard\n    participant ZUST as app-store\n    participant EVT as Tauri Events\n    \n    UI->>REC: invoke(\"start_recording\")\n    REC->>CAP: start_capture()\n    CAP-->>REC: active_recording\n    REC->>EVT: emit(\"recording-state-changed\", Listening)\n    REC-->>UI: Ok(())\n    \n    Note over UI: Pill shows Listening + waveform\n    \n    UI->>REC: invoke(\"stop_recording\")\n    REC->>CAP: stop_capture() → WAV path\n    REC->>EVT: emit(\"recording-state-changed\", Processing)\n    REC-->>UI: Ok(wav_path)\n    \n    Note over UI: Pill shows spinner\n    \n    rect rgb(200, 230, 200)\n        Note over ORCH,CLIP: Background task (spawn_blocking)\n        ORCH->>WHIS: invoke(binary, model, wav)\n        WHIS-->>ORCH: stdout / JSON segments\n        ORCH->>POST: run_pipeline(text, rules)\n        POST-->>ORCH: cleaned_text, segments\n        ORCH->>DB: insert_session()\n        ORCH->>CLIP: write(text) / insert()\n        ORCH->>EVT: emit(\"transcription-completed\", result)\n        ORCH->>EVT: emit(\"output-result\", output)\n        ORCH->>EVT: emit(\"recording-state-changed\", Success)\n        ORCH->>EVT: schedule_idle_reset()\n    end\n    \n    EVT->>ZUST: setLastTranscription(result)\n    EVT->>ZUST: setRecordingState(\"success\")\n    EVT->>ZUST: setLastOutputResult(output)\n    ZUST->>UI: re-render SuccessContent\n```\n\n## Data Flow by Domain\n\n### Recording Domain\n\n| Data | Rust Location | Sync/Async | Frontend Store |\n|------|--------------|------------|----------------|\n| `recordingState` | `AppState.recording_state` | Event `recording-state-changed` | `app-store.recordingState` |\n| `recordingError` | Event payload | Event `recording-state-changed` | `app-store.recordingError` |\n| `audioLevel` | cpal callback | Event `audio-level` | `app-store.audioLevel` |\n| `isPillExpanded` | Window state | invoke `expandPill/collapsePill` | `app-store.isPillExpanded` |\n\n### Transcription Domain\n\n| Data | Rust Location | Sync/Async | Frontend Store |\n|------|--------------|------------|----------------|\n| `lastTranscription` | `AppState.last_transcription` | Event `transcription-completed` | `app-store.lastTranscription` |\n| `sessions` | `sessions_repo` | invoke `list_sessions` | `history-store.sessions` |\n| `dashboardStats` | `stats/service.rs` | invoke `get_dashboard_stats` | `dashboard-store.stats` |\n\n### Settings Domain\n\n| Data | Rust Location | Sync/Async | Frontend Store |\n|------|--------------|------------|----------------|\n| `settings` | `settings_repo` | invoke `get_settings` / `update_setting` | `settings-store.settings` |\n| `theme` | settings + event | invoke + Event `theme-changed` | `lib/theme.ts` |\n\n## Key Implementation Notes\n\n1. **Event listeners are registered in `PillApp.tsx`** — not in every component. This prevents duplicate listeners and centralizes state updates.\n\n2. **High-frequency events are throttled** — `audio-level` uses `useThrottledEvent` to limit to ~60fps via `requestAnimationFrame`.\n\n3. **Commands run on the Rust main thread** — they block until complete. Long-running operations (transcription) spawn background tasks and return immediately.\n\n4. **AppState is thread-safe** — uses `Mutex<T>` for mutable state accessed from both sync commands and async background tasks.\n\n5. **Events broadcast to all windows** — `app.emit()` sends to both pill and main window simultaneously.\n\n6. **Zustand store updates trigger React re-renders** — only components subscribed to changed state slices re-render.\n",
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\state-flow.md"
}
The diagram has been created. Let me verify the file is correct:

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\docs\\dev\\state-flow.md",
  "limit": 50
}
<summary>
## Summary: Create state flow architecture diagram

### Changes Implemented
- Created comprehensive Mermaid diagrams in `docs/dev/state-flow.md` showing:
  - Full state flow between Rust AppState, Tauri events/commands, Zustand stores, and React components
  - Recording state transition diagram (Idle → Listening → Processing → Success/Error)
  - Sync vs async communication patterns with `invoke()` vs `emit()`
  - Audio level flow sequence (high-frequency ~10ms updates)
  - Transcription flow sequence (background task with whisper.cpp)
  - Data flow tables by domain (recording, transcription, settings)

### Files Modified
- `docs/dev/state-flow.md` (new file)

### Notes for Developer
- The diagrams highlight sync vs async patterns: `invoke()` commands are blocking/frontend-initiated, while Tauri `emit()` events are non-blocking/backend-pushed
- Key events documented: `recording-state-changed`, `audio-level`, `transcription-completed`, `output-result`, `theme-changed`, `navigate-to`, `silence-detected`
- All Mermaid diagrams use HTML-encoded characters (`&lt;`, `&gt;`, `&amp;`) for proper rendering in markdown viewers

### Verification Status
- Document created and verified by reading back the file content. No Playwright test needed as this is a static documentation artifact containing Mermaid diagrams.
</summary>