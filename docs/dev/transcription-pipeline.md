# Whisper.cpp Sidecar Pipeline

This document explains how LocalVoice integrates whisper.cpp as a **sidecar process** — spawning it as a separate child process and communicating via stdin/stdout.

## Overview

LocalVoice does not link against whisper.cpp directly. Instead, it runs the `whisper-cli` executable as a subprocess. This architectural choice:

- **Simplifies builds** — no C++ compilation required during Rust builds
- **Isolates failures** — transcription crashes don't crash the main app
- **Enables flexibility** — swap whisper binaries without recompiling

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        LocalVoice (Tauri)                       │
│                                                                 │
│  ┌──────────────┐    ┌─────────────────┐    ┌────────────────┐ │
│  │ Audio Capture │───▶│ WAV File (.wav)│───▶│ Transcription  │ │
│  │   (cpal)     │    │   (temp file)   │    │  Orchestrator  │ │
│  └──────────────┘    └─────────────────┘    └───────┬────────┘ │
│                                                       │         │
│                                                       ▼         │
│                                             ┌────────────────┐ │
│                                             │  Post-Process  │ │
│                                             │  Pipeline      │ │
│                                             └───────┬────────┘ │
│                                                     │         │
└─────────────────────────────────────────────────────┼─────────┘
                                                      │
                                                      ▼
