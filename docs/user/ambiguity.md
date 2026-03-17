# Ambiguity Suggestions

## What It Does

LocalVoice tracks words and phrases that whisper transcribes with low confidence (below 60% by default). When the same phrase appears at least 3 times with low confidence, it surfaces in the **Suggestions** tab of the Dictionary page.

Each suggestion shows:
- The phrase whisper keeps misreading
- How many times it has been seen
- The average confidence score
- A suggested replacement (if a matching correction rule already exists)

## How to Use It

1. Open the **Dictionary** page from the main navigation.
2. Click the **Suggestions** tab.
3. For each listed phrase:
   - If a suggested replacement is shown: click **Accept** to create a correction rule automatically, or **Edit** to modify the replacement first.
   - If no suggestion is shown: click **Create Rule…**, type the correct word or phrase, and confirm.
   - Click **Dismiss** to remove the suggestion without creating a rule. It will reappear if whisper keeps struggling with that phrase.

Once you accept a suggestion, a correction rule is created and applied to all future transcriptions automatically.

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `ambiguity.confidence_threshold` | `0.6` | Segments below this confidence are flagged |
| `ambiguity.min_occurrences` | `3` | Minimum appearances before a phrase is surfaced |

Both settings can be adjusted under Settings → Transcription.

## Related

- [Dictionary & Correction Rules](dictionary.md)
- [Developer: MS-09 Ambiguity](../dev/ms09-ambiguity.md)
