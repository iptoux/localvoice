# MS-06 — Dashboard

## What Was Built

The Dashboard page surfaces live usage metrics from the SQLite sessions table: total words, total sessions, average WPM, total recording time, language breakdown bar chart, and a words-over-time line chart. All data updates when the user switches the date range (Last 7 days / Last 30 days / All time).

## Key Decisions

- **Pre-aggregated SQL** (`stats/aggregations.rs`): all metric formulas live in Rust, never duplicated in the frontend. The WPM formula `(word_count / duration_ms) * 60_000` is documented once in `aggregations.rs`.
- **`stats/service.rs` as a thin wrapper**: keeps the command layer (`commands/stats.rs`) free of query logic and makes aggregations independently testable.
- **Recharts 3** for charts: installed via pnpm. `@types/recharts` is not needed — Recharts 3 ships its own TypeScript definitions. Tooltip `formatter` callbacks use `v as number` cast because Recharts types `ValueType` as `string | number | undefined`.
- **Zustand dashboard store** (`stores/dashboard-store.ts`): fetches both scalar stats and timeseries in parallel via `Promise.all`. Converts the `RangePreset` enum to a `DateRange` object + bucket granularity (`day` for 7d/30d, `week` for all-time).
- **SQLite week bucketing**: `date(started_at, 'weekday 0', '-6 days')` rounds each date down to its Monday — the standard ISO week start.
- **Empty state**: charts render a placeholder message when no data is available; stat cards show `—` for WPM when `avg_wpm == 0`.

## Architecture Notes

### New Rust modules

| Module | Purpose |
|--------|---------|
| `src-tauri/src/stats/aggregations.rs` | `get_dashboard_stats`, `get_usage_timeseries`; also defines `DashboardStats`, `LanguageCount`, `TimeseriesPoint`, `DateRange` |
| `src-tauri/src/stats/service.rs` | Thin service wrapper over aggregations |
| `src-tauri/src/commands/stats.rs` | `get_dashboard_stats`, `get_usage_timeseries` Tauri commands |

### Frontend

- `src/types/index.ts`: `LanguageCount`, `DashboardStats`, `TimeseriesPoint`, `DateRange` interfaces.
- `src/stores/dashboard-store.ts`: `useDashboardStore` Zustand slice with `stats`, `timeseries`, `range`, `loading`, `error`, `setRange`, `fetch`.
- `src/pages/Dashboard.tsx`: stat cards + `LineChart` (words over time) + `BarChart` (language breakdown) + `RangeSelector`.

## Known Limitations / Future Work

- `total_duration_ms` displayed as `h:mm` — includes transcription time, not just audio length (same limitation as MS-05 session duration).
- No per-model breakdown chart (deferred to MS-10 polish or MS-07 models page).
- Bundle size warning (~635 kB minified) is from Recharts; code-splitting is deferred to MS-10.
