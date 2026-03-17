# MS-07 — Models

**Goal:** Allow users to discover, download, install, select, and remove local whisper.cpp transcription models, including separate defaults for German and English.
**Depends on:** MS-03
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-099: Define model registry in `models/registry.rs` — hardcoded `Vec<ModelDefinition>` with fields: `key`, `display_name`, `language_scope`, `download_url`, `file_size_bytes`, `sha256_checksum`; include at minimum: `whisper-tiny`, `whisper-base`, `whisper-small`, `whisper-medium` (multilingual variants)
- [ ] TASK-100: Implement `db/repositories/models_repo.rs` — `upsert_model_installation`, `get_model(key)`, `list_installed_models()`, `set_default_for_language(language, key)`, `delete_model(key)`
- [ ] TASK-101: Implement `models/downloader.rs` — async HTTP download (reqwest) to a `.tmp` file in models storage path; emit Tauri event `model-download-progress { key, percent }` during download; rename to final filename on completion; cancel support via `AbortHandle`
- [ ] TASK-102: Implement `models/verify.rs` — compute SHA-256 of downloaded file; compare to registry checksum; return `Ok` or `Err(ChecksumMismatch)`
- [ ] TASK-103: Implement `models/service.rs` — `download_model(key)`: download → verify → upsert DB record as `installed=true`; `delete_model(key)`: remove file + set `installed=false`; `set_default_model(language, key)`
- [ ] TASK-104: Implement `commands/models.rs` — Tauri commands: `list_available_models()` (registry + DB install state merged), `download_model(key)`, `delete_model(key)`, `set_default_model(language, key)`; also `get_download_progress(key)` or rely on events
- [ ] TASK-105: Update `transcription/orchestrator.rs` — resolve model path by reading `transcription.default_model_de` / `transcription.default_model_en` from settings, then look up `local_path` from `model_installations`
- [ ] TASK-106: React: Models page — list with columns: Name, Language, Size, Installed badge, Default badge
- [ ] TASK-107: React: Download button per model — shows progress bar while downloading; changes to "Installed" on completion
- [ ] TASK-108: React: Delete button per installed model (with confirmation dialog)
- [ ] TASK-109: React: Default model selector — separate dropdowns or radio groups for German default and English default (only installed models listed)
- [ ] TASK-110: React: Handle `model-download-progress` event to update progress bar live

## Product/UX Tasks

- [ ] TASK-111: Confirm download URLs are accessible; document fallback if CDN changes
- [ ] TASK-112: Validate storage path default is reasonable (e.g. `%APPDATA%\localvoice\models`)

## QA / Acceptance

- [ ] TASK-113: Verify model downloads fully, checksum passes, DB record created
- [ ] TASK-114: Verify installed model is used for next transcription after being set as default
- [ ] TASK-115: Verify installed model survives app restart (DB record + file both present)
- [ ] TASK-116: Verify deleting a model removes the file and clears the installed flag

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
