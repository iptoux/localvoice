# Session Reprocessing

## What It Does

Session reprocessing lets you re-run transcription on a previously recorded session using a different whisper model or language. This is useful when you switch to a more accurate model or want to try a different language detection.

## How to Use It

### Enable Audio Retention

Reprocessing requires the original audio file. By default, audio files are deleted after transcription.

1. Open **Settings > Audio Storage**
2. Turn on **Keep audio files**
3. Optionally adjust the **Retention period** (how long files are kept) and **Max storage** limit

All future recordings will now save their audio for reprocessing.

### Reprocess a Session

1. Go to **History** and select a session
2. Click the **Reprocess** button in the action bar (only visible if audio was kept)
3. Choose a different **language** and/or **model**
4. Click **Reprocess** — the session text will update with the new transcription

### Compare Results

After reprocessing, the session detail view shows three tabs:
- **Cleaned** — the current post-processed text
- **Raw** — the current raw whisper output
- **Original** — the raw text from before the first reprocess

A badge shows how many times the session has been reprocessed.

## Post-Processing Toggles

Individual post-processing steps can be toggled in **Settings > Transcription**:

- **Auto-capitalization** — capitalize the first word of each sentence
- **Auto-punctuation** — add missing punctuation
- **Remove filler words** — strip "uh", "um", "aeh" etc.
- **Auto-apply dictionary rules** — apply your correction rules automatically

These settings affect both new recordings and reprocessed sessions.

## Related

- [History](history.md)
- [Models](models.md)
- [Dictionary & Correction Rules](dictionary.md)
- [Developer docs: MS-14](../dev/ms14-reprocess-pipeline.md)
