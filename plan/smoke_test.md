# LocalVoice v0.1 — Smoke Test Checklist

Execute these steps in order on a clean install (no DB, no models). Check each box when verified.

---

## 1. First Launch & Onboarding

- [ ] App launches without crashing; pill window appears in top-left corner
- [ ] Main window opens from tray or double-click
- [ ] Onboarding overlay is shown automatically (no model installed)
- [ ] "Download a Model" button navigates to Models page and closes overlay
- [ ] "Skip for now" closes the overlay without navigating

## 2. Model Download & Install

- [ ] Models page lists all available models with correct name and size
- [ ] Clicking "Download" on a model starts the download; progress bar updates in real time
- [ ] After download completes, model shows "Installed" badge
- [ ] Delete a model; "Installed" badge disappears; file is removed from disk
- [ ] Setting DE / EN default model persists across app restarts

## 3. Recording

- [ ] Global shortcut (`Ctrl+Shift+Space`) starts recording; pill turns red
- [ ] Press shortcut again (or stop); pill transitions to processing state
- [ ] Silence timeout automatically stops recording after ~1.5 s of silence
- [ ] Cancel recording (via command or tray) returns pill to idle

## 4. Transcription

- [ ] Speech is transcribed and text appears in pill (success state)
- [ ] Transcribed text is copied to clipboard
- [ ] Auto-insert mode: text is pasted into the focused application
- [ ] Transcription result appears in History within seconds
- [ ] Missing model shows actionable notification: "No model installed…"
- [ ] Transcription error shows OS notification (if notifications.on_error = true)

## 5. History

- [ ] History page lists sessions newest-first
- [ ] Clicking a session opens the detail drawer with segments
- [ ] Search by text filters results
- [ ] Date range filter works correctly
- [ ] Delete session removes it from list
- [ ] Export selected sessions writes a valid JSON / txt file

## 6. Dashboard

- [ ] Stats (total sessions, words, avg WPM) update after new transcriptions
- [ ] Timeseries chart renders without errors
- [ ] Language breakdown shows correct proportions
- [ ] Switching date range (7d / 30d / 90d / all) re-fetches data

## 7. Correction Rules

- [ ] Add a correction rule (e.g. "clawd" → "Claude")
- [ ] Record a clip saying the source phrase; verify it is replaced in output
- [ ] Usage count increments after each matched transcription
- [ ] Disable a rule; verify it no longer fires
- [ ] Delete a rule; verify it is gone

## 8. Ambiguity Suggestions

- [ ] Record the same low-confidence phrase 3+ times
- [ ] Phrase appears in Dictionary → Suggestions tab
- [ ] Accept suggestion: correction rule is created; suggestion disappears
- [ ] Dismiss suggestion: it disappears; re-appears after +5 more occurrences

## 9. Settings & System

- [ ] Autostart toggle writes / removes Windows registry entry (verify in Task Manager → Startup)
- [ ] Error notification toggle: disable → transcription error shows no notification
- [ ] Success notification toggle: enable → transcription success shows OS notification
- [ ] "Reset to defaults" restores all settings to initial values
- [ ] Microphone selector changes the recording device
- [ ] Transcription language setting is passed to whisper

## 10. Window Persistence

- [ ] Drag the pill to a new screen position; restart app → pill appears at saved position
- [ ] Resize the main window; restart → window opens at saved size
- [ ] Move the main window; restart → window appears at saved position

## 11. Logs

- [ ] Logs page shows warn/error entries after a failed transcription
- [ ] Level filter (Warn / Error / All) works
- [ ] Export saves a valid JSON file with log entries
- [ ] Clear removes all entries from the list

## 12. Error Paths

- [ ] No model installed → helpful notification, not a crash
- [ ] Microphone unavailable → helpful error message
- [ ] App handles rapid start/stop recording without crashing

---

## Sign-Off

| Tester | Date | Result |
|--------|------|--------|
| | | |