┌─────────────────────────────────────────────────────────────────┐
│                    whisper-cli (Sidecar)                        │
│                                                                 │
│   stdin: nothing (file path passed as argument)                │
│   stdout: transcription text or JSON                           │
│   stderr: progress logs, warnings                               │
│                                                                 │
│   Binary location: see "Binary Resolution" below                │
└─────────────────────────────────────────────────────────────────┘
```

## Binary Resolution

The Rust backend searches for `whisper-cli` in this order:

| Priority | Location | Notes |
|----------|----------|-------|
| 1 | `WHISPER_BIN_PATH` env var | Absolute path override |
| 2 | Next to app executable | Production bundle |
| 3 | `src-tauri/binaries/` | Development mode |
| 4 | Tauri resource dir | Packed resources |
| 5 | System `PATH` | Fallback |

### Binary Names Checked

Windows: `whisper-cli.exe`, `main.exe`  
Unix: `whisper-cli`, `main`

### Model Resolution

Models are resolved similarly:

| Priority | Location | Notes |
|----------|----------|-------|
| 1 | `WHISPER_MODEL_PATH` env var | Absolute path override |
| 2 | `transcription.model_path` setting | From database |
| 3 | `{app_data}/models/*.bin` | Auto-scanned |

## Command-Line Protocol

### Primary Invocation (Full Features)

```bash
whisper-cli \
  -m <model_path> \
  -f <audio_file.wav> \
  -l <language> \
  -ojf \
  -of <output_prefix>
```

| Flag | Description |
|------|-------------|
| `-m` | Model file path |
| `-f` | Input audio file (WAV) |
| `-l` | Language code (`de`, `en`, `auto`, etc.) |
| `-ojf` | Output JSON with timestamps and confidence |
| `-of` | Output file prefix (`.json` extension added) |

### Fallback Invocation (Compatibility)

If the primary invocation fails, LocalVoice retries with minimal flags:

```bash
whisper-cli \
  -m <model_path> \
  -f <audio_file.wav> \
  -l <language>
```

This older format outputs plain text to stdout without JSON.

## Output Formats

### JSON Output (`-ojf`)

When successful, whisper-cli writes a JSON file containing:

```json
{
  "transcription": [
    {
      "offsets": { "from": 0, "to": 2500 },
      "text": " Hello world",
      "tokens": [
        { "p": 0.95 },
        { "p": 0.92 }
      ]
    }
  ]
}
```

LocalVoice parses this to extract:
- **Segment timestamps** (`start_ms`, `end_ms`)
- **Confidence scores** (average token probability per segment)

### Plain Text Output (Fallback)

```
[00:00:00.000 --> 00:00:02.500]   Hello world
[00:00:02.500 --> 00:00:05.000]   This is a test.
```

Lines are parsed with a regex: `[TIMESTAMP] text`

## Post-Processing Pipeline

After whisper returns, LocalVoice applies:

1. **Whitespace normalization** — collapse multiple spaces, trim
2. **Filler-word removal** (optional) — removes "uh", "um", etc.
3. **Capitalization** (optional) — sentence-start capitals
4. **Punctuation** (optional) — adds periods, commas
5. **Correction rules** — user dictionary replacements

## Debugging Transcription Issues

### Enable Verbose Logging

Run the app with `RUST_LOG=debug`:

```bash
set RUST_LOG=debug
cargo tauri dev
```

This outputs whisper CLI invocations and stderr from the sidecar.

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| `whisper-cli binary not found` | Binary not in expected locations | Set `WHISPER_BIN_PATH` or place binary in `src-tauri/binaries/` |
| `Failed to spawn whisper-cli` | Binary not executable or missing DLLs | Check DLLs are co-located with the executable |
| `Model path does not exist` | Model file missing or moved | Update model path in Settings, or set `WHISPER_MODEL_PATH` |
| `whisper-cli failed (exit code N)` | Model/audio incompatibility or corrupted file | Try a different model, re-download model |

### Verify Binary Works Manually

Test the whisper binary directly from command line:

```bash
cd src-tauri/binaries
whisper-cli.exe -m ../models/ggml-base.bin -f test.wav -l de
```

### Check JSON Output

If transcription succeeds but segments appear empty:

1. Enable debug logging
2. Look for the JSON file path in logs
3. Inspect the file directly:

```bash
type %TEMP%\localvoice_out_*.json
```

### DLL Resolution on Windows

If you see `Entry Point Not Found` or similar DLL errors:

1. Ensure `whisper.dll` and `ggml.dll` are in the same directory as `whisper-cli.exe`
2. Or set `PATH` to include the DLL directory:

```powershell
$env:PATH = "C:\path\to\whisper\bin;$env:PATH"
cargo tauri dev
```

## Swapping Whisper Builds

### Using a Custom Binary

1. Build or download a whisper.cpp binary
2. Either:
   - Place it in `src-tauri/binaries/` named `whisper-cli.exe`
   - Set `WHISPER_BIN_PATH=C:\full\path\to\your\whisper.exe`

### Using Different Model Files

Models are GGML format files (`.bin`). Sources:

- Official releases: https://github.com/ggerganov/whisper.cpp/releases
- HuggingFace: https://huggingface.co/ggerganov/whisper.cpp

Popular models by size:

| Model | Size | Speed | Quality |
|-------|------|-------|---------|
| `ggml-tiny.bin` | 75 MB | Fastest | Basic |
| `ggml-base.bin` | 142 MB | Fast | Good |
| `ggml-small.bin` | 466 MB | Medium | Better |
| `ggml-medium.bin` | 1.5 GB | Slow | High |
| `ggml-large.bin` | 2.9 GB | Slowest | Best |

### Build whisper.cpp from Source

```bash
# Clone the repository
git clone https://github.com/ggerganov/whisper.cpp.git
cd whisper.cpp

# Build the CLI
mkdir build && cd build
cmake ..
cmake --build . --config Release

# The binary will be in: build/bin/Release/whisper-cli.exe
```

### CMake Build Options

Enable additional features when building:

```bash
cmake .. -DBUILD_SHARED_LIBS=ON -DWHISPER_ENABLE_CLARBIE=ON
```

Or for CUDA support (if available):

```bash
cmake .. -DWHISPER_CUBLAS=ON
```

## Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `WHISPER_BIN_PATH` | Override whisper binary location | `C:\tools\whisper\whisper-cli.exe` |
| `WHISPER_MODEL_PATH` | Override model location | `C:\models\ggml-base.bin` |
| `RUST_LOG` | Logging level | `debug`, `info`, `warn` |

## File Locations

| File | Default Location |
|------|------------------|
| App data | `%APPDATA%\com.localvoice.app\` |
| Models | `%APPDATA%\com.localvoice.app\models\` |
| Temp audio | System temp directory |
| Persisted audio | `%APPDATA%\com.localvoice.app\audio\` |

## Related Documentation

- [ADR-001: whisper.cpp as Sidecar Process](../adrs/001-whispercpp-sidecar.md)
- [MS-03 Transcription](../adrs/ms03-transcription.md)
- [User: Transcription Setup](../user/transcription.md)
- [Post-Processing Pipeline](./ms14-reprocess-pipeline.md)
