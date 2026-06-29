# Hybrid Transcription Pipeline

This document explains how LocalVoice runs local transcription through a shared orchestrator and engine-specific runtimes.

## Overview

LocalVoice does not link against native transcription libraries directly. Rust spawns local child processes and normalizes their output into one `TranscriptionResult` shape:

- `WhisperCppEngine` runs `whisper-cli` for GGML `.bin` models.
- `ParakeetCppEngine` runs `parakeet-cli` for GGUF `.gguf` models.
- `NemoEngine` runs an optional Python worker for NVIDIA `.nemo` checkpoints.

After an engine returns raw text, segments, and optional words, the existing post-processing path still applies:

1. Whitespace normalization.
2. Optional filler-word removal.
3. Optional capitalization and punctuation.
4. Dictionary and correction rules.
5. History persistence, output, and UI events.

Partial streaming updates are allowed where supported, but final output semantics stay unchanged.

## Engine Selection

The orchestrator resolves the runtime from the installed model registry first. If no registry model is selected, legacy Whisper settings and direct model paths remain supported.

| Field | Purpose |
|---|---|
| `engine` | `whisper-cpp`, `parakeet-cpp`, or `nemo` |
| `artifactFormat` | `ggml-bin`, `gguf`, or `nemo` |
| `runtime` | `bundled-sidecar`, `optional-nemo`, or `external-path` |
| `supportsStreaming` | Whether partial UI updates can be emitted |
| `supportsWordTimestamps` | Whether word-level timestamps can be stored |
| `supportsConfidence` | Whether confidence scores are expected |

The model registry in `src-tauri/src/models/registry.rs` is the source of truth for engine metadata, artifact names, checksums, license URLs, and language locales.

## Sidecar Resolution

Whisper and Parakeet use Tauri `externalBin` entries and are named without target triples in `tauri.conf.json`:

```json
{
  "externalBin": [
    "binaries/whisper-cli",
    "binaries/parakeet-cli"
  ]
}
```

Tauri validates target-triple files during local checks and builds. Bootstrap scripts and CI place binaries such as:

- `src-tauri/binaries/whisper-cli-x86_64-pc-windows-msvc.exe`
- `src-tauri/binaries/parakeet-cli-x86_64-pc-windows-msvc.exe`
- `src-tauri/binaries/parakeet-cli-aarch64-apple-darwin`
- `src-tauri/binaries/parakeet-cli-x86_64-unknown-linux-gnu`

Runtime resolution order:

| Priority | Whisper | Parakeet | Notes |
|---|---|---|---|
| 1 | `WHISPER_BIN_PATH` | `PARAKEET_BIN_PATH` | Absolute override |
| 2 | App executable directory | App executable directory | Production bundle |
| 3 | `src-tauri/binaries/` | `src-tauri/binaries/` | Development |
| 4 | Tauri resource directory | Tauri resource directory | Packed resources |
| 5 | `PATH` | `PATH` | Last fallback |

## Whisper Protocol

Primary invocation:

```bash
whisper-cli \
  -m <model_path> \
  -f <audio_file.wav> \
  -l <language> \
  -ojf \
  -of <output_prefix>
```

The JSON output file is parsed for segments and confidence. If JSON mode fails, LocalVoice retries with a plain text invocation and parses timestamped lines where available.

## Parakeet Protocol

Parakeet GGUF invocation:

```bash
parakeet-cli transcribe \
  --model <model_path> \
  --input <audio_file.wav> \
  --json \
  --timestamps \
  --lang <language>
```

`src-tauri/src/transcription/parakeet_parser.rs` accepts fixture-driven JSON shapes and normalizes them to transcript text, segments, and words. `src-tauri/src/transcription/parakeet_sidecar.rs` owns sidecar resolution, process spawning, timeout handling, and smoke tests.

## NeMo Worker Protocol

NeMo is optional and app-managed. The Python worker is bundled as a resource at:

```text
src-tauri/resources/nemo_worker/localvoice_nemo_worker.py
```

Rust starts the worker directly with the configured Python interpreter. No Tauri shell plugin is required.

