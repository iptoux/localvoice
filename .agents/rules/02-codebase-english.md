# Rule 02 — Codebase Language: English

## Rule

All code, comments, identifiers, commit messages, and technical documentation in this repository must be written in English.

## Applies To

- Source files (Rust, TypeScript, CSS, SQL, config files)
- Code comments and inline documentation (`//`, `/* */`, `/** */`, `#`)
- Variable names, function names, type names, module names
- Git commit messages and branch names
- Files in `plan/`, `docs/`, and any other repo-tracked markdown
- Error messages and log strings within source code

## Required Behavior

- Write all new code, names, and comments in English — no exceptions
- If existing code contains non-English identifiers or comments, migrate them to English when touching that file
- User-facing UI strings (labels, toasts, tooltips) are exempt — they follow the app's locale settings (de/en)
- `CLAUDE.md` and `.claude/rules/` files may use German if the user communicates in German, but any rule that references code constructs must name them in English

## Never Do

- Add German (or any non-English) variable names, function names, or comments to source files
- Write commit messages in German
- Name branches with German words
