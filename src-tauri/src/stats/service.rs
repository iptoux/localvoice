use crate::db::DbConn;
use crate::errors::CmdResult;
use crate::stats::aggregations::{
    get_dashboard_stats as agg_stats, get_usage_timeseries as agg_timeseries, DateRange,
    DashboardStats, TimeseriesPoint,
};

/// Returns all scalar dashboard metrics for the given date range.
pub fn get_dashboard_stats(db: &DbConn, range: DateRange) -> CmdResult<DashboardStats> {
    agg_stats(db, &range)
}

/// Returns a time-series of word counts and session counts bucketed by day or week.
pub fn get_usage_timeseries(
    db: &DbConn,
    range: DateRange,
    bucket: String,
) -> CmdResult<Vec<TimeseriesPoint>> {
    agg_timeseries(db, &range, &bucket)
}
