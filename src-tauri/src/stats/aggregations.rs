//! Pre-aggregated SQL queries for the dashboard.
//!
//! All metric formulas are defined here — never duplicated in the frontend.
//!
//! WPM formula: `(word_count / duration_ms) * 60_000`
//! (stored per-session as `estimated_wpm`; averaged here for the dashboard card)

use serde::Serialize;

use crate::db::DbConn;
use crate::errors::{AppError, CmdResult};

// ── Public types (TASK-087) ───────────────────────────────────────────────────

/// Scalar summary metrics for the dashboard header cards.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    pub total_word_count: i64,
    pub total_session_count: i64,
    pub avg_wpm: f64,
    pub total_duration_ms: i64,
    pub language_counts: Vec<LanguageCount>,
    /// Top 3 models by session count in the selected range.
    pub top_models: Vec<ModelUsageStat>,
}

/// Session count for a single language.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageCount {
    pub language: String,
    pub count: i64,
}

/// Usage stats for a single model.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelUsageStat {
    pub model_id: String,
    pub session_count: i64,
    pub total_word_count: i64,
    pub total_duration_ms: i64,
    pub avg_wpm: f64,
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

    // ── Top 3 models ──────────────────────────────────────────────────────────
    let model_sql = format!(
        "SELECT
            COALESCE(model_id, 'unknown') as mid,
            COUNT(*) as cnt,
            COALESCE(SUM(word_count), 0),
            COALESCE(SUM(duration_ms), 0),
            COALESCE(AVG(CASE WHEN estimated_wpm IS NOT NULL THEN estimated_wpm END), 0.0)
         FROM sessions
         {where_clause}
         GROUP BY mid
         ORDER BY cnt DESC
         LIMIT 3"
    );
    let params_refs3: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| b.as_ref()).collect();
    let mut stmt3 = conn.prepare(&model_sql).map_err(AppError::from)?;
    let top_models = stmt3
        .query_map(params_refs3.as_slice(), |row| {
            Ok(ModelUsageStat {
                model_id: row.get(0)?,
                session_count: row.get(1)?,
                total_word_count: row.get(2)?,
                total_duration_ms: row.get(3)?,
                avg_wpm: row.get(4)?,
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
        top_models,
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

// ── Language breakdown (TASK-211) ─────────────────────────────────────────

/// Per-language breakdown with word count, session count, and total duration.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageBreakdown {
    pub language: String,
    pub session_count: i64,
    pub word_count: i64,
    pub duration_ms: i64,
}

/// Returns per-language word count, session count, and duration for the given range.
pub fn get_language_breakdown(db: &DbConn, range: &DateRange) -> CmdResult<Vec<LanguageBreakdown>> {
    let conn = db.lock().unwrap();
    let (where_clause, params) = build_where(range);
    let sql = format!(
        "SELECT language, COUNT(*), COALESCE(SUM(word_count), 0), COALESCE(SUM(duration_ms), 0)
         FROM sessions
         {where_clause}
         GROUP BY language
         ORDER BY SUM(word_count) DESC"
    );
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| b.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(AppError::from)?;
    let rows = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(LanguageBreakdown {
                language: row.get(0)?,
                session_count: row.get(1)?,
                word_count: row.get(2)?,
                duration_ms: row.get(3)?,
            })
        })
        .map_err(AppError::from)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(AppError::from)?;
    Ok(rows)
}

// ── Correction stats (TASK-211) ──────────────────────────────────────────

/// Usage stats for a single correction rule.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionStat {
    pub source_phrase: String,
    pub target_phrase: String,
    pub usage_count: i64,
    pub last_used_at: Option<String>,
}

/// Returns top 10 most-used correction rules.
pub fn get_correction_stats(db: &DbConn) -> CmdResult<Vec<CorrectionStat>> {
    let conn = db.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT source_phrase, target_phrase, usage_count, last_used_at
             FROM correction_rules
             WHERE is_active = 1
             ORDER BY usage_count DESC
             LIMIT 10",
        )
        .map_err(AppError::from)?;
    let rows = stmt
        .query_map([], |row| {
            Ok(CorrectionStat {
                source_phrase: row.get(0)?,
                target_phrase: row.get(1)?,
                usage_count: row.get(2)?,
                last_used_at: row.get(3)?,
            })
        })
        .map_err(AppError::from)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(AppError::from)?;
    Ok(rows)
}

// ── WPM trend (TASK-212) ─────────────────────────────────────────────────

/// One data point in a WPM time-series.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WpmPoint {
    pub date: String,
    pub avg_wpm: f64,
    pub session_count: i64,
}

