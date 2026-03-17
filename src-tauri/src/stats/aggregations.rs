//! Pre-aggregated SQL queries for the dashboard.
//!
//! All metric formulas are defined here — never duplicated in the frontend.
//!
//! WPM formula: `(word_count / duration_ms) * 60_000`
//! (stored per-session as `estimated_wpm`; averaged here for the dashboard card)

use rusqlite::params;
use serde::Serialize;

use crate::db::DbConn;
use crate::errors::{AppError, CmdResult};

// ── Public types (TASK-087) ───────────────────────────────────────────────────

/// Scalar summary metrics for the dashboard header cards.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    /// Sum of `word_count` across all sessions in range.
    pub total_word_count: i64,
    /// Number of sessions in range.
    pub total_session_count: i64,
    /// Average of non-null `estimated_wpm` across sessions in range.
    pub avg_wpm: f64,
    /// Sum of `duration_ms` across sessions in range (milliseconds).
    pub total_duration_ms: i64,
    /// Session count per language code, sorted by count descending.
    pub language_counts: Vec<LanguageCount>,
}

/// Session count for a single language.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageCount {
    pub language: String,
    pub count: i64,
}

/// One data point in a usage time-series.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesPoint {
    /// Bucket date label (ISO 8601 date string, e.g. "2026-03-17").
    pub date: String,
    pub word_count: i64,
    pub session_count: i64,
}

/// Date range filter for all stats queries. Both bounds are inclusive ISO 8601 strings.
#[derive(Debug, Clone, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct DateRange {
    pub start: Option<String>,
    pub end: Option<String>,
}

// ── Queries ───────────────────────────────────────────────────────────────────

/// Returns scalar aggregate metrics for the given date range.
pub fn get_dashboard_stats(db: &DbConn, range: &DateRange) -> CmdResult<DashboardStats> {
    let conn = db.lock().unwrap();
    let (where_clause, params) = build_where(range);

    // ── Scalar aggregates ─────────────────────────────────────────────────────
    let sql = format!(
        "SELECT
            COALESCE(SUM(word_count), 0),
            COUNT(*),
            COALESCE(AVG(CASE WHEN estimated_wpm IS NOT NULL THEN estimated_wpm END), 0.0),
            COALESCE(SUM(duration_ms), 0)
         FROM sessions
         {where_clause}"
    );
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| b.as_ref()).collect();
    let (total_word_count, total_session_count, avg_wpm, total_duration_ms) = conn
        .query_row(&sql, params_refs.as_slice(), |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .map_err(AppError::from)?;

    // ── Language counts ───────────────────────────────────────────────────────
    let lang_sql = format!(
        "SELECT language, COUNT(*) as cnt
         FROM sessions
         {where_clause}
         GROUP BY language
         ORDER BY cnt DESC"
    );
    let params_refs2: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| b.as_ref()).collect();
    let mut stmt = conn.prepare(&lang_sql).map_err(AppError::from)?;
    let language_counts = stmt
        .query_map(params_refs2.as_slice(), |row| {
            Ok(LanguageCount {
                language: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(AppError::from)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(AppError::from)?;

    Ok(DashboardStats {
        total_word_count,
        total_session_count,
        avg_wpm,
        total_duration_ms,
        language_counts,
    })
}

/// Returns a time-series of word counts and session counts bucketed by day or week.
///
/// `bucket` should be `"day"` or `"week"`. Defaults to `"day"`.
pub fn get_usage_timeseries(
    db: &DbConn,
    range: &DateRange,
    bucket: &str,
) -> CmdResult<Vec<TimeseriesPoint>> {
    let conn = db.lock().unwrap();
    let (where_clause, params) = build_where(range);

    // SQLite date truncation: for 'week' we round down to Monday using
    // `date(started_at, 'weekday 0', '-6 days')`.
    let date_expr = match bucket {
        "week" => "date(started_at, 'weekday 0', '-6 days')",
        _ => "date(started_at)",
    };

    let sql = format!(
        "SELECT
            {date_expr} AS bucket,
            COALESCE(SUM(word_count), 0),
            COUNT(*)
         FROM sessions
         {where_clause}
         GROUP BY bucket
         ORDER BY bucket ASC"
    );

    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| b.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(AppError::from)?;
    let points = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(TimeseriesPoint {
                date: row.get(0)?,
                word_count: row.get(1)?,
                session_count: row.get(2)?,
            })
        })
        .map_err(AppError::from)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(AppError::from)?;

    Ok(points)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Builds a parameterized `WHERE` clause from a `DateRange`.
fn build_where(range: &DateRange) -> (String, Vec<Box<dyn rusqlite::ToSql>>) {
    let mut conditions: Vec<&str> = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(start) = &range.start {
        if !start.is_empty() {
            conditions.push("started_at >= ?");
            param_values.push(Box::new(start.clone()));
        }
    }
    if let Some(end) = &range.end {
        if !end.is_empty() {
            conditions.push("started_at <= ?");
            param_values.push(Box::new(end.clone()));
        }
    }

    let clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    (clause, param_values)
}
