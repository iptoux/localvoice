# i18n — Internationalization

**Status:** Structure prepared, library not yet installed

This directory contains translation files and i18n configuration for LocalVoice.

## Files

| File | Description |
|------|-------------|
| `index.ts` | i18n configuration (requires `i18next` and `react-i18next`) |
| `en.json` | English translations |
| `de.json` | German translations |
| `types.ts` | TypeScript type definitions |

## Installation

When ready to implement full i18n support:

```bash
npm install i18next react-i18next
```

## Current Usage

UI strings are currently hardcoded in components. See [docs/dev/i18n.md](../../docs/dev/i18n.md) for the migration guide.

## Supported Languages

| Code | Language | Status |
|------|----------|--------|
| `en` | English | Prepared |
| `de` | German | Prepared |

## Adding a New Language

1. Copy `en.json` to the new language file (e.g., `fr.json`)
2. Translate all string values
3. Update `src/i18n/index.ts` to include the new language
4. Add the language to `SUPPORTED_UI_LANGUAGES` and `UILanguage` type
