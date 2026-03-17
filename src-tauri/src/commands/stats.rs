use tauri::State;

use crate::errors::CmdResult;
use crate::state::AppState;
use crate::stats::aggregations::{DashboardStats, DateRange, TimeseriesPoint};
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
///
/// `bucket` must be `"day"` or `"week"`.
#[tauri::command]
pub fn get_usage_timeseries(
    state: State<AppState>,
    range: DateRange,
    bucket: String,
) -> CmdResult<Vec<TimeseriesPoint>> {
    service::get_usage_timeseries(&state.db, range, bucket)
}
