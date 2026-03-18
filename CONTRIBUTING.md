# Contributing to LocalVoice

Thank you for your interest in contributing to LocalVoice!

## Development Setup

1. Clone the repository
2. Run the bootstrap script:
   - Windows: `.\scripts\bootstrap.ps1`
   - Unix/macOS: `./scripts/bootstrap.sh`
3. Start development: `pnpm tauri dev`

See [docs/dev/index.md](docs/dev/index.md) for the full developer reference.

## Branch Strategy

This project follows a **feature-branch-per-milestone** pattern:

- `main` — stable, release-ready code
- `ms/*` — feature branches for each milestone (e.g., `ms/10-polish`)

All work happens on feature branches. Changes are merged via pull requests.

## Coding Standards

- **Language:** All code, comments, and commit messages in English
- **TypeScript:** Strict mode enabled; avoid `any`
- **Rust:** Follow `rustfmt` conventions; use meaningful names
- **Style:** ESLint and Clippy enforce style rules — run before committing

## Commit Messages

Format: `type(scope): description`

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

Examples:
- `feat(recording): add microphone selector`
- `fix(transcription): handle empty audio buffer`
- `docs(history): update API reference`

## Pull Request Workflow

1. Create a new branch from `main`: `git switch -c ms/xx-feature-name`
2. Implement your changes
3. Run linting and type checks:
   ```bash
   pnpm run lint      # TypeScript
   cargo clippy       # Rust
   ```
4. Run tests:
   ```bash
   cargo test         # Rust tests
   ```
5. Update documentation if needed
6. Open a PR with a clear description
7. Ensure all CI checks pass
8. Request review

## Testing Requirements

- **Rust:** Unit tests on all command handlers and business logic
- **TypeScript:** Component tests for UI components where practical
- **Smoke tests:** Verify the app launches and basic recording flow works

## Documentation

- User-facing docs: `docs/user/*.md`
- Developer docs: `docs/dev/*.md`
- Architecture Decision Records: `docs/adrs/*.md`

Update docs when adding or changing features.

## Reporting Issues

Use the [issue templates](.github/ISSUE_TEMPLATE.md) for bug reports and feature requests. Include:

- Steps to reproduce (for bugs)
- Expected vs actual behavior
- OS and app version
- Relevant logs (see Settings → Logs)

## Questions?

Open a discussion on GitHub or reach out via the project channels.
