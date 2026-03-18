# Tauri Command API Reference

Auto-generated from Rust source. All commands are invoked from the frontend via `invoke()`.

**Error handling:** Every command returns `CmdResult<T>`, which is `Result<T, String>`. Frontend code should check for errors and display user-friendly messages via `errors/mod.rs::user_friendly_message()`.

**Frontend wrapper:** All commands are pre-wrapped in `src/lib/tauri.ts` for typed access.

---

## Recording

### `start_recording`

Starts a new audio recording session.

```typescript
invoke('start_recording'): Promise<void>
```

**Behavior:**
- Reads `recording.device_id`, `recording.silence_threshold`, and `recording.silence_timeout_ms` from settings
- Begins microphone capture via cpal
- Transitions pill to `Listening` state (emits `recording-state-changed`)
- Spawns a background silence-detection thread; if silence timeout fires, auto-stops
- Returns immediately; transcription runs asynchronously in the background

**Error codes:**
| Error | Cause |
|-------|-------|
| `"Recording already in progress"` | Pill is already in Listening or Processing state |
| Device errors | No microphone or access denied |
| `"Invalid audio device"` | Device ID in settings does not match any available device |

---

### `stop_recording`

Stops the active recording, saves a WAV file, and triggers background transcription.

```typescript
invoke('stop_recording'): Promise<string>
// Returns: absolute path to the saved WAV file
```

**Behavior:**
- Stops audio capture and writes WAV to temp dir
- Stores WAV path in `last_wav_path` state
- Transitions pill to `Processing` state
- Kicks off transcription asynchronously (emits `transcription-complete` or `transcription-error` when done)

**Error codes:**
| Error | Cause |
|-------|-------|
| `"No active recording"` | Called without an active recording |

---

### `cancel_recording`

Discards the current recording buffer and returns to Idle without saving.

```typescript
invoke('cancel_recording'): Promise<void>
```

**Behavior:**
- Discards all buffered samples
- Transitions pill back to `Idle`
- No file is written

---

### `get_recording_state`

Returns the current recording state.

```typescript
invoke('get_recording_state'): Promise<RecordingState>
// RecordingState: "idle" | "listening" | "processing" | "success" | "error"
```

---

### `list_input_devices`

Returns all available audio input devices.

```typescript
invoke('list_input_devices'): Promise<DeviceInfo[]>

interface DeviceInfo {
  id: string;        // cpal device identifier
  name: string;      // human-readable name
  isDefault: boolean;
}
```

---

## Transcription

### `transcribe_last_recording`

Re-transcribes the most recently recorded WAV file.

```typescript
invoke('transcribe_last_recording', {
  language?: string,    // ISO 639-1 code (e.g. "de", "en"); defaults to `transcription.default_language`
  modelId?: string      // currently unused (placeholder for MS-07)
}): Promise<TranscriptionResult>

interface TranscriptionResult {
  rawText: string;         // raw whisper output (before post-processing)
  cleanedText: string;      // after normalization, filler removal, corrections
  segments: TranscriptSegment[];
  language: string;         // actual language used (e.g. "de", "en", "auto")
  modelId: string;          // stem of the model file (e.g. "ggml-base")
  durationMs: number;       // wall-clock transcription time in ms
  output?: OutputResult;    // set by orchestrator after output step
  removedFillers: string[];  // fillers removed during post-processing
}

interface TranscriptSegment {
  startMs: number;
  endMs: number;
  text: string;
  confidence?: number;   // mean token probability [0, 1]
}

interface OutputResult {
  mode: "clipboard" | "insert";
  success: boolean;
  error?: string;
}
```

**Error codes:**
| Error | Cause |
|-------|-------|
| `"No recording available to transcribe"` | `stop_recording` was never called or WAV file is missing |
| whisper errors | Model not found, invalid audio format, etc. |

---

### `get_last_transcription`

Returns the most recently completed transcription result.

```typescript
invoke('get_last_transcription'): Promise<TranscriptionResult | null>
```

---

## History

### `list_sessions`

Returns a filtered, paginated list of sessions (newest first).

