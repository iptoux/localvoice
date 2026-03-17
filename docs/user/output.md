# Output Workflow

## What It Does

After every successful transcription, LocalVoice automatically delivers the text to you. You choose how in **Settings → Output**:

| Mode | What happens |
|------|-------------|
| **Clipboard** (default) | The transcribed text is copied to your clipboard. Paste it anywhere with `Ctrl+V`. |
| **Auto-insert** | The text is copied to your clipboard **and** immediately pasted into whatever app was focused when you triggered the recording via `Ctrl+V`. Your previous clipboard content is restored afterwards. |

## How to Use It

1. Press `Ctrl+Shift+Space` (or your configured shortcut) to start recording.
2. Speak, then press the shortcut again to stop.
3. The pill shows a spinner while transcribing.
4. On success, the pill turns green and shows:
   - A **"Copied"** or **"Inserted"** badge depending on your output mode.
   - A short preview of the transcribed text.
5. The pill returns to its ready state automatically after about 2 seconds.

## Changing the Output Mode

1. Double-click the pill (or click **Open** in the tray menu) to open the main window.
2. Navigate to **Settings**.
3. Under **Output**, select **Clipboard** or **Auto-insert**.

## Notes on Auto-insert

- Auto-insert works by writing the text to the clipboard and sending `Ctrl+V` to the focused app. Most desktop applications support this.
- Your previous clipboard content is restored a moment after the paste.
- If Auto-insert fails (e.g. the focused app does not accept `Ctrl+V`), the text is still available in your clipboard as a fallback and the badge shows **"Failed"** briefly.
- Browser-based apps and some games may intercept or ignore the simulated keypress — use Clipboard mode for those.

## Related

- [Recording](recording.md)
- [Transcription](transcription.md)
- [Developer docs — MS-04](../dev/ms04-output.md)
