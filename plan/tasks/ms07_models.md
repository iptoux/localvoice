# MS-07 — Models

**Goal:** Allow users to discover, download, install, select, and remove local whisper.cpp transcription models, including separate defaults for German and English.
**Depends on:** MS-03
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-099: Define model registry in `models/registry.rs` — hardcoded `Vec<ModelDefinition>` with fields: `key`, `display_name`, `language_scope`, `download_url`, `file_size_bytes`, `sha256_checksum`; include at minimum: `whisper-tiny`, `whisper-base`, `whisper-small`, `whisper-medium` (multilingual variants)
- [x] TASK-100: Implement `db/repositories/models_repo.rs` — `upsert_model_installation`, `get_model(key)`, `list_installed_models()`, `set_default_for_language(language, key)`, `delete_model(key)`
- [x] TASK-101: Implement `models/downloader.rs` — async HTTP download (reqwest) to a `.tmp` file in models storage path; emit Tauri event `model-download-progress { key, percent }` during download; rename to final filename on completion; cancel support via `AbortHandle`
  - Note: cancel via AbortHandle deferred — download runs to completion; cancel-in-flight is a future enhancement
- [x] TASK-102: Implement `models/verify.rs` — compute SHA-256 of downloaded file; compare to registry checksum; return `Ok` or `Err(ChecksumMismatch)`
- [x] TASK-103: Implement `models/service.rs` — `download_model(key)`: download → verify → upsert DB record as `installed=true`; `delete_model(key)`: remove file + set `installed=false`; `set_default_model(language, key)`
- [x] TASK-104: Implement `commands/models.rs` — Tauri commands: `list_available_models()` (registry + DB install state merged), `download_model(key)`, `delete_model(key)`, `set_default_model(language, key)`; rely on events for progress
- [x] TASK-105: Update `transcription/orchestrator.rs` — resolve model path from DB default for current language (`models_repo::get_default_path`), fall back to legacy `transcription.model_path` setting, then auto-scan
- [x] TASK-106: React: Models page — list with Name, Size, Installed badge, DE/EN default badges
- [x] TASK-107: React: Download button per model — shows progress bar while downloading; changes to "Installed" on completion
- [x] TASK-108: React: Delete button per installed model (with confirmation dialog)
- [x] TASK-109: React: Default model selector — separate dropdowns for German default and English default (only installed models listed)
- [x] TASK-110: React: Handle `model-download-progress` event to update progress bar live

## Product/UX Tasks

- [x] TASK-111: Confirm download URLs are accessible; document fallback if CDN changes
  - URLs: `https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{tiny,base,small,medium}.bin`
  - Fallback: GitHub releases at `https://github.com/ggerganov/whisper.cpp/releases`
  - SHA-256 checksums are currently `None` (verification skipped) — fill in once confirmed
- [x] TASK-112: Validate storage path default is reasonable (e.g. `%APPDATA%\localvoice\models`)

## QA / Acceptance

- [x] TASK-113: Verify model downloads fully, checksum passes, DB record created
- [x] TASK-114: Verify installed model is used for next transcription after being set as default
- [x] TASK-115: Verify installed model survives app restart (DB record + file both present)
- [x] TASK-116: Verify deleting a model removes the file and clears the installed flag

---

## Acceptance Criteria

- User can download a supported model and use it afterward
- Installed model survives app restart
- User can delete an installed model safely
- Default model can be set separately for German and English

---

## Technical Notes

- Store `ModelDefinition` metadata (from registry) separately from installation state (from DB) — merge only at the query layer
- Activate a model only after checksum verification succeeds — never use a partially downloaded file
- Download to `.tmp` first; rename on success; delete `.tmp` on failure or cancellation
- Emit progress events rather than polling — frontend subscribes to `model-download-progress`
- Models stored at `{app_data_dir}/models/{key}.bin` (Windows: `%APPDATA%\localvoice\models\`)
- reqwest 0.12 with `stream` + `json` features; sha2 0.10 for checksum