```typescript
invoke('list_sessions', { filter: SessionFilter }): Promise<Session[]>

interface SessionFilter {
  query?: string;          // full-text search across rawText and cleanedText
  language?: string;       // ISO 639-1 code (e.g. "de", "en")
  dateFrom?: string;       // ISO 8601 lower bound for startedAt
  dateTo?: string;         // ISO 8601 upper bound for startedAt
  modelId?: string;        // model stem filter
  limit?: number;
  offset?: number;
}

interface Session {
  id: string;
  startedAt: string;       // ISO 8601 timestamp
  endedAt: string;
  durationMs: number;
  language: string;
  modelId?: string;
  triggerType: string;     // "hotkey" | "button" | ...
  inputDeviceId?: string;
  rawText: string;
  cleanedText: string;
  wordCount: number;
  charCount: number;
  avgConfidence?: number;
  estimatedWpm?: number;
  outputMode: string;
  outputTargetApp?: string;
  insertedSuccessfully: boolean;
  errorMessage?: string;
  createdAt: string;
  audioPath?: string;
  originalRawText?: string;
  reprocessedCount: number;
}
```

---

### `get_session`

Returns a single session together with its time-stamped segments.

```typescript
invoke('get_session', { sessionId: string }): Promise<SessionWithSegments>

interface SessionWithSegments {
  session: Session;
  segments: SessionSegment[];
}

interface SessionSegment {
  id: string;
  sessionId: string;
  startMs: number;
  endMs: number;
  text: string;
  confidence?: number;
  segmentIndex: number;
}
```

**Error codes:**
| Error | Cause |
|-------|-------|
| `"Session not found"` | No session with the given ID |

---

### `delete_session`

Permanently deletes a session and all its segments.

```typescript
invoke('delete_session', { sessionId: string }): Promise<void>
```

**Error codes:**
| Error | Cause |
|-------|-------|
| `"Session not found"` | No session with the given ID |

---

### `export_sessions`

Exports selected sessions to a user-chosen file via native save dialog.

```typescript
invoke('export_sessions', {
  sessionIds: string[],   // empty array returns an error
  format: string          // "json" for JSON array, anything else for plain text
}): Promise<string>
// Returns: chosen file path
```

**Error codes:**
| Error | Cause |
|-------|-------|
| `"No sessions selected for export"` | Empty `sessionIds` array |
| `"Export cancelled"` | User closed the dialog without choosing |
| Write errors | File write failed |

---

### `reprocess_session`

Re-transcribes a session using its stored audio file. Emits `session-reprocessed` on success.

```typescript
invoke('reprocess_session', {
  sessionId: string,
  language?: string,    // override the transcription language
  modelId?: string      // override the model
}): Promise<SessionWithSegments>
```

**Error codes:**
| Error | Cause |
|-------|-------|
| `"Session not found"` | No session with the given ID |
| `"No audio path"` | Session has no stored audio file |
| whisper errors | Transcription failed |

---

## Dictionary

### `list_dictionary_entries`

```typescript
invoke('list_dictionary_entries'): Promise<DictionaryEntry[]>

interface DictionaryEntry {
  id: string;
  phrase: string;
  normalizedPhrase: string;
  language?: string;
  entryType: string;    // "term" | "name" | "acronym" | "product" | "custom"
  notes?: string;
  createdAt: string;
  updatedAt: string;
}
```

---

### `create_dictionary_entry`

```typescript
invoke('create_dictionary_entry', {
  payload: CreateEntryPayload
}): Promise<DictionaryEntry>

interface CreateEntryPayload {
  phrase: string;
  language?: string;
  entryType: string;
  notes?: string;
}
```

---

### `update_dictionary_entry`

```typescript
invoke('update_dictionary_entry', {
  id: string,
  payload: CreateEntryPayload
}): Promise<void>
```

---

### `delete_dictionary_entry`

```typescript
invoke('delete_dictionary_entry', { id: string }): Promise<void>
```

---

## Correction Rules

### `list_correction_rules`

