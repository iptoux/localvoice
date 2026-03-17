# MS-07 — Models

## What Was Built

A full model management pipeline: static registry of downloadable whisper.cpp GGML models, async HTTP download with progress events, SHA-256 checksum verification, SQLite install records, and a React Models page with live progress bars and language-specific default selectors.

## Key Decisions

- **Static registry vs DB** — `models/registry.rs` holds immutable metadata (URL, size, checksum). The DB `model_installations` table tracks only mutable install state. These are merged at the query layer in `models/service.rs` to produce `ModelInfo`.
- **`.tmp` rename pattern** — downloads are written to `{key}.bin.tmp`; renamed to `{key}.bin` only on success. Failures call `cleanup_tmp` to avoid leaving partial files.
- **Checksum as `None` = skip** — SHA-256 checksums are `None` in the registry for now (TASK-111 deferred). The `verify` module treats `None` as "pass", so the pipeline is wired but not enforced until checksums are confirmed.
- **reqwest 0.12 stream** — `futures_util::StreamExt` streams chunks to disk without buffering the full file in memory; progress events are throttled to one per percent change.
- **Model resolution priority in orchestrator** — explicit override → DB default for language (`models_repo::get_default_path`) → legacy `transcription.model_path` setting → auto-scan of `{app_data_dir}/models/*.bin`.

## Architecture Notes

```
models/
  registry.rs      Static ModelDefinition array — URL, size, checksum
  downloader.rs    Async reqwest stream download → .tmp → rename; emits model-download-progress
  verify.rs        SHA-256 via sha2 crate; None checksum = skip
  service.rs       Orchestrates download/verify/DB upsert, delete, set_default; exposes ModelInfo
db/repositories/
  models_repo.rs   CRUD for model_installations: upsert, get, list_installed, set_default_for_language, get_default_path, mark_uninstalled
commands/
  models.rs        list_available_models, download_model (async), delete_model, set_default_model
src/
  stores/models-store.ts   Zustand: models[], downloading{}, fetch/startDownload/removeModel/setDefault
  pages/Models.tsx         Models list, download progress bars, delete confirm, default dropdowns
```

## Known Limitations / Future Work

- **Cancel in-flight downloads** — `AbortHandle`-based cancellation was deferred; downloads run to completion once started.
- **SHA-256 checksums** — all entries in the registry have `sha256_checksum: None`. TASK-111 tracks confirming the Hugging Face file hashes.
- **Download URL fallback** — HuggingFace CDN is the primary source. If it changes, update `registry.rs` URLs. GitHub releases are documented as fallback.
