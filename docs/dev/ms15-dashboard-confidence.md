# MS-15 — Stronger Dashboard & Confidence Visualization

## What Was Built
- Language breakdown donut chart on Dashboard (Recharts PieChart)
- Top correction rules bar chart / bar list on Dashboard
- WPM trend line chart on Dashboard
- Daily comparison API endpoint (side-by-side stats for two dates)
- Confidence-colored segments in session detail (green/yellow/red dots)
- Word-level diff view (Raw → Cleaned) in session detail with LCS algorithm

## Key Decisions
- All new chart data fetched in parallel via dashboard store (5 parallel API calls)
- Donut chart uses `innerRadius`/`outerRadius` for clean look, with legend below
- WPM trend uses green (#34d399) to visually distinguish from word count (blue #60a5fa)
- Correction stats uses bar list for ≤5 items, horizontal BarChart for >5
- Diff uses LCS-based word diff — simple and sufficient for transcription text
- Tooltip styling uses CSS variables for theme compatibility

## Architecture Notes
- `stats/aggregations.rs` — 4 new query functions: `get_language_breakdown`, `get_correction_stats`, `get_wpm_trend`, `get_daily_comparison`
- `stats/service.rs` — thin pass-through for all new functions
- `commands/stats.rs` — 4 new Tauri commands registered in `lib.rs`
- `dashboard-store.ts` — extended with `languageBreakdown`, `correctionStats`, `wpmTrend` slices
- `Dashboard.tsx` — fully rewritten with 2x3 grid layout: stat cards, words+language row, wpm+corrections row, top models
- `History.tsx` — confidence dots (green ≥0.8, yellow ≥0.5, red <0.5), WordDiff component with LCS

## Known Limitations / Future Work
- Daily comparison command exists but no UI yet (could add date picker comparison widget)
- Custom date range in UI not implemented (only preset buttons: 7d, 30d, all)
- Correction stats chart links to dictionary page not implemented (just displays data)
