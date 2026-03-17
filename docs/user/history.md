# History

## What It Does

Every successful dictation session is saved automatically. The **History** page lets you browse, search, and manage all past transcriptions.

## How to Use It

1. Double-click the pill (or click **Open** in the tray) to open the main window.
2. Click **History** in the sidebar.
3. The list shows your sessions newest-first with date, language, word count, and a text preview.

### Search

Type in the search box to filter sessions by their transcript content. Results update as you type (after a short debounce).

### Filters

Use the dropdowns and date pickers to narrow by:
- **Language** — German, English, or other detected language
- **From / To** — date range

Click **Clear filters** to reset.

### Session detail

Click any row to open the detail drawer:

- **Cleaned** tab — the processed, capitalised transcript
- **Raw** tab — the original whisper output before post-processing
- **Segments** (expandable) — timestamped segments with optional confidence scores

### Actions

| Action | What it does |
|--------|-------------|
| **Copy** | Copies the currently shown text (Cleaned or Raw) to the clipboard |
| **Export ↗** | Opens a save-file dialog and writes the session as plain text |
| **Delete** | Permanently removes the session (requires a second click to confirm) |

### Export all / Export page

The **Export page ↗** button in the top-right exports all sessions currently visible in the list as a plain text file.

## Related

- [Output Workflow](output.md)
- [Developer docs — MS-05](../dev/ms05-history.md)
