# Models

## What It Does

The Models page lets you download, install, and manage local transcription models. LocalVoice supports bundled sidecar engines for Whisper GGML and Parakeet GGUF models, plus optional NVIDIA NeMo support for native `.nemo` checkpoints.

Base installers include the small sidecar executables only. Model weights, `.nemo` checkpoints, CUDA packages, and Python/NeMo environments are downloaded or configured after install.

## How to Use It

1. Open the main window and navigate to **Models** in the sidebar.
2. Review each model card for engine, artifact format, runtime, size, language scope, and streaming support.
3. Click **Download** next to a model to start the download. A progress bar shows bytes transferred and percentage complete.
4. Once installed, the model shows an **Installed** badge and can be selected as a language default.
5. Use the default model controls to choose which installed model is used for each language.
6. To remove a model, click **Delete** and confirm. The file is removed from disk and any matching default is cleared.

## Runtime Types

| Runtime | Artifact | Behavior |
|---|---|---|
| Whisper.cpp | `.bin` GGML | Bundled sidecar, broad language support, stable default path |
| Parakeet.cpp | `.gguf` | Bundled sidecar, portable Parakeet/Nemotron support, streaming-capable metadata |
| NVIDIA NeMo | `.nemo` | Optional Python runtime, highest-fidelity NVIDIA-native path, requires a passing health check |

`.nemo` models show as available in the registry, but they cannot be selected as defaults until the optional NeMo runtime health check passes.

## Models Available

| Family | Formats | Best for |
|---|---|---|
| Whisper Tiny/Base/Small/Medium/Large | GGML `.bin` | General dictation and broad CPU compatibility |
| Nemotron 3.5 ASR Streaming 0.6B | GGUF, `.nemo` | Multilingual streaming workflows |
| Parakeet TDT 0.6B v3 | GGUF, `.nemo` | High-quality multilingual Parakeet transcription |

GGUF models are offered in Q4, Q5, Q8, and F16 variants where available. Lower quantization uses less storage and memory; F16 keeps the most precision.

## Storage Location

Models are stored in the app data directory:

- Windows: `%APPDATA%\localvoice\models\`
- macOS: `~/Library/Application Support/com.localvoice.app/models/`
- Linux: `~/.local/share/com.localvoice.app/models/`

## Related

- [Transcription](transcription.md)
- [Developer: Models](../dev/ms07-models.md)
- [Developer: Hybrid Runtime](../dev/parakeet-hybrid-runtime.md)
