# Transcription

## What It Does

After you stop a recording, LocalVoice transcribes the audio offline. The selected model determines which local engine runs:

- **Whisper.cpp** for GGML `.bin` models.
- **Parakeet.cpp** for GGUF `.gguf` models.
- **NVIDIA NeMo** for optional `.nemo` models.

No audio is sent to a cloud service. When streaming is enabled and the selected model/runtime supports it, LocalVoice can receive partial text before you stop recording. The final transcript still goes through the same cleanup, dictionary rules, history persistence, and clipboard or auto-insert output flow.

In the default **Recording overlay** pill mode, the pill shows only the waveform while recording and hides after stop. In **Classic pill** mode, streaming-capable models can also show live transcript preview text in the pill.

The transcription flow transitions through:

- **Transcribing...** - the selected local engine is running.
- **Live insert** - an optional streaming mode writes worker-emitted deltas while you are still recording.
- **Done** - transcription succeeded and output ran.
- **Error** - something went wrong; the app shows a user-facing error notification or state.

## First-Time Setup

Public installers include the required Whisper and Parakeet sidecar executables. Model weights are not bundled.

1. Open **Models**.
2. Download a Whisper GGML or Parakeet GGUF model.
3. Set it as the default model for the language you use.
4. Record again.

For development builds, run the bootstrap script so Tauri can find the target-triple sidecars in `src-tauri/binaries/`.

## Streaming Mode

Streaming is controlled in **Settings -> Transcription**:

1. Enable **Streaming transcription**.
2. Choose a chunk size. `320 ms` is the default balance between latency and overhead.
3. Keep **Streaming output** on **No live insert** unless you explicitly want LocalVoice to write streaming deltas into the focused application while you speak.

No-live-insert streaming never writes partial text into another app. Live insert writes only deltas returned by the streaming worker; if the worker emits no text before finalization, LocalVoice keeps the normal final output on stop. If the selected model is Whisper, `.nemo`, or another non-streaming model, LocalVoice uses the normal stop-to-transcribe flow.

Parakeet GGUF streaming uses the bundled `parakeet-stream-worker` sidecar. If the worker is missing or the model cannot start a streaming session, LocalVoice falls back to the normal WAV transcription path after recording stops.

## Optional NeMo Runtime

`.nemo` models require a local Python environment with NVIDIA NeMo installed. LocalVoice does not bundle Python, CUDA, or NeMo in public installers.

1. Configure `transcription.nemo.python_path` or use a Python available on `PATH`.
2. Run the NeMo runtime health check from the app before selecting a `.nemo` model.
3. Select a `.nemo` model only after the runtime is reported as available.

If the NeMo health check fails, LocalVoice keeps working with Whisper and Parakeet GGUF choices.

## Changing the Language

1. Open the main window and go to **Settings -> Recording -> Transcription Language**.
2. Select the language that matches your speech, or use auto-detect where supported.
3. Choose a default model whose language metadata includes the selected language.

## Troubleshooting

If transcription fails:

- Confirm the selected model is installed.
- Confirm the runtime shown on the model card is available.
- For `.nemo`, run the NeMo health check and verify the configured Python path.
- For development builds, make sure `whisper-cli-*`, `parakeet-cli-*`, and `parakeet-stream-worker-*` exist in `src-tauri/binaries/`.

See the [developer debugging guide](../dev/transcription-pipeline.md#debugging-transcription-issues) for protocol-level details.

## Related

- [Recording](recording.md)
- [Models](models.md)
- [Developer: Hybrid Runtime](../dev/parakeet-hybrid-runtime.md)
- [Transcription Pipeline](../dev/transcription-pipeline.md)
