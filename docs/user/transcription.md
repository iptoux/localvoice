# Transcription

## What It Does

After you stop a recording, LocalVoice automatically transcribes it offline using
[whisper.cpp](https://github.com/ggerganov/whisper.cpp). No audio ever leaves your computer.

The pill transitions through:
- **Transcribing…** (amber) — whisper.cpp is running
- **Done** / transcript preview (green) — transcription succeeded
- **Error** (red) — something went wrong (see error text in pill)

## First-Time Setup

Transcription requires two files you must provide manually:

### 1. whisper-cli binary

1. Download a release from https://github.com/ggerganov/whisper.cpp/releases
2. Rename it to `whisper-cli-x86_64-pc-windows-msvc.exe`
3. Place it in `src-tauri/binaries/` (development) or alongside the app executable (installed)

Alternatively, set the `WHISPER_BIN_PATH` environment variable to the full path of any
whisper.cpp CLI binary.

### 2. Whisper model

1. Download a model from HuggingFace (search "ggerganov/whisper.cpp") or the releases page.
   Recommended starting model: `ggml-base.bin` (~142 MB, good balance of speed/accuracy).
2. Place it in `%APPDATA%\com.localvoice.app\models\` on Windows.

Alternatively, set `WHISPER_MODEL_PATH` to the full path of the model file, or update
`transcription.model_path` in Settings.

## Changing the Language

1. Open the main window and go to **Settings → Recording → Transcription Language**.
2. Select the language that matches your speech.
3. "Auto-detect" works for any language but is slightly slower.

## Related

- [Recording](recording.md) — how to record audio
- [Developer notes: MS-03 Transcription](../dev/ms03-transcription.md)