/// Returns average WPM per time bucket (day or week).
pub fn get_wpm_trend(db: &DbConn, range: &DateRange, bucket: &str) -> CmdResult<Vec<WpmPoint>> {
    let conn = db.lock().unwrap();
    let (where_clause, params) = build_where(range);
    let date_expr = match bucket {
        "week" => "date(started_at, 'weekday 0', '-6 days')",
        _ => "date(started_at)",
    };
    let sql = format!(
        "SELECT
            {date_expr} AS bucket,
            COALESCE(AVG(CASE WHEN estimated_wpm IS NOT NULL THEN estimated_wpm END), 0.0),
            COUNT(*)
         FROM sessions
         {where_clause}
         GROUP BY bucket
         ORDER BY bucket ASC"
    );
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| b.as_ref()).collect();
    let mut stmt = conn.prepare(&sql).map_err(AppError::from)?;
    let rows = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(WpmPoint {
                date: row.get(0)?,
                avg_wpm: row.get(1)?,
                session_count: row.get(2)?,
            })
        })
        .map_err(AppError::from)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(AppError::from)?;
    Ok(rows)
}

// ── Daily comparison (TASK-212) ──────────────────────────────────────────

/// Side-by-side stats for a single date.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStats {
    pub date: String,
    pub session_count: i64,
    pub word_count: i64,
    pub duration_ms: i64,
    pub avg_wpm: f64,
}

