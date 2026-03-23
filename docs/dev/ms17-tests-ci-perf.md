# MS-17 — Test Suite, CI/CD & Performance

## What Was Built

- **91 Rust unit/integration tests** across all major backend modules
- **48 frontend component tests** using Vitest + @testing-library/react
- **GitHub Actions CI pipeline** running on every push/PR to main
- **Performance optimizations**: lazy page loading, deferred startup tasks, Vite code splitting with `manualChunks`
- **SQLite log persistence**: replaced in-memory log buffer with a `tokio::mpsc` channel writing to `app_logs` table

## Key Decisions

- **In-memory SQLite for Rust tests**: Every test module that needs DB access creates a `Connection::open_in_memory()` with full migrations applied via `migrations::run()`. No test isolation setup needed — each test gets a fresh DB.
- **TASK-237 scope**: The "full pipeline" integration test tests the post-processing pipeline (normalize → filler removal → dictionary rules) directly, not the whisper sidecar. The sidecar is a separate binary with no practical unit test interface.
- **tokio background channel for logging**: The `AppLogger` now sends `LogEntry` values over an `UnboundedSender<LogEntry>`; a background task spawned via `tauri::async_runtime::spawn` receives entries and writes them with `spawn_blocking` (rusqlite is sync). This avoids holding a mutex lock across await points.
- **Frontend store seeding**: Tests use `useStore.setState()` directly to control state rather than mocking the entire store module. This exercises real component rendering with controlled data.
- **PointerEvent polyfill**: `@base-ui/react` Switch component references `PointerEvent` internally. jsdom does not provide it. Added a minimal polyfill in `src/test/setup.ts`.
- **SelectContent portal caveat**: shadcn/ui `SelectContent` renders in a DOM portal and is only mounted when the dropdown is open. Tests that check for select option text must open the dropdown first or test the trigger instead.

## Architecture Notes

### Rust Test Modules

All tests live in `#[cfg(test)] mod tests { ... }` blocks inside the implementation files:

| File | Test count | What's covered |
|------|-----------|----------------|
| `db/migrations.rs` | 6 | migration idempotency, schema version, default seeds |
| `postprocess/normalize.rs` | 16 | whitespace collapse, capitalize, punctuation, full pipeline |
| `postprocess/ambiguity.rs` | 10 | confidence threshold, dedup, short segment filtering |
| `dictionary/service.rs` | 8 | rule CRUD, apply_rules, case-insensitive matching |
| `stats/aggregations.rs` | 11 | dashboard stats, timeseries, wpm trend, daily comparison |
| `history/export.rs` | 8 | text, JSON, CSV export; quote escaping; empty inputs |
| `transcription/pipeline.rs` | 8 | end-to-end post-processing with fixture text |
| other modules | ~24 | dictionary rules, filler removal, etc. |

### Frontend Test Files

| File | Tests | Approach |
|------|-------|----------|
| `Pill.test.tsx` | 18 | state-driven rendering via `useAppStore.setState` |
| `SettingsPage.test.tsx` | 14 | store seeding, switch toggle, shortcut badge, select |
| `Dashboard.test.tsx` | 16 | store seeding, range selector, stat cards, section headers |

### CI Pipeline (`.github/workflows/ci.yml`)

Two parallel jobs on `windows-latest`:
- `rust-test`: `cargo test --all-features` with Cargo cache
- `frontend`: `pnpm lint` + `pnpm test` + `pnpm build` with pnpm store cache

### Vite Code Splitting

`manualChunks` in `vite.config.ts` splits vendor code into named chunks:
- `vendor-react`: react, react-dom, react-router-dom
- `vendor-charts`: recharts
- `vendor-icons`: lucide-react
- `vendor-i18n`: i18next + plugins
- `vendor-ui`: @radix-ui, @base-ui, cmdk, etc.

## Known Limitations / Future Work

- TASK-247c (CI blocks merge on failure) requires an actual PR to verify manually
- Startup time measurement is qualitative; no automated benchmark exists yet
- `act(...)` warnings from React 19 async state updates in tests are benign (tests pass) but could be silenced by wrapping renders in `act`
- The `SelectContent` portal limitation means audio device option tests only verify the trigger renders, not the options themselves
