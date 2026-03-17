# Models

## What It Does

The Models page lets you download, install, and manage local whisper.cpp transcription models. All models are stored on your device — no cloud connection is needed for transcription once a model is installed.

## How to Use It

1. Open the main window and navigate to **Models** in the sidebar.
2. The page lists all available models with their name, size, and language support.
3. Click **Download** next to a model to start the download. A progress bar shows bytes transferred and percentage complete.
4. Once installed, the model shows an **Installed** badge and a **Delete** button.
5. Use the **Default models** dropdowns at the top to set which installed model is used for German and English transcription.
6. To remove a model, click **Delete** and confirm — the file is removed from disk and the default is cleared.

## Models Available

| Model | Size | Best for |
|-------|------|----------|
| Tiny  | ~75 MB  | Fast transcription, lower accuracy |
| Base  | ~142 MB | Good balance of speed and accuracy |
| Small | ~466 MB | Higher accuracy, moderate speed |
| Medium | ~1.5 GB | Best accuracy, slower transcription |

All models support German and English (multilingual).

## Storage Location

Models are stored in the app data directory:
- Windows: `%APPDATA%\localvoice\models\`

## Related

- [Transcription](transcription.md)
- [Developer: MS-07 Models](../dev/ms07-models.md)
