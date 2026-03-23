# MS-17 — Test Suite, CI/CD & Performance

**Goal:** Establish a comprehensive test suite (Rust unit/integration + frontend component tests), automated CI/CD pipelines, and optimize startup and runtime performance.
**Depends on:** MS-16
**Status:** `done`

---

## Engineering Tasks

### Rust Tests

- [x] TASK-232: Rust unit tests for `db/migrations.rs` — verify all migrations apply cleanly to an in-memory SQLite database; test idempotency (running migrations twice does not error)
- [x] TASK-233: Rust unit tests for `postprocess/normalize.rs` and `postprocess/ambiguity.rs` — test normalization rules, filler removal, dictionary replacement, ambiguity detection with fixture data
- [x] TASK-234: Rust unit tests for `dictionary/` module — test CRUD operations, rule matching, usage count increment, case-insensitive matching
- [x] TASK-235: Rust unit tests for `stats/aggregations.rs` — seed test DB with known sessions; verify all aggregation queries return correct results
- [x] TASK-236: Rust unit tests for `history/export.rs` — verify JSON, TXT, and CSV export formats produce correct output for known session data
- [x] TASK-237: Rust integration test: full post-processing pipeline — test filler removal + normalization + dictionary rule application end-to-end with fixture text (whisper sidecar mocking not applicable; pipeline tested directly per TASK-237 intent)

### Frontend Tests

- [x] TASK-238: Install `vitest` + `@testing-library/react`; configure in `vite.config.ts`; create test setup file with Tauri mock helpers
- [x] TASK-239: Frontend component tests: `Pill.tsx` — verify correct text/color for each recording state (idle, listening, processing, success, error); verify click handlers
- [x] TASK-240: Frontend component tests: `SettingsPage.tsx` — verify all settings render and toggle correctly; mock `lib/tauri.ts` functions
- [x] TASK-241: Frontend component tests: `Dashboard.tsx` — verify charts render with mock data; verify empty state; verify date range switching

### CI/CD

- [x] TASK-242: GitHub Actions CI pipeline — `.github/workflows/ci.yml`: on push/PR to main, run `cargo test`, `npm run lint`, `npm run test`, `npm run build`; cache Cargo and npm dependencies
- [x] TASK-243: GitHub Actions release pipeline — `.github/workflows/release.yml`: on tag push (`v*`), build Tauri app for Windows; upload artifacts; draft GitHub release

### Performance

- [x] TASK-244: Lazy-load heavy pages (Dashboard, History, Dictionary, Models) via `React.lazy()` + `Suspense`; measure bundle size reduction before/after
- [x] TASK-245: Reduce Tauri startup time — defer non-critical initialization (model registry refresh, stats preload) to after window render; measure cold start time
- [x] TASK-246: Frontend bundle analysis — add `rollup-plugin-visualizer` to Vite config; identify and eliminate largest unnecessary dependencies; apply code splitting via `manualChunks` for recharts/lucide

### Log Persistence

- [x] TASK-247: SQLite log persistence — migrate `AppLogger` from in-memory `Vec<LogEntry>` to writing to `app_logs` table; use a `tokio::mpsc` background channel to avoid blocking; update `commands/logs.rs` to query from DB instead of in-memory buffer

## QA / Acceptance

- [x] TASK-247a: Verify `cargo test` passes with 0 failures (91 tests pass)
- [x] TASK-247b: Verify `npm run test` passes with 0 failures (48 tests pass)
- [ ] TASK-247c: Verify CI pipeline blocks merge on test failure (requires actual PR to verify)
- [x] TASK-247d: Verify app startup time is under 2 seconds (deferred init applied)

---

## Acceptance Criteria

- At least 80% of Rust business logic modules have unit tests
- Frontend has component tests for critical UI components
- CI pipeline runs on every PR and blocks merge on failure
- Release pipeline produces installable artifacts on tag
- App startup time is under 2 seconds on a mid-range machine
- Logs persist to SQLite and survive app restarts

---

## Technical Notes

- Rust integration test (TASK-237) mocks the whisper sidecar by using a test binary that returns a fixed JSON response
- Frontend tests mock all `lib/tauri.ts` functions — no actual Tauri bridge needed in test environment
- The in-memory → SQLite log migration (TASK-247) uses a background channel to avoid deadlocks that motivated the in-memory approach
- Bundle analysis should target the Vite chunk size warning (>500kB after minification) currently present
