# Pill Modes

## What It Does

LocalVoice has two pill modes:

- **Recording overlay** is the default. The pill is hidden while idle, appears at the bottom center of the screen while recording, and shows only the live waveform.
- **Classic pill** keeps the older always-available compact pill with idle, processing, success, error, transcript preview, and expanded quick actions.

Both modes use the same recording, transcription, history, dictionary, clipboard, and auto-insert pipeline. The mode only changes how the pill window is presented.

## Recording Overlay

1. Open **Settings -> Appearance**.
2. Set **Pill mode** to **Recording overlay**.
3. Press the global shortcut or hold it when Push-to-Talk is enabled.
4. The waveform pill appears bottom-center while LocalVoice records.
5. Stop or release the shortcut. The overlay hides while transcription and output continue in the background.

The overlay is fixed in place and is not draggable. It never shows transcript text; streaming text is available only through live insert or the final output flow.

## Classic Pill

1. Open **Settings -> Appearance**.
2. Set **Pill mode** to **Classic pill**.
3. Optionally set **Default view** to **Classic pill** if you want it visible on launch.

The classic pill supports:

- Idle state with LocalVoice status.
- Waveform while recording.
- Processing, success, and error states.
- Streaming transcript preview while recording, when the selected model supports streaming.
- Expanded view with transcript preview, language/model metadata, start/stop, copy, History, and Settings actions.
- Dragging and persisted position.

## Related

- [Recording](recording.md)
- [Getting Started](getting-started.md)
- [Transcription](transcription.md)