The worker script is packaged with `manifest.json`, which declares the protocol version, entrypoint, runtime, message names, and optional Python modules. The worker itself speaks NDJSON. Supported message types:

| Type | Direction | Purpose |
|---|---|---|
| `health` | CLI mode | Check Python, NeMo import, and runtime readiness |
| `load` | Rust -> worker | Load a `.nemo` checkpoint for the worker process |
| `transcribe_file` | Rust -> worker | Transcribe a WAV file |
| `stream_chunk` | Rust -> worker | Reserved for streaming chunks |
| `finalize` | Rust -> worker | Reserved for ending a stream |
| `cancel` | Rust -> worker | Reserved for cancellation |

The initial worker supports health checks and file transcription. Streaming message types are part of the protocol contract and return a clear unsupported response until the warm streaming loop is enabled.

## Runtime Settings

Migration 10 seeds:

| Key | Default | Purpose |
|---|---|---|
| `transcription.default_engine` | `whisper-cpp` | Preferred engine |
| `transcription.preferred_runtime` | `bundled-sidecar` | Runtime preference |
| `transcription.streaming.enabled` | `false` | Enables partial streaming UI updates |
| `transcription.streaming.chunk_ms` | `320` | Target chunk size for streaming engines |
| `transcription.nemo.python_path` | empty | Optional Python interpreter path |
| `transcription.parakeet.device` | empty | Reserved Parakeet device selection |

## Persistence

Sessions now store engine metadata:

- `sessions.engine`
- `sessions.model_artifact_format`
- `sessions.runtime`

Segment storage is unchanged. Word-level timestamps are stored in `session_words` when an engine returns them.

## Build And Release Packaging

CI runs `.github/actions/setup-whisper` and `.github/actions/setup-parakeet-cpp` before Rust tests and release builds. The Parakeet action pins `mudler/parakeet.cpp` to `v0.3.2`, downloads CPU/portable assets, verifies SHA-256 checksums, and writes target-triple sidecar binaries.

Release jobs audit that the bundled sidecars and NeMo worker resource are present before building installers. Public installers bundle:

- `whisper-cli`
- `parakeet-cli`
- NeMo worker script and manifest resources

Public installers do not bundle:

- Whisper, Parakeet, Nemotron, or `.nemo` model weights.
- Python, NeMo, CUDA, or Vulkan runtime stacks.
- GPU-specific Parakeet runtime packs.

## Debugging Transcription Issues

### Environment Overrides

| Variable | Purpose |
|---|---|
| `WHISPER_BIN_PATH` | Override `whisper-cli` location |
| `WHISPER_MODEL_PATH` | Legacy Whisper model override |
| `PARAKEET_BIN_PATH` | Override `parakeet-cli` location |
| `RUST_LOG` | Rust logging level |

### Common Errors

| Error | Cause | Solution |
|---|---|---|
| `whisper-cli binary not found` | Missing Whisper sidecar | Run bootstrap or set `WHISPER_BIN_PATH` |
| `parakeet-cli binary not found` | Missing Parakeet sidecar | Run bootstrap or set `PARAKEET_BIN_PATH` |
| `Model path does not exist` | Model deleted or download failed | Re-download from Models |
| `.nemo runtime is not available` | Python/NeMo health check failed | Configure Python and install NeMo |
| `sidecar failed` | Runtime/model/audio mismatch | Check model format and sidecar version |

### Manual Checks

```bash
src-tauri/binaries/whisper-cli-x86_64-pc-windows-msvc.exe --help
src-tauri/binaries/parakeet-cli-x86_64-pc-windows-msvc.exe --help
```

For NeMo:

```bash
python src-tauri/resources/nemo_worker/localvoice_nemo_worker.py --health
```

## Related Documentation

- [Hybrid Runtime Feature](parakeet-hybrid-runtime.md)
- [ADR-001: whisper.cpp as Sidecar Process](../adrs/001-whispercpp-sidecar.md)
- [User: Transcription](../user/transcription.md)
- [User: Models](../user/models.md)
- [Post-Processing Pipeline](ms14-reprocess-pipeline.md)
