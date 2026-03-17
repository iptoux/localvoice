# Rule 01 — Mask Credentials on Output

## Rule

Never output secrets, credentials, API keys, tokens, passwords, or private keys in plain text — neither in chat responses, code comments, log statements, nor in any file written to disk.

## Applies To

- All files written or edited in this repository
- All terminal output (bash commands, logs, debug prints)
- All chat responses (including code blocks and inline code)

## Required Behavior

- If a secret appears in a file being read, mask it in responses: show only the first 4 characters followed by `****` (e.g. `sk-a****`)
- If code needs a secret value, always use environment variables or a `.env` file — never hardcode the value
- If a `.env` file is found, read it for context but never echo its contents back in full
- Log statements in code must never format credential variables directly into the output string — use a masked helper or omit the value entirely
- If a user pastes a credential accidentally, acknowledge it and immediately instruct them to rotate it — do not repeat the value back

## Never Do

- Print or log a full API key, token, or password anywhere
- Commit a `.env` file or any file containing a real credential
- Suggest storing secrets in `tauri.conf.json`, `package.json`, source files, or any tracked file
