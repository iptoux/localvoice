# PRD - Desktop Voice Dictation App

**Working title:** `FlowDict`  
**Type:** Offline-first desktop dictation app  
**Stack:** Tauri v2 + TypeScript + Rust + whisper.cpp  
**Primary platform:** Windows first, later macOS/Linux  
**Initial languages:** German, English

---

## 1. Product Vision

A fast, local desktop app for system-wide voice input. Users start and stop recording with a global shortcut, speak, and the recognized text is transcribed locally, post-processed, and then inserted into any app or copied to the clipboard.

The app should feel lightweight. By default, it should not open as a large main window, but as a **small floating pill** that shows the current **voice state**.

---

## 2. Product Goal

The app should solve three core problems:

1. **Fast local voice input** without a cloud dependency
2. **Better recognition over time** through a personal dictionary and correction rules
3. **Full traceability** through history, metrics, and reprocessing

---

## 3. Core Principles

- Offline-first
- Privacy-first
- Fast interaction
- Minimal UI surface
- System-wide usable
- User-improving intelligence through dictionary and correction learning

---

## 4. Target Audience

- Developers
- Power users
- Users with a lot of writing work
- Users who want local/offline dictation
- Users with repeated technical terms, names, or domain-specific vocabulary

---

## 5. Platform UX Concept

### Default Window Behavior

On app start, the main window is **a small pill by default**.

### Pill Goals

- always quickly accessible
- shows only the current state
- minimally distracting
- ideal as a desktop overlay

### Pill States

- Idle
- Listening
- Processing
- Success
- Error
- optional: Muted / No microphone
- optional: Model downloading

### Pill Content

- small microphone or waveform icon
- state text
- optional timer during recording
- optional animated ring or wave
- click opens expanded view or full app

### Expanded View

A compact view can open from the pill:

- latest transcript
- Start/Stop
- language
- model
- quick actions

### Full Window

Full view for:

- Dashboard
- History
- Dictionary
- Models
- Settings

---

## 6. Product Requirements Document

### 6.1 Problem Statement

Users want to convert speech to text quickly, locally, and without browser dependence or cloud services. Existing solutions are often:

- not local
- too heavyweight
- not pleasant for system-wide usage
- not adaptive to personal vocabulary

### 6.2 Goals

#### Product Goals

- ship a solid local dictation app with strong UX
- build a clean base for future expansion
- differentiate through personal dictionary and ambiguity handling
- deliver a fast MVP with maintainable architecture

#### User Goals

- start and stop recording with a global shortcut
- transcribe locally
- use the text immediately
- search history
- improve unclear words
- manage local models
- use German and English simply

### 6.3 Non-Goals for MVP

- cloud sync
- team/shared dictionary
- full real-time streaming into target apps with perfect per-character insertion
- mobile companion
- complex LLM rewriting
- full auto-language detection
- snippets/macros library
- speaker diarization

### 6.4 User Stories

#### Core

- As a user, I want to start and stop recording with a global shortcut.
- As a user, I want the text to be transcribed locally.
- As a user, I want the text inserted into the active app or copied to the clipboard.
- As a user, I want to choose German or English.

#### UI

- As a user, I want the app to default to a small pill so it takes very little space.
- As a user, I want the pill to instantly show whether the app is listening, processing, or done.
- As a user, I want to open the full interface when needed.

#### History

- As a user, I want to search previous sessions.
- As a user, I want to compare raw transcript and cleaned transcript.
- As a user, I want to delete or export old sessions.

#### Dictionary

- As a user, I want to correct misrecognized words and save rules.
- As a user, I want suggestions for unclear words.
- As a user, I want to save my own words permanently.

#### Models

- As a user, I want to download and remove local models.
- As a user, I want to choose a default model per language.

#### Settings

- As a user, I want to configure theme, shortcut, autostart, language, and window behavior.
- As a user, I want to choose whether text is inserted directly or only copied.

### 6.5 Functional Requirements

#### A. Recording and Transcription

