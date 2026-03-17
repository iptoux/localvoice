# Dictionary & Correction Rules

## What It Does

The Dictionary page lets you teach LocalVoice how to fix words that whisper.cpp consistently mishears. Correction rules are applied automatically to every transcript before it reaches your clipboard.

## Correction Rules

A rule maps a **heard text** (what whisper produces) to a **corrected text** (what you actually said).

Example: if whisper keeps transcribing "Claude" as "clawd", add a rule:
- Heard: `clawd`
- Corrected: `Claude`

From that point on, every transcript containing "clawd" (case-insensitive) is automatically fixed.

### How to add a rule

1. Open **Dictionary** → **Rules** tab.
2. Click **+ Add rule**.
3. Enter the heard text and the corrected text.
4. Optionally restrict the rule to German or English only.
5. Click **Save**.

### Disable without deleting

Toggle the green switch on any rule row to disable it temporarily. Disabled rules are greyed out and skipped during transcription.

### Usage count

The number shown after each rule (`3×`) is how many times it has fired. Rules are sorted by usage count so the most reliable ones appear first.

## Terms

The Terms tab stores reference entries (names, acronyms, product names) for future features like ambiguity detection and suggestions. Adding a term here does not affect transcription directly.

## Related

- [Transcription](transcription.md)
- [Developer: MS-08 Dictionary](../dev/ms08-dictionary.md)
