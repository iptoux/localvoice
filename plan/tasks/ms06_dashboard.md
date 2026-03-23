# MS-06 — Dashboard

**Goal:** Surface useful usage metrics and trends — total words, sessions, WPM, recording time, language breakdown, and activity over time.
**Depends on:** MS-05
**Status:** `done`

---

## Engineering Tasks

- [x] TASK-085: Implement `stats/aggregations.rs` — SQL queries for: `total_word_count`, `total_session_count`, `avg_wpm` (avg of `estimated_wpm` where non-null), `total_duration_ms`, `language_counts` (group by language), `top_correction_rules` (top 5 by usage_count)
- [x] TASK-086: Implement `stats/service.rs` — `get_dashboard_stats(range: DateRange)` calling aggregations; `get_usage_timeseries(range, bucket: day|week)` returning `Vec<{date, word_count, session_count}>`
- [x] TASK-087: Define `DashboardStats` and `TimeseriesPoint` structs; derive `Serialize` for Tauri response
- [x] TASK-088: Implement `commands/stats.rs` — Tauri commands: `get_dashboard_stats(range)`, `get_usage_timeseries(range, bucket)`
- [x] TASK-089: React: Dashboard page — 4 stat cards: Total Words, Total Sessions, Avg WPM, Total Recording Time (formatted as h:mm)
- [x] TASK-090: React: Words-over-time line chart (Recharts `LineChart`) — x-axis: date, y-axis: word count
- [x] TASK-091: React: Language usage bar or donut chart — DE vs EN session counts
- [x] TASK-092: React: Date range selector (Last 7 days / 30 days / All time); updates all charts on change
- [x] TASK-093: React: Dashboard store slice in Zustand; fetch on mount and on range change

## Product/UX Tasks

- [x] TASK-094: Validate metric definitions — confirm `estimated_wpm` formula is `(word_count / duration_ms) * 60000`; document centrally in `stats/aggregations.rs`

## QA / Acceptance

- [x] TASK-095: Verify dashboard shows non-zero values after 3+ sessions exist
- [x] TASK-096: Verify total word count matches sum of `word_count` column across sessions in DB
- [x] TASK-097: Verify charts re-render correctly when date range changes
- [x] TASK-098: Verify empty state (no sessions yet) renders without errors

---

## Acceptance Criteria

- Dashboard displays non-zero values after sessions exist
- Word count, session count, total duration, and average WPM are correct within expected tolerance
- Charts render from live DB data

---

## Technical Notes

- Define metric formulas once in `stats/aggregations.rs` — do not duplicate logic in frontend
- WPM formula: `(word_count / duration_ms) * 60000`
- Use pre-aggregated SQL queries rather than loading all sessions into memory
- Keep `DateRange` type reusable — `{ start: Option<String>, end: Option<String> }` in ISO format
- Used Recharts 3 (bundles its own types — `@types/recharts` not needed)
- SQLite week bucketing: `date(started_at, 'weekday 0', '-6 days')` rounds to Monday