1. User can start and stop recording with a global shortcut.
2. Audio is captured from the selected microphone.
3. Audio is processed locally.
4. whisper.cpp performs transcription locally.
5. User can select language:
   - German
   - English
6. User can select a model.
7. Result is stored.
8. Result is inserted or copied.

#### B. Pill UI

1. The app starts in pill form by default.
2. The pill is floating and compact.
3. The pill shows the voice state.
4. The pill can expand into a compact view or full window.
5. Pill position and size are persisted.
6. Always-on-top is optional.

#### C. Dashboard

1. total word count
2. total sessions
3. WPM
4. total recording time
5. most used language
6. most frequent corrections
7. unclear words
8. time-series charts

#### D. History

1. session list
2. search
3. filter by language, model, date
4. raw vs cleaned transcript
5. copy/export/delete
6. reprocess session

#### E. Dictionary

1. manual entries
2. correction rules
3. ambiguity suggestions
4. enable/disable per rule
5. usage tracking
6. optional automatic replacement

#### F. Settings

1. theme
2. autostart
3. start hidden
4. pill default mode
5. shortcut
6. microphone
7. language
8. model
9. insert vs clipboard
10. local model path
11. window positions and sizes
12. local processing options

#### G. Model Manager

1. list available models
2. download
3. progress
4. installed / not installed state
5. file size
6. delete
7. default per language

### 6.6 Non-Functional Requirements

#### Performance

- Start recording within about 150-300 ms after shortcut
- UI state changes visible immediately
- local transcription of short sessions should feel fast enough
- UI should stay lightweight

#### Reliability

- recording abort must not destabilize the app
- history persistence must be robust
- failed or corrupt downloads must be detected
- model state must stay consistent

#### Privacy

- no cloud by default
- audio stays local
- local database
- local model files

#### Usability

- pill as the primary entry point
- clear state transitions
- few clicks
- strong defaults

#### Maintainability

- modular Rust architecture
- clear separation of UI, core, database, and OS integration
- sidecar-based whisper.cpp integration for MVP

---

## 7. UX Specification

### 7.1 Information Architecture

Primary views:

- Pill
- Expanded View
- Dashboard
- History
- Dictionary
- Models
- Settings

### 7.2 Pill Specification

#### Default

At startup:

- app runs in tray
- pill is visible
- full window stays closed

#### Size

Small, horizontal, rounded, capsule-like UI.

#### Pill Content by State

**Idle**
- Mic icon
- "Ready"
- optional shortcut hint

**Listening**
- active pulse
- elapsed time
- "Listening..."

**Processing**
- spinner/wave
- "Transcribing..."

**Success**
- check icon
- "Inserted" or "Copied"

**Error**
- warning icon
- short error text

#### Interactions

- click: open expanded view
- double click: open full app
- right click: context menu
- drag: move position

#### Context Menu

- Start/Stop
- Open App
- History
- Settings
- Quit

### 7.3 Full App Navigation

Sidebar:

- Dashboard
- History
- Dictionary
- Models
- Settings

---

## 8. Technical Module Structure

### 8.1 High-Level Architecture

```text
Frontend (TypeScript/React)
  -> Tauri command bridge
Rust Core
  -> audio capture
  -> transcription orchestration
  -> post-processing
  -> DB access
  -> model manager
  -> OS integration
whisper.cpp sidecar
SQLite
Local filesystem (models, config, temp audio)
```

### 8.2 Frontend Module Structure

```text
src/
  app/
    routes/
    layout/
  components/
    pill/
    overlay/
    dashboard/
    history/
    dictionary/
    models/
    settings/
    common/
  features/
    recording/
    transcription/
    history/
    dictionary/
    stats/
    models/
    settings/
    window-state/
  stores/
    app-store.ts
    recording-store.ts
    settings-store.ts
    history-store.ts
    models-store.ts
    dictionary-store.ts
  hooks/
  lib/
    tauri.ts
    format.ts
    validation.ts
  types/
```

Frontend responsibilities:

- rendering
- UI state
- command calls to Rust
- view-model transformations
- charts and tables
- pill / expanded / full window behavior

