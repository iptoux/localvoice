use crate::db::DbConn;
use crate::errors::CmdResult;
use crate::stats::aggregations::{
    self, CorrectionStat, DailyStats, DateRange, DashboardStats, LanguageBreakdown,
    TimeseriesPoint, WpmPoint,
};

/// Returns all scalar dashboard metrics for the given date range.
pub fn get_dashboard_stats(db: &DbConn, range: DateRange) -> CmdResult<DashboardStats> {
    aggregations::get_dashboard_stats(db, &range)
}

/// Returns a time-series of word counts and session counts bucketed by day or week.
pub fn get_usage_timeseries(
    db: &DbConn,
    range: DateRange,
    bucket: String,
) -> CmdResult<Vec<TimeseriesPoint>> {
    aggregations::get_usage_timeseries(db, &range, &bucket)
}

/// Returns per-language breakdown (word count, sessions, duration).
pub fn get_language_breakdown(db: &DbConn, range: DateRange) -> CmdResult<Vec<LanguageBreakdown>> {
    aggregations::get_language_breakdown(db, &range)
}

/// Returns top 10 most-used correction rules.
pub fn get_correction_stats(db: &DbConn) -> CmdResult<Vec<CorrectionStat>> {
    aggregations::get_correction_stats(db)
}

/// Returns average WPM per time bucket.
pub fn get_wpm_trend(db: &DbConn, range: DateRange, bucket: String) -> CmdResult<Vec<WpmPoint>> {
    aggregations::get_wpm_trend(db, &range, &bucket)
}

/// Returns side-by-side stats for two dates.
pub fn get_daily_comparison(
    db: &DbConn,
    date_a: String,
    date_b: String,
) -> CmdResult<(DailyStats, DailyStats)> {
    aggregations::get_daily_comparison(db, &date_a, &date_b)
}
