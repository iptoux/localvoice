# Text Insertion

## What It Does

After transcription, LocalVoice can either copy text to the clipboard or automatically paste it into the focused application. The insert flow uses clipboard + Ctrl+V simulation with configurable timing and automatic fallback.

## How to Use It

### Choosing Output Mode

Go to **Settings → Output → Output mode** and select:

- **Clipboard only** — text is copied to clipboard; paste manually with Ctrl+V
- **Auto-insert** — text is pasted automatically into the focused app via Ctrl+V; your previous clipboard content is restored afterwards

### Adjusting Insert Delay

When using Auto-insert, you can adjust the **Insert delay** slider (50–500 ms):

- **Lower values** (50–100 ms) — faster insertion, works well in most apps
- **Higher values** (200–500 ms) — use if text is lost or garbled in slower apps

### Long Texts

Texts longer than 4 000 characters are automatically split into chunks, each pasted sequentially. This prevents buffer overflow in apps that struggle with very large clipboard payloads.

### Fallback Behavior

If auto-insert fails (e.g. the target app blocks paste), LocalVoice:

1. Keeps the text on the clipboard
2. Shows "Text copied — paste manually" in the pill
3. You can paste with Ctrl+V yourself

### Target App Detection

When using auto-insert, LocalVoice records which application was in focus (e.g. "Notepad", "Visual Studio Code"). This information is stored with the session in history.

## Related

- [Output](output.md)
- [Recording](recording.md)
