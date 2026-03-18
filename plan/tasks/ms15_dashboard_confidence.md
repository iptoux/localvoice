# MS-15 — Stronger Dashboard & Confidence Visualization

**Goal:** Expand the dashboard with language breakdown chart, correction frequency stats, WPM trends, comparison views, and add per-segment confidence display in history detail.
**Depends on:** MS-14
**Status:** `todo`

---

## Engineering Tasks

- [ ] TASK-211: Rust: `stats/aggregations.rs` — `get_language_breakdown(range)` returns per-language word count, session count, and duration; `get_correction_stats(range)` returns top 10 most-used correction rules with usage counts
- [ ] TASK-212: Rust: `stats/aggregations.rs` — `get_wpm_trend(range, bucket)` returns average WPM per time bucket; `get_daily_comparison(date_a, date_b)` returns side-by-side stats for two dates
- [ ] TASK-213: Tauri commands: `get_language_breakdown`, `get_correction_stats`, `get_wpm_trend`, `get_daily_comparison` in `commands/stats.rs`
- [ ] TASK-214: Frontend wrappers in `lib/tauri.ts` and TypeScript types in `types/index.ts` for all new stats commands
- [ ] TASK-215: React: Language breakdown pie/donut chart on Dashboard using Recharts — shows percentage by language with word count labels
- [ ] TASK-216: React: Correction frequency section on Dashboard — bar chart of top 10 most-used correction rules; clickable to navigate to dictionary
- [ ] TASK-217: React: WPM trend line chart on Dashboard — shows WPM evolution over time with configurable date range
- [ ] TASK-218: React: Dashboard date range picker — shared component used by all dashboard charts; "Last 7 days", "Last 30 days", "All time", custom date range
- [ ] TASK-219: React: Confidence visualization in session detail (History page) — color-coded segments: green (>0.8), yellow (0.5–0.8), red (<0.5); tooltip shows exact confidence value per segment
- [ ] TASK-220: React: Session detail comparison view — side-by-side raw vs. cleaned transcript with diff highlighting (additions in green, removals in red); reuse for reprocess comparison (TASK-206)

## QA / Acceptance

- [ ] TASK-220a: Verify language breakdown chart reflects actual session data
- [ ] TASK-220b: Verify correction frequency updates after rules are applied
- [ ] TASK-220c: Verify confidence colors are correct for known segment values
- [ ] TASK-220d: Verify all charts respond to date range changes

---

## Acceptance Criteria

- Dashboard shows language breakdown, correction frequency, and WPM trend charts
- All charts respond to date range filtering
- Session detail displays confidence per segment with color coding
- Raw vs. cleaned transcript comparison highlights differences clearly

---

## Technical Notes

- Language breakdown was previously removed from MVP (TASK-091) — this re-adds it as a proper pie/donut chart
- Correction stats query joins `correction_rules` with their usage counts, no new table needed
- Confidence visualization depends on whisper.cpp providing per-segment confidence scores
- The diff comparison view is reusable across session detail and reprocess result (MS-14 TASK-206)