```typescript
invoke('list_correction_rules'): Promise<CorrectionRule[]>

interface CorrectionRule {
  id: string;
  sourcePhrase: string;
  normalizedSourcePhrase: string;
  targetPhrase: string;
  language?: string;
  ruleMode: string;      // "manual" | "suggested" | "learned"
  isActive: boolean;
  autoApply: boolean;
  usageCount: number;
  lastUsedAt?: string;
  createdAt: string;
  updatedAt: string;
}
```

---

### `create_correction_rule`

```typescript
invoke('create_correction_rule', {
  payload: CreateRulePayload
}): Promise<CorrectionRule>

interface CreateRulePayload {
  sourcePhrase: string;
  targetPhrase: string;
  language?: string;
  autoApply: boolean;
}
```

---

### `update_correction_rule`

```typescript
invoke('update_correction_rule', {
  id: string,
  payload: UpdateRulePayload
}): Promise<void>

interface UpdateRulePayload {
  sourcePhrase: string;
  targetPhrase: string;
  language?: string;
  isActive: boolean;
  autoApply: boolean;
}
```

---

### `delete_correction_rule`

```typescript
invoke('delete_correction_rule', { id: string }): Promise<void>
```

---

## Ambiguity

### `list_ambiguous_terms`

Returns low-confidence terms with occurrence counts.

```typescript
invoke('list_ambiguous_terms'): Promise<AmbiguousTerm[]>

interface AmbiguousTerm {
  id: string;
  phrase: string;
  normalizedPhrase: string;
  language?: string;
  occurrences: number;
  avgConfidence?: number;
  lastSeenAt: string;
  suggestedTarget?: string;
  dismissed: boolean;
  createdAt: string;
  updatedAt: string;
}
```

> Minimum occurrences threshold: `ambiguity.min_occurrences` setting (default: 3).

---

### `accept_ambiguity_suggestion`

Creates a correction rule from an ambiguity suggestion.

```typescript
invoke('accept_ambiguity_suggestion', {
  id: string,
  targetPhrase: string
}): Promise<void>
```

---

### `dismiss_ambiguity_suggestion`

Dismisses an ambiguity suggestion (sets `dismissed = true`).

```typescript
invoke('dismiss_ambiguity_suggestion', { id: string }): Promise<void>
```

---

## Filler Words

### `list_filler_words`

```typescript
invoke('list_filler_words', {
  language?: string    // ISO 639-1 code; omit for all languages
}): Promise<FillerWord[]>

interface FillerWord {
  id: string;
  word: string;
  language: string;
  isDefault: boolean;
  createdAt: string;
}
```

---

### `add_filler_word`

```typescript
invoke('add_filler_word', {
  word: string,
  language: string     // ISO 639-1 code
}): Promise<FillerWord>
```

---

### `delete_filler_word`

```typescript
invoke('delete_filler_word', { id: string }): Promise<void>
```

---

### `reset_filler_words`

Resets filler words to defaults for a language.

```typescript
invoke('reset_filler_words', { language: string }): Promise<FillerWord[]>
// Returns: the updated default list for that language
```

---

### `get_filler_stats`

Returns usage statistics for filler words.

```typescript
invoke('get_filler_stats', {
  language?: string
}): Promise<FillerStat[]>

interface FillerStat {
  word: string;
  language: string;
  count: number;
  lastRemovedAt: string;
}
```

---

### `get_filler_total_count`

```typescript
invoke('get_filler_total_count'): Promise<number>
// Returns: total count of filler words removed across all sessions
```

---

## Models

### `list_available_models`

Returns all known models merged with their current install state.

```typescript
invoke('list_available_models'): Promise<ModelInfo[]>

interface ModelInfo {
  key: string;                    // registry key (e.g. "tiny", "base", "small")
  displayName: string;
  languageScope: string;
  fileSizeBytes: number;
  installed: boolean;
  isDefaultForDe: boolean;
  isDefaultForEn: boolean;
  defaultForLanguages: string[];
  localPath?: string;
  installedAt?: string;
  description: string;
  speed: string;
  accuracy: string;
  category: string;
  recommendedFor: string;
}
```

---

### `download_model`

Downloads, verifies, and installs a model. Emits `model-download-progress` events during transfer.

