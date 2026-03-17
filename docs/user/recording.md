# Recording

## What It Does

LocalVoice listens to your microphone and captures your voice via a global keyboard shortcut.
While recording, the pill shows a pulsing red indicator and an elapsed timer.
All audio stays on your device — nothing is sent to the internet.

## How to Use It

1. **Start recording** — press the global shortcut (default: `Ctrl+Shift+Space`) from anywhere on your computer.
   The pill turns red and shows "Listening…" with a running timer.

2. **Stop recording** — press the shortcut again.
   The pill shows "Transcribing…" while the audio is processed locally (MS-03 and later).

3. **Cancel recording** — press the shortcut a third time while "Transcribing…" is shown,
   or wait for the result. Cancelling discards the audio without saving anything.

## Changing the Microphone

1. Open the main window (double-click the pill or use the tray menu → Open App).
2. Go to **Settings**.
3. Under **Recording**, choose your microphone from the dropdown.
   Selecting "System default" always uses whatever device is set as default in Windows.

## Changing the Shortcut

The shortcut is stored in the settings database under the key `recording.shortcut`.
A graphical shortcut editor is planned for a later release. For now, the Settings page
shows the current shortcut with instructions.

## Related

- [Getting Started](getting-started.md)
- [Developer notes: MS-02 Recording](../dev/ms02-recording.md)
