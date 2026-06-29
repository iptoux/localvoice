# Parakeet Hybrid Runtime

## What Was Built

LocalVoice now has a hybrid transcription runtime that keeps Whisper as the stable default while adding:

- Parakeet GGUF transcription through bundled `parakeet-cli`.
- Optional NVIDIA NeMo `.nemo` transcription through a Python worker.
- Model registry metadata for engine, artifact format, runtime, streaming support, word timestamps, confidence support, license, and language locales.
- Release-safe packaging that bundles sidecars and worker scripts, but never bundles model weights or Python/CUDA/NeMo stacks.

## Key Decisions

- **Sidecar first:** Parakeet GGUF uses `parakeet.cpp` as a sidecar, matching the existing Whisper architecture and avoiding C++ FFI/linking complexity.
- **NeMo is optional:** `.nemo` models require a passing runtime health check before they can be selected as defaults.
- **Release size stays controlled:** Base installers include CPU/portable sidecars only. GPU-specific Parakeet packs, CUDA, Vulkan, Python, and model weights are deferred to optional runtime packs.
- **Pinned upstream:** CI pins `mudler/parakeet.cpp` to `v0.3.2`. Any update must change the setup action manifest and pass checksum verification.
- **Final output semantics are unchanged:** Streaming can update UI state, but final text still flows through post-processing, dictionary rules, history, and clipboard/insert output.

## Architecture Notes

```
src-tauri/src/transcription/
  engine.rs             engine metadata and runtime descriptors
  orchestrator.rs       engine selection and final result normalization
  parakeet_sidecar.rs   parakeet-cli resolution and process execution
  parakeet_parser.rs    JSON fixture parser for GGUF transcripts
  parakeet_runtime.rs   loader path setup for Parakeet runtime resources
  nemo_worker.rs        Python worker health and file transcription bridge
  types.rs              shared segment, word, and result payloads

src-tauri/parakeet-runtime/
  generated runtime libraries staged by CI/release builds

src-tauri/resources/nemo_worker/
  localvoice_nemo_worker.py
  manifest.json

.github/actions/setup-parakeet-cpp/
  action.yml            pinned sidecar download, checksum verification, smoke test
```

The model registry is the contract between UI, database, and runtime selection. `ModelDefinition` includes artifact/runtime metadata, while `ModelRuntime` is the resolved installed model used by the orchestrator.

## Build And CI

- `scripts/bootstrap.ps1` and `scripts/bootstrap.sh` install `parakeet-cli-*` for local development.
- `ci.yml` runs `setup-whisper` and `setup-parakeet-cpp` before Rust tests.
- `release.yml` runs both setup actions before each platform build.
- Release jobs audit `whisper-cli-*`, `parakeet-cli-*`, `parakeet-stream-worker-*`, `src-tauri/parakeet-runtime/`, `resources/nemo_worker/localvoice_nemo_worker.py`, and `resources/nemo_worker/manifest.json`.
- `.nemo` load/streaming tests remain manual or self-hosted GPU work because GitHub-hosted runners do not provide a stable NeMo/CUDA environment.

## Public Release Behavior

Fresh installs work without NeMo installed. Users can download Whisper or Parakeet GGUF models from the Models page and transcribe locally with bundled sidecars.

`.nemo` models remain visible but runtime-gated:

- Health check fails: model can be downloaded, but default selection is rejected.
- Health check passes: model can be selected and the Python worker performs transcription.
- Runtime errors: app degrades cleanly back to other installed GGUF/Whisper options.

## Known Limitations / Future Work

- The NeMo worker currently implements health checks and file transcription. `stream_chunk`, `finalize`, and `cancel` are reserved protocol messages and return unsupported responses until warm streaming is enabled.
- Parakeet streaming metadata is present, but UI partial-update plumbing is still conservative.
- GPU Parakeet packs are intentionally not bundled until artifact size, code signing, and driver compatibility are validated.
- The release action smoke test verifies that Parakeet sidecars start with their packaged runtime paths; deeper `parakeet-cli info` behavior should be expanded per platform if the upstream CLI guarantees a stable info/version command.