/// Returns side-by-side stats for two dates.
pub fn get_daily_comparison(
    db: &DbConn,
    date_a: &str,
    date_b: &str,
) -> CmdResult<(DailyStats, DailyStats)> {
    fn stats_for_date(conn: &rusqlite::Connection, date: &str) -> rusqlite::Result<DailyStats> {
        conn.query_row(
            "SELECT
                COALESCE(COUNT(*), 0),
                COALESCE(SUM(word_count), 0),
                COALESCE(SUM(duration_ms), 0),
                COALESCE(AVG(CASE WHEN estimated_wpm IS NOT NULL THEN estimated_wpm END), 0.0)
             FROM sessions
             WHERE date(started_at) = ?1",
            rusqlite::params![date],
            |row| {
                Ok(DailyStats {
                    date: date.to_string(),
                    session_count: row.get(0)?,
                    word_count: row.get(1)?,
                    duration_ms: row.get(2)?,
                    avg_wpm: row.get(3)?,
                })
            },
        )
    }

    let conn = db.lock().unwrap();
    let a = stats_for_date(&conn, date_a).map_err(AppError::from)?;
    let b = stats_for_date(&conn, date_b).map_err(AppError::from)?;
    Ok((a, b))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;
    use rusqlite::{params, Connection};
    use std::sync::{Arc, Mutex};

    fn test_db() -> crate::db::DbConn {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        migrations::run(&conn).unwrap();
        Arc::new(Mutex::new(conn))
    }

    fn insert_session(
        db: &crate::db::DbConn,
        id: &str,
        started_at: &str,
        language: &str,
        word_count: i64,
        duration_ms: i64,
        estimated_wpm: Option<f64>,
    ) {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO sessions
                (id, started_at, ended_at, duration_ms, language, trigger_type,
                 raw_text, cleaned_text, word_count, char_count, output_mode,
                 inserted_successfully, created_at, reprocessed_count, estimated_wpm)
             VALUES (?1, ?2, ?2, ?3, ?4, 'hotkey', '', '', ?5, 0, 'clipboard', 0, ?2, 0, ?6)",
            params![
                id,
                started_at,
                duration_ms,
                language,
                word_count,
                estimated_wpm
            ],
        )
        .unwrap();
    }

    // ── get_dashboard_stats ───────────────────────────────────────────────────

    #[test]
    fn dashboard_stats_empty_db_returns_zeros() {
        let db = test_db();
        let range = DateRange::default();
        let stats = get_dashboard_stats(&db, &range).unwrap();
        assert_eq!(stats.total_session_count, 0);
        assert_eq!(stats.total_word_count, 0);
        assert_eq!(stats.total_duration_ms, 0);
    }

    #[test]
    fn dashboard_stats_aggregates_sessions() {
        let db = test_db();
        insert_session(
            &db,
            "1",
            "2026-01-01T10:00:00Z",
            "de",
            100,
            60_000,
            Some(100.0),
        );
        insert_session(
            &db,
            "2",
            "2026-01-02T10:00:00Z",
            "en",
            200,
            120_000,
            Some(100.0),
        );
        let range = DateRange::default();
        let stats = get_dashboard_stats(&db, &range).unwrap();
        assert_eq!(stats.total_session_count, 2);
        assert_eq!(stats.total_word_count, 300);
        assert_eq!(stats.total_duration_ms, 180_000);
        assert!((stats.avg_wpm - 100.0).abs() < 0.01);
    }

    #[test]
    fn dashboard_stats_filters_by_date_range() {
        let db = test_db();
        insert_session(&db, "1", "2026-01-01T10:00:00Z", "de", 50, 30_000, None);
        insert_session(&db, "2", "2026-02-01T10:00:00Z", "de", 100, 60_000, None);
        let range = DateRange {
            start: Some("2026-02-01T00:00:00Z".to_string()),
            end: None,
        };
        let stats = get_dashboard_stats(&db, &range).unwrap();
        assert_eq!(stats.total_session_count, 1);
        assert_eq!(stats.total_word_count, 100);
    }

    #[test]
    fn dashboard_stats_includes_language_counts() {
        let db = test_db();
        insert_session(&db, "1", "2026-01-01T10:00:00Z", "de", 100, 60_000, None);
        insert_session(&db, "2", "2026-01-02T10:00:00Z", "de", 50, 30_000, None);
        insert_session(&db, "3", "2026-01-03T10:00:00Z", "en", 200, 60_000, None);
        let range = DateRange::default();
        let stats = get_dashboard_stats(&db, &range).unwrap();
        assert_eq!(stats.language_counts.len(), 2);
        // de has 2 sessions, en has 1 — ordered by count desc
        assert_eq!(stats.language_counts[0].language, "de");
        assert_eq!(stats.language_counts[0].count, 2);
    }

    // ── get_usage_timeseries ─────────────────────────────────────────────────

    #[test]
    fn timeseries_groups_by_day() {
        let db = test_db();
        insert_session(&db, "1", "2026-01-01T10:00:00Z", "de", 100, 60_000, None);
        insert_session(&db, "2", "2026-01-01T14:00:00Z", "de", 50, 30_000, None);
        insert_session(&db, "3", "2026-01-02T10:00:00Z", "en", 200, 60_000, None);
        let range = DateRange::default();
        let points = get_usage_timeseries(&db, &range, "day").unwrap();
        assert_eq!(points.len(), 2);
        // First bucket: 2026-01-01, two sessions, 150 words
        assert_eq!(points[0].session_count, 2);
        assert_eq!(points[0].word_count, 150);
        // Second bucket: 2026-01-02, one session, 200 words
        assert_eq!(points[1].session_count, 1);
        assert_eq!(points[1].word_count, 200);
    }

    #[test]
    fn timeseries_empty_when_no_sessions() {
        let db = test_db();
        let range = DateRange::default();
        let points = get_usage_timeseries(&db, &range, "day").unwrap();
        assert!(points.is_empty());
    }

    // ── get_language_breakdown ───────────────────────────────────────────────

    #[test]
    fn language_breakdown_ordered_by_word_count() {
        let db = test_db();
        insert_session(&db, "1", "2026-01-01T10:00:00Z", "de", 100, 60_000, None);
        insert_session(&db, "2", "2026-01-02T10:00:00Z", "de", 50, 30_000, None);
        insert_session(&db, "3", "2026-01-03T10:00:00Z", "en", 200, 60_000, None);
        let range = DateRange::default();
        let breakdown = get_language_breakdown(&db, &range).unwrap();
        assert_eq!(breakdown.len(), 2);
        // en has more words (200) than de (150)
        assert_eq!(breakdown[0].language, "en");
        assert_eq!(breakdown[0].word_count, 200);
        assert_eq!(breakdown[1].language, "de");
        assert_eq!(breakdown[1].word_count, 150);
    }

    // ── get_wpm_trend ─────────────────────────────────────────────────────────

    #[test]
    fn wpm_trend_averages_per_bucket() {
        let db = test_db();
        insert_session(
            &db,
            "1",
            "2026-01-01T10:00:00Z",
            "de",
            100,
            60_000,
            Some(100.0),
        );
        insert_session(
            &db,
            "2",
            "2026-01-01T14:00:00Z",
            "de",
            100,
            60_000,
            Some(200.0),
        );
        let range = DateRange::default();
        let trend = get_wpm_trend(&db, &range, "day").unwrap();
        assert_eq!(trend.len(), 1);
        assert!((trend[0].avg_wpm - 150.0).abs() < 0.01);
        assert_eq!(trend[0].session_count, 2);
    }

    // ── get_daily_comparison ─────────────────────────────────────────────────

    #[test]
    fn daily_comparison_returns_per_date_stats() {
        let db = test_db();
        insert_session(
            &db,
            "1",
            "2026-01-01T10:00:00Z",
            "de",
            100,
            60_000,
            Some(100.0),
        );
        insert_session(
            &db,
            "2",
            "2026-01-02T10:00:00Z",
            "en",
            200,
            120_000,
            Some(150.0),
        );
        let (day_a, day_b) = get_daily_comparison(&db, "2026-01-01", "2026-01-02").unwrap();
        assert_eq!(day_a.word_count, 100);
        assert_eq!(day_a.session_count, 1);
        assert_eq!(day_b.word_count, 200);
        assert_eq!(day_b.session_count, 1);
    }

    #[test]
    fn daily_comparison_empty_date_returns_zeros() {
        let db = test_db();
        let (day_a, day_b) = get_daily_comparison(&db, "2099-01-01", "2099-01-02").unwrap();
        assert_eq!(day_a.word_count, 0);
        assert_eq!(day_b.word_count, 0);
    }

    // ── get_correction_stats ─────────────────────────────────────────────────

    #[test]
    fn correction_stats_empty_when_no_rules() {
        let db = test_db();
        let stats = get_correction_stats(&db).unwrap();
        assert!(stats.is_empty());
    }

    #[test]
    fn correction_stats_returns_active_rules() {
        use crate::dictionary::service;
        let db = test_db();
        service::create_rule(&db, "k8s", "Kubernetes", None, true).unwrap();
        let stats = get_correction_stats(&db).unwrap();
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].source_phrase, "k8s");
        assert_eq!(stats[0].target_phrase, "Kubernetes");
    }
}

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