### 8.3 Rust Module Structure

```text
src-tauri/src/
  main.rs
  commands/
    recording.rs
    transcription.rs
    history.rs
    dictionary.rs
    stats.rs
    settings.rs
    models.rs
    window.rs
    system.rs
  audio/
    capture.rs
    devices.rs
    wav_writer.rs
    level_meter.rs
  transcription/
    orchestrator.rs
    whisper_sidecar.rs
    parser.rs
    language.rs
    pipeline.rs
  postprocess/
    normalize.rs
    punctuation.rs
    capitalization.rs
    fillers.rs
    replacements.rs
    ambiguity.rs
  dictionary/
    service.rs
    suggestions.rs
    rules.rs
  history/
    service.rs
    export.rs
    reprocess.rs
  stats/
    service.rs
    aggregations.rs
  settings/
    service.rs
    defaults.rs
  models/
    service.rs
    downloader.rs
    registry.rs
    verify.rs
  os/
    hotkeys.rs
    clipboard.rs
    text_insertion.rs
    tray.rs
    autostart.rs
    windows.rs
  db/
    mod.rs
    migrations.rs
    schema.rs
    repositories/
      sessions_repo.rs
      dictionary_repo.rs
      settings_repo.rs
      models_repo.rs
  state/
    app_state.rs
  errors/
    mod.rs
```

### 8.4 Module Responsibilities

#### `audio`
- detect microphones
- start/stop capture
- generate PCM/WAV
- optionally provide audio level for UI

#### `transcription`
- prepare temp audio
- start whisper.cpp sidecar
- pass model and language params
- parse output
- forward result into post-processing

#### `postprocess`
- whitespace cleanup
- punctuation
- capitalization
- filler removal
- dictionary replacements
- ambiguity detection

#### `dictionary`
- manage manual entries
- apply correction rules
- derive suggestions from history

#### `history`
- store and read sessions
- fetch details
- reprocess old entries
- exports

#### `stats`
- aggregate dashboard metrics

#### `models`
- know available models
- manage downloads
- verify files
- assign defaults per language

#### `os`
- global shortcuts
- tray
- clipboard
- text insertion
- autostart
- pill/full window handling

#### `db`
- SQLite connection
- migrations
- repositories

---

## 9. Database Schema

Recommendation: **SQLite**

### 9.1 Tables Overview

- `sessions`
- `session_segments`
- `session_tokens` optional later
- `dictionary_entries`
- `correction_rules`
- `ambiguous_terms`
- `model_installations`
- `settings`
- `model_downloads` optional
- `app_events` optional

### 9.2 Schema Proposal

#### `sessions`

```sql
CREATE TABLE sessions (
  id TEXT PRIMARY KEY,
  started_at TEXT NOT NULL,
  ended_at TEXT NOT NULL,
  duration_ms INTEGER NOT NULL,
  language TEXT NOT NULL,
  model_id TEXT,
  trigger_type TEXT NOT NULL,
  input_device_id TEXT,
  raw_text TEXT NOT NULL,
  cleaned_text TEXT NOT NULL,
  word_count INTEGER NOT NULL DEFAULT 0,
  char_count INTEGER NOT NULL DEFAULT 0,
  avg_confidence REAL,
  estimated_wpm REAL,
  output_mode TEXT NOT NULL,
  output_target_app TEXT,
  inserted_successfully INTEGER NOT NULL DEFAULT 0,
  error_message TEXT,
  created_at TEXT NOT NULL
);
```

Notes:
- `trigger_type`: shortcut, button, tray
- `output_mode`: insert, clipboard, preview
- `avg_confidence`: if available
- `output_target_app`: optional, if technically detectable

#### `session_segments`

```sql
CREATE TABLE session_segments (
  id TEXT PRIMARY KEY,
  session_id TEXT NOT NULL,
  start_ms INTEGER NOT NULL,
  end_ms INTEGER NOT NULL,
  text TEXT NOT NULL,
  confidence REAL,
  segment_index INTEGER NOT NULL,
  FOREIGN KEY(session_id) REFERENCES sessions(id) ON DELETE CASCADE
);
```