```typescript
invoke('download_model', { key: string }): Promise<void>
// key: registry key (e.g. "tiny", "base", "small")
```

**Error codes:**
| Error | Cause |
|-------|-------|
| `"Model not found in registry"` | Invalid key |
| Download errors | Network failure, disk full, checksum mismatch |

---

### `delete_model`

Deletes the model file from disk and clears its install record.

```typescript
invoke('delete_model', { key: string }): Promise<void>
```

---

### `set_default_model`

Sets the default model for a given language.

```typescript
invoke('set_default_model', {
  language: string,   // ISO 639-1 code ("de" or "en")
  key: string          // registry key of the model
}): Promise<void>
```

---

## Settings

### `get_settings`

Returns all settings as a flat key-value map.

```typescript
invoke('get_settings'): Promise<Record<string, string>>
// Keys use dot-notation, e.g. "recording.device_id", "transcription.default_language"
```

---

### `update_setting`

Upserts a single setting.

```typescript
invoke('update_setting', {
  key: string,    // dot-notation key
  value: string
}): Promise<void>
```

---

### `reset_settings`

Resets all settings to factory defaults.

```typescript
invoke('reset_settings'): Promise<void>
```

---

### `update_shortcut`

Updates the global recording shortcut.

```typescript
invoke('update_shortcut', { shortcut: string }): Promise<void>
// Format: Electron-style (e.g. "Ctrl+Shift+Space", "CommandOrControl+Shift+Space")
```

**Behavior:**
- Validates the shortcut format before persisting
- Unregisters all current shortcuts
- Persists to `recording.shortcut` setting
- Registers the new shortcut globally

**Error codes:**
| Error | Cause |
|-------|-------|
| `"Invalid shortcut '...'"` | Malformed shortcut string |
| Unregister errors | Failed to clear old shortcut |
| Register errors | Failed to register new shortcut (e.g. already in use by another app) |

---

## Window

### `show_pill`

Shows the pill window if hidden.

```typescript
invoke('show_pill'): Promise<void>
```

---

### `hide_pill`

Hides the pill window.

```typescript
invoke('hide_pill'): Promise<void>
```

---

### `open_main_window`

Opens or focuses the main window.

```typescript
invoke('open_main_window'): Promise<void>
// Creates the window with default size (1100x720) if it does not exist
```

---

### `expand_pill`

Expands the pill to show the expanded view (220x280px).

```typescript
invoke('expand_pill'): Promise<void>
```

---

### `collapse_pill`

Collapses the pill back to compact mode (220x70px).

```typescript
invoke('collapse_pill'): Promise<void>
```

---

### `set_pill_position`

Moves the pill to the given screen coordinates.

```typescript
invoke('set_pill_position', {
  x: number,    // screen X coordinate in pixels
  y: number     // screen Y coordinate in pixels
}): Promise<void>
```

---

## System

### `check_first_run`

Returns `true` when no model is installed (i.e. user needs onboarding).

```typescript
invoke('check_first_run'): Promise<boolean>
```

---

### `has_default_model`

Returns `true` when a default model is configured for the current language.

```typescript
invoke('has_default_model'): Promise<boolean>
```

---

### `set_autostart`

Enables or disables launching LocalVoice on OS login.

```typescript
invoke('set_autostart', { enabled: boolean }): Promise<void>
```

---

### `get_autostart`

Returns the current autostart state.

```typescript
invoke('get_autostart'): Promise<boolean>
```

---

## Stats

### `get_dashboard_stats`

Returns scalar dashboard metrics for the given date range.

```typescript
invoke('get_dashboard_stats', { range: DateRange }): Promise<DashboardStats>

interface DateRange {
  start?: string;   // ISO 8601 date (inclusive lower bound)
  end?: string;     // ISO 8601 date (inclusive upper bound)
}

interface DashboardStats {
  totalWordCount: number;
  totalSessionCount: number;
  avgWpm: number;
  totalDurationMs: number;
  languageCounts: LanguageCount[];
  topModels: ModelUsageStat[];
}

interface LanguageCount {
  language: string;
  count: number;
}

interface ModelUsageStat {
  modelId: string;
  sessionCount: number;
  totalWordCount: number;
  totalDurationMs: number;
  avgWpm: number;
}
```

