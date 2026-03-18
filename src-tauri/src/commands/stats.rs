use tauri::State;

use crate::errors::CmdResult;
use crate::state::AppState;
use crate::stats::aggregations::{
    CorrectionStat, DailyStats, DateRange, DashboardStats, LanguageBreakdown, TimeseriesPoint,
    WpmPoint,
};
use crate::stats::service;

/// Returns scalar dashboard metrics (total words, sessions, avg WPM, duration)
/// plus per-language session counts for the given date range.
#[tauri::command]
pub fn get_dashboard_stats(
    state: State<AppState>,
    range: DateRange,
) -> CmdResult<DashboardStats> {
    service::get_dashboard_stats(&state.db, range)
}

/// Returns a daily or weekly time-series of word counts and session counts.
#[tauri::command]
pub fn get_usage_timeseries(
    state: State<AppState>,
    range: DateRange,
    bucket: String,
) -> CmdResult<Vec<TimeseriesPoint>> {
    service::get_usage_timeseries(&state.db, range, bucket)
}

/// Returns per-language breakdown with word count, session count, and duration.
#[tauri::command]
pub fn get_language_breakdown(
    state: State<AppState>,
    range: DateRange,
) -> CmdResult<Vec<LanguageBreakdown>> {
    service::get_language_breakdown(&state.db, range)
}

/// Returns top 10 most-used correction rules with usage counts.
#[tauri::command]
pub fn get_correction_stats(state: State<AppState>) -> CmdResult<Vec<CorrectionStat>> {
    service::get_correction_stats(&state.db)
}

/// Returns average WPM per time bucket (day or week).
#[tauri::command]
pub fn get_wpm_trend(
    state: State<AppState>,
    range: DateRange,
    bucket: String,
) -> CmdResult<Vec<WpmPoint>> {
    service::get_wpm_trend(&state.db, range, bucket)
}

/// Returns side-by-side stats for two dates.
#[tauri::command]
pub fn get_daily_comparison(
    state: State<AppState>,
    date_a: String,
    date_b: String,
) -> CmdResult<(DailyStats, DailyStats)> {
    service::get_daily_comparison(&state.db, date_a, date_b)
}