#### `dictionary_entries`

```sql
CREATE TABLE dictionary_entries (
  id TEXT PRIMARY KEY,
  phrase TEXT NOT NULL,
  normalized_phrase TEXT NOT NULL,
  language TEXT,
  entry_type TEXT NOT NULL,
  notes TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

`entry_type` values:
- term
- name
- acronym
- product
- custom

#### `correction_rules`

```sql
CREATE TABLE correction_rules (
  id TEXT PRIMARY KEY,
  source_phrase TEXT NOT NULL,
  normalized_source_phrase TEXT NOT NULL,
  target_phrase TEXT NOT NULL,
  language TEXT,
  rule_mode TEXT NOT NULL,
  confidence_threshold REAL,
  is_active INTEGER NOT NULL DEFAULT 1,
  auto_apply INTEGER NOT NULL DEFAULT 1,
  usage_count INTEGER NOT NULL DEFAULT 0,
  last_used_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

`rule_mode` values:
- manual
- suggested
- learned

#### `ambiguous_terms`

```sql
CREATE TABLE ambiguous_terms (
  id TEXT PRIMARY KEY,
  phrase TEXT NOT NULL,
  normalized_phrase TEXT NOT NULL,
  language TEXT,
  occurrences INTEGER NOT NULL DEFAULT 1,
  avg_confidence REAL,
  last_seen_at TEXT NOT NULL,
  suggested_target TEXT,
  dismissed INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

#### `model_installations`

```sql
CREATE TABLE model_installations (
  id TEXT PRIMARY KEY,
  model_key TEXT NOT NULL UNIQUE,
  display_name TEXT NOT NULL,
  language_scope TEXT NOT NULL,
  local_path TEXT NOT NULL,
  file_size_bytes INTEGER,
  checksum TEXT,
  installed INTEGER NOT NULL DEFAULT 0,
  installed_at TEXT,
  version TEXT,
  is_default_for_de INTEGER NOT NULL DEFAULT 0,
  is_default_for_en INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

`language_scope` values:
- multilingual
- en
- de
- custom

#### `settings`

```sql
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

#### Optional `model_downloads`

```sql
CREATE TABLE model_downloads (
  id TEXT PRIMARY KEY,
  model_key TEXT NOT NULL,
  status TEXT NOT NULL,
  progress_percent REAL NOT NULL DEFAULT 0,
  error_message TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

---

## 10. Settings Keys

Example keys in `settings`:

```text
app.theme
app.language
app.start_hidden
app.autostart
ui.default_mode
ui.pill.always_on_top
ui.pill.position_x
ui.pill.position_y
ui.pill.width
ui.pill.height
ui.main_window.width
ui.main_window.height
recording.input_device_id
recording.shortcut
recording.push_to_talk
recording.silence_timeout_ms
recording.play_start_sound
recording.play_stop_sound
transcription.default_language
transcription.default_model_de
transcription.default_model_en
transcription.auto_punctuation
transcription.auto_capitalization
transcription.remove_fillers
output.mode
output.auto_paste
dictionary.auto_apply_rules
dictionary.suggestion_mode
models.storage_path
```

---

## 11. Domain Models

### Session

```ts
type Session = {
  id: string
  startedAt: string
  endedAt: string
  durationMs: number
  language: 'de' | 'en'
  modelId?: string
  rawText: string
  cleanedText: string
  wordCount: number
  charCount: number
  avgConfidence?: number
  estimatedWpm?: number
  outputMode: 'insert' | 'clipboard' | 'preview'
  insertedSuccessfully: boolean
  errorMessage?: string
}
```

### CorrectionRule

```ts
type CorrectionRule = {
  id: string
  sourcePhrase: string
  targetPhrase: string
  language?: 'de' | 'en'
  ruleMode: 'manual' | 'suggested' | 'learned'
  isActive: boolean
  autoApply: boolean
  usageCount: number
}
```

### ModelInstallation

```ts
type ModelInstallation = {
  id: string
  modelKey: string
  displayName: string
  languageScope: 'multilingual' | 'de' | 'en'
  localPath: string
  installed: boolean
  version?: string
  isDefaultForDe: boolean
  isDefaultForEn: boolean
}
```

---

## 12. Transcription Pipeline

### Input-to-Output Flow

1. global shortcut or pill button
2. recording state = listening
3. audio capture starts
4. recording stop
5. state = processing
6. save temp WAV
7. call whisper.cpp sidecar with model + language
8. parse segments
9. run post-processing pipeline
10. detect ambiguity
11. save history
12. output:
   - insert
   - clipboard
   - preview
13. state = success/error
14. reset to idle after a short period

### Post-Processing Order

1. normalize whitespace
2. trim
3. punctuation optional
4. capitalization optional
5. filler-word removal optional
6. apply correction rules
7. mark ambiguity
8. produce final cleaned output

---

## 13. API / Tauri Commands

### Recording
- `start_recording()`
- `stop_recording()`
- `cancel_recording()`
- `get_recording_state()`

### Transcription
- `transcribe_last_recording()`
- `reprocess_session(session_id)`

### History
- `list_sessions(filter)`
- `get_session(session_id)`
- `delete_session(session_id)`
- `export_sessions(format)`

### Dictionary
- `list_dictionary_entries()`
- `create_dictionary_entry(payload)`
- `update_dictionary_entry(id, payload)`
- `delete_dictionary_entry(id)`
- `list_correction_rules()`
- `create_correction_rule(payload)`
- `update_correction_rule(id, payload)`
- `delete_correction_rule(id)`
- `list_ambiguous_terms()`
- `accept_ambiguity_suggestion(id, target_phrase)`
- `dismiss_ambiguity_suggestion(id)`

### Stats
- `get_dashboard_stats(range)`
- `get_usage_timeseries(range)`

### Models
- `list_available_models()`
- `list_installed_models()`
- `download_model(model_key)`
- `delete_model(model_key)`
- `set_default_model(language, model_key)`

### Settings
- `get_settings()`
- `update_setting(key, value)`
- `reset_settings()`

### Window/UI
- `show_pill()`
- `hide_pill()`
- `open_main_window()`
- `set_pill_position(x, y)`

---

## 14. MVP Scope

### Must Be Included in MVP

- pill as default main window
- global recording shortcut
- microphone recording
- local transcription via whisper.cpp
- German + English
- basic model manager
- basic history
- basic dashboard
- basic settings
- clipboard and optional insert
- basic manual dictionary rules

### Can Be Simplified in MVP

- unclear words handled heuristically only
- insert flow can initially be clipboard + paste fallback
- simple stats only
- no live transcription
- no token-level confidence required

---

## 15. MVP Milestones

### Milestone 1 - Foundation and Shell

**Goal:** technical project base is working

**Deliverables:**
- Tauri v2 app setup
- React/TypeScript UI
- Rust command bridge
- SQLite setup + migrations
- tray + basic window handling
- pill window as default
- theme/system settings basic

**Done definition:**
- app starts
- pill is shown by default
- main window can be opened
- DB is created

### Milestone 2 - Recording Core

**Goal:** capture audio and expose UI state

**Deliverables:**
- microphone list
- start/stop recording
- global shortcut
- listening/processing/idle states
- timer in pill
- temp audio output

**Done definition:**
- shortcut starts recording
- pill shows Listening
- stop ends recording
- temp audio is created

### Milestone 3 - Local Transcription

**Goal:** end-to-end local STT

**Deliverables:**
- whisper.cpp sidecar integration
- model loading
- DE/EN language selection
- segment parsing
- raw transcript generation
- minimal cleaned transcript generation

**Done definition:**
- DE/EN audio is transcribed locally
- result appears in UI
- errors are shown cleanly

### Milestone 4 - Output Workflow

**Goal:** make output usable

**Deliverables:**
- clipboard copy
- optional auto-paste / insert fallback
- success/error states in pill
- latest output in expanded view

**Done definition:**
- transcribed text reliably reaches a usable user-facing output path

### Milestone 5 - History

**Goal:** make sessions traceable

**Deliverables:**
- store sessions
- history list
- session details
- search and basic filters
- copy/delete

**Done definition:**
- sessions are persistent
- user can open old transcripts

### Milestone 6 - Dashboard

**Goal:** visualize usage

**Deliverables:**
- word count
- session count
- avg WPM
- total recording time
- top languages
- simple chart for words over time

**Done definition:**
- dashboard shows real DB data

### Milestone 7 - Models

**Goal:** local model management

**Deliverables:**
- model list
- download
- installed marker
- delete
- default model for DE/EN

**Done definition:**
- user can manage local models
- DE/EN defaults work

### Milestone 8 - Dictionary v1

**Goal:** first learning layer

**Deliverables:**
- manual dictionary terms
- correction rules
- automatic replacement during transcription
- rule usage count

**Done definition:**
- user can permanently correct known wrong terms

### Milestone 9 - Ambiguity v1

**Goal:** detect unclear terms

**Deliverables:**
- low-confidence / heuristic detection
- list of suspect terms
- accept/reject suggestions
- convert accepted suggestions into correction rules

**Done definition:**
- app identifies repeatedly problematic words

### Milestone 10 - Polish

**Goal:** make MVP release-ready

**Deliverables:**
- improved error handling
- loading states
- migration tests
- settings polish
- autostart
- remembered window positions
- onboarding for first model

**Done definition:**
- stable desktop app for first users

---

## 16. Release Order

### MVP v0.1
- pill
- recording
- local transcription
- DE/EN
- basic model management
- history
- dashboard
- settings

### v0.2
- dictionary rules
- better insert flow
- expanded pill view
- search/filter history

### v0.3
- ambiguity engine
- reprocess session
- exports
- stronger stats

---

## 17. Risks and Mitigations

### 1. System-wide text insertion

**Risk:** OS-wide insertion is platform-dependent  
**Mitigation:** start with clipboard-first, then add optional paste automation

### 2. Confidence data

**Risk:** word-level confidence may not always be ideal  
**Mitigation:** use segment-level confidence plus correction-frequency heuristics

### 3. Model downloads

**Risk:** file corruption or interrupted downloads  
**Mitigation:** checksum, temp file, verify-before-activate

### 4. UX overload

**Risk:** full window becomes too heavy  
**Mitigation:** keep pill-first UX and progressive disclosure

---

## 18. Technical Recommendation for Start

### Frontend
- React
- TypeScript
- Zustand
- TanStack Table
- Recharts
- Tailwind
- shadcn/ui

### Backend
- Rust
- Tauri v2
- SQLite with `sqlx` or `rusqlite`
- `cpal` for audio
- whisper.cpp as sidecar
- serde / serde_json

### Recommendation

For MVP, start with **whisper.cpp as a sidecar** rather than direct FFI bindings. This reduces build complexity and gets you to a working product faster.

---

## 19. Proposed Repository Layout

```text
flowdict/
  src/
    components/
    features/
    stores/
    pages/
    lib/
    types/
  src-tauri/
    src/
      commands/
      audio/
      transcription/
      postprocess/
      dictionary/
      history/
      stats/
      models/
      os/
      db/
      settings/
      state/
  assets/
  docs/
    prd.md
    architecture.md
    schema.sql
```

---

## 20. MVP Acceptance Criteria

MVP is reached when:

- the app starts by default as a **small pill**
- the pill visibly represents the **voice state**
- a user can record via shortcut
- DE/EN can be transcribed locally
- models can be downloaded and selected
- transcripts are saved
- stats appear in the dashboard
- dictionary rules can be applied
- the app is usable in normal desktop workflows

---

## 21. Recommended Next Step

The next best implementation artifacts would be:

1. an epic/ticket backlog
2. an SQL migration file
3. folder structure and interface definitions for Rust and TypeScript