---

### `get_usage_timeseries`

Returns daily or weekly word/session counts over time.

```typescript
invoke('get_usage_timeseries', {
  range: DateRange,
  bucket: string    // "day" or "week"
}): Promise<TimeseriesPoint[]>

interface TimeseriesPoint {
  date: string;         // ISO 8601 date string (e.g. "2026-03-17")
  wordCount: number;
  sessionCount: number;
}
```

---

### `get_language_breakdown`

Returns per-language word count, session count, and duration.

```typescript
invoke('get_language_breakdown', { range: DateRange }): Promise<LanguageBreakdown[]>

interface LanguageBreakdown {
  language: string;
  sessionCount: number;
  wordCount: number;
  durationMs: number;
}
```

---

### `get_correction_stats`

Returns top 10 most-used correction rules with usage counts.

```typescript
invoke('get_correction_stats'): Promise<CorrectionStat[]>

interface CorrectionStat {
  sourcePhrase: string;
  targetPhrase: string;
  usageCount: number;
  lastUsedAt?: string;
}
```

---

### `get_wpm_trend`

Returns average WPM per time bucket.

```typescript
invoke('get_wpm_trend', {
  range: DateRange,
  bucket: string    // "day" or "week"
}): Promise<WpmPoint[]>

interface WpmPoint {
  date: string;
  avgWpm: number;
  sessionCount: number;
}
```

---

### `get_daily_comparison`

Returns side-by-side stats for two dates.

```typescript
invoke('get_daily_comparison', {
  dateA: string,    // ISO 8601 date
  dateB: string     // ISO 8601 date
}): Promise<[DailyStats, DailyStats]>

interface DailyStats {
  date: string;
  sessionCount: number;
  wordCount: number;
  durationMs: number;
  avgWpm: number;
}
```

---

## Logs

### `list_logs`

Returns the most recent log entries, optionally filtered.

```typescript
invoke('list_logs', {
  levelFilter?: string,   // "warn" | "error" | omit for all
  limit?: number          // default 500
}): Promise<LogEntry[]>

interface LogEntry {
  id: string;
  level: string;      // "warn" | "error" | "info" | "debug"
  area: string;       // module or component name
  message: string;
  createdAt: string;  // ISO 8601 timestamp
}
```

---

### `export_logs`

Opens a native save dialog and writes all log entries as JSON.

```typescript
invoke('export_logs'): Promise<void>
// Filename: localvoice-logs.json
```

**Error codes:**
| Error | Cause |
|-------|-------|
| `"Log buffer not initialized"` | App not started properly |
| `"Export cancelled"` | User closed the dialog |

---

### `clear_logs`

Clears all buffered log entries.

```typescript
invoke('clear_logs'): Promise<void>
```

---

### `set_logging_enabled`

Enables or disables in-app log buffering at runtime.

```typescript
invoke('set_logging_enabled', { enabled: boolean }): Promise<void>
```

---

## Frontend Event References

The frontend subscribes to these Tauri events (via `listen()`) to update UI state:

| Event | Payload | Triggered by |
|-------|---------|--------------|
| `recording-state-changed` | `{ state: RecordingState, error?: string }` | Any recording command, silence auto-stop |
| `transcription-complete` | `TranscriptionResult` | Transcription orchestrator on success |
| `transcription-error` | `{ error: string }` | Transcription orchestrator on failure |
| `session-reprocessed` | `sessionId: string` | `reprocess_session` command |
| `model-download-progress` | `{ key: string, progress: number }` | Model download in progress |
| `silence-detected` | none | Silence timeout reached during recording |

---

## Event Emitters (Frontend → Backend)

Frontend events the backend listens to (via `listen_unpinned()`):

| Event | Payload | Handled by |
|-------|---------|------------|
| `insert-text` | `{ text: string }` | `os/text_insertion.rs` — auto-insert pipeline |
| `clipboard-copied` | `{ text: string }` | Logging in the transcription orchestrator |
