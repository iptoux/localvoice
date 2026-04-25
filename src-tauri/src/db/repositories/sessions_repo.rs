use rusqlite::params;
use uuid::Uuid;

use crate::db::models::{Session, SessionFilter, SessionSegment, SessionWithSegments};
use crate::db::DbConn;
use crate::errors::{AppError, CmdResult};
use crate::transcription::types::TranscriptSegment;

const DEFAULT_PAGE_SIZE: i64 = 50;

// ── Write ─────────────────────────────────────────────────────────────────────

/// Inserts a new session row. Returns an error if the id already exists.
pub fn insert_session(db: &DbConn, session: &Session) -> CmdResult<()> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO sessions (
            id, started_at, ended_at, duration_ms, language, model_id,
            trigger_type, input_device_id, raw_text, cleaned_text,
            word_count, char_count, avg_confidence, estimated_wpm,
            output_mode, output_target_app, inserted_successfully,
            error_message, created_at, audio_path, original_raw_text,
            reprocessed_count, original_model_id, original_language, original_avg_confidence
         ) VALUES (
            ?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25
         )",
        params![
            session.id,
            session.started_at,
            session.ended_at,
            session.duration_ms,
            session.language,
            session.model_id,
            session.trigger_type,
            session.input_device_id,
            session.raw_text,
            session.cleaned_text,
            session.word_count,
            session.char_count,
            session.avg_confidence,
            session.estimated_wpm,
            session.output_mode,
            session.output_target_app,
            session.inserted_successfully as i64,
            session.error_message,
            session.created_at,
            session.audio_path,
            session.original_raw_text,
            session.reprocessed_count,
            session.original_model_id,
            session.original_language,
            session.original_avg_confidence,
        ],
    )
    .map_err(AppError::from)?;
    Ok(())
}

/// Inserts all segments for `session_id`. Existing segments are not re-inserted.
pub fn insert_segments(
    db: &DbConn,
    session_id: &str,
    segments: &[TranscriptSegment],
) -> CmdResult<()> {
    let conn = db.lock().unwrap();
    for (idx, seg) in segments.iter().enumerate() {
        conn.execute(
            "INSERT INTO session_segments
                (id, session_id, start_ms, end_ms, text, confidence, segment_index)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                Uuid::new_v4().to_string(),
                session_id,
                seg.start_ms,
                seg.end_ms,
                seg.text,
                seg.confidence.map(|c| c as f64),
                idx as i64,
            ],
        )
        .map_err(AppError::from)?;
    }
    Ok(())
}

// ── Read ──────────────────────────────────────────────────────────────────────

/// Returns a paginated, filtered list of sessions, newest first.
pub fn list_sessions(db: &DbConn, filter: &SessionFilter) -> CmdResult<Vec<Session>> {
    let limit = filter.limit.unwrap_or(DEFAULT_PAGE_SIZE);
    let offset = filter.offset.unwrap_or(0);

    // Build WHERE clause with parameterized values.
    let mut conditions: Vec<&str> = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(lang) = &filter.language {
        if !lang.is_empty() {
            conditions.push("language = ?");
            param_values.push(Box::new(lang.clone()));
        }
    }
    if let Some(from) = &filter.date_from {
        if !from.is_empty() {
            conditions.push("started_at >= ?");
            param_values.push(Box::new(from.clone()));
        }
    }
    if let Some(to) = &filter.date_to {
        if !to.is_empty() {
            conditions.push("started_at <= ?");
            param_values.push(Box::new(to.clone()));
        }
    }
    if let Some(model) = &filter.model_id {
        if !model.is_empty() {
            conditions.push("model_id = ?");
            param_values.push(Box::new(model.clone()));
        }
    }
    if let Some(q) = &filter.query {
        if !q.is_empty() {
            // Escape LIKE special characters, then wrap in %.
            let escaped = q
                .replace('\\', "\\\\")
                .replace('%', "\\%")
                .replace('_', "\\_");
            let pattern = format!("%{escaped}%");
            conditions.push("(cleaned_text LIKE ? ESCAPE '\\' OR raw_text LIKE ? ESCAPE '\\')");
            param_values.push(Box::new(pattern.clone()));
            param_values.push(Box::new(pattern));
        }
    }
    if let Some(true) = filter.has_audio {
        conditions.push("audio_path IS NOT NULL");
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT id, started_at, ended_at, duration_ms, language, model_id, trigger_type,
                input_device_id, raw_text, cleaned_text, word_count, char_count,
                avg_confidence, estimated_wpm, output_mode, output_target_app,
                inserted_successfully, error_message, created_at,
                audio_path, original_raw_text, reprocessed_count,
                original_model_id, original_language, original_avg_confidence
         FROM sessions
         {where_clause}
         ORDER BY started_at DESC
         LIMIT ? OFFSET ?"
    );

    param_values.push(Box::new(limit));
    param_values.push(Box::new(offset));

    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(&sql).map_err(AppError::from)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = param_values.iter().map(|b| b.as_ref()).collect();
    let rows = stmt
        .query_map(params_refs.as_slice(), row_to_session)
        .map_err(AppError::from)?
        .collect::<rusqlite::Result<Vec<Session>>>()
        .map_err(AppError::from)?;

    Ok(rows)
}

/// Returns a single session together with all its segments.
pub fn get_session(db: &DbConn, id: &str) -> CmdResult<SessionWithSegments> {
    let conn = db.lock().unwrap();

    let session = conn
        .query_row(
            "SELECT id, started_at, ended_at, duration_ms, language, model_id, trigger_type,
                    input_device_id, raw_text, cleaned_text, word_count, char_count,
                    avg_confidence, estimated_wpm, output_mode, output_target_app,
                    inserted_successfully, error_message, created_at,
                    audio_path, original_raw_text, reprocessed_count,
                    original_model_id, original_language, original_avg_confidence
             FROM sessions WHERE id = ?1",
            params![id],
            row_to_session,
        )
        .map_err(|e| AppError(format!("Session not found: {e}")))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, session_id, start_ms, end_ms, text, confidence, segment_index
             FROM session_segments
             WHERE session_id = ?1
             ORDER BY segment_index",
        )
        .map_err(AppError::from)?;

    let segments = stmt
        .query_map(params![id], |row| {
            Ok(SessionSegment {
                id: row.get(0)?,
                session_id: row.get(1)?,
                start_ms: row.get(2)?,
                end_ms: row.get(3)?,
                text: row.get(4)?,
                confidence: row.get(5)?,
                segment_index: row.get(6)?,
            })
        })
        .map_err(AppError::from)?
        .collect::<rusqlite::Result<Vec<SessionSegment>>>()
        .map_err(AppError::from)?;

    Ok(SessionWithSegments { session, segments })
}

/// Deletes a session by id. Associated segments are removed via ON DELETE CASCADE.
pub fn delete_session(db: &DbConn, id: &str) -> CmdResult<()> {
    let conn = db.lock().unwrap();
    let n = conn
        .execute("DELETE FROM sessions WHERE id = ?1", params![id])
        .map_err(AppError::from)?;
    if n == 0 {
        return Err(AppError(format!("Session '{id}' not found")));
    }
    Ok(())
}

/// Deletes multiple sessions in a single transaction. Segments are removed via ON DELETE CASCADE.
pub fn bulk_delete_sessions(db: &DbConn, ids: &[String]) -> CmdResult<usize> {
    if ids.is_empty() {
        return Ok(0);
    }
    let conn = db.lock().unwrap();
    let placeholders = (1..=ids.len())
        .map(|i| format!("?{i}"))
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!("DELETE FROM sessions WHERE id IN ({placeholders})");
    let params_refs: Vec<&dyn rusqlite::ToSql> =
        ids.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
    let n = conn
        .execute(&sql, params_refs.as_slice())
        .map_err(AppError::from)?;
    Ok(n)
}

/// Returns sessions matching the given ids, ordered newest first.
pub fn get_sessions_by_ids(db: &DbConn, ids: &[String]) -> CmdResult<Vec<Session>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let placeholders = (1..=ids.len())
        .map(|i| format!("?{i}"))
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT id, started_at, ended_at, duration_ms, language, model_id, trigger_type,
                input_device_id, raw_text, cleaned_text, word_count, char_count,
                avg_confidence, estimated_wpm, output_mode, output_target_app,
                inserted_successfully, error_message, created_at,
                audio_path, original_raw_text, reprocessed_count,
                original_model_id, original_language, original_avg_confidence
         FROM sessions
         WHERE id IN ({placeholders})
         ORDER BY started_at DESC"
    );
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare(&sql).map_err(AppError::from)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> =
        ids.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
    let rows = stmt
        .query_map(params_refs.as_slice(), row_to_session)
        .map_err(AppError::from)?
        .collect::<rusqlite::Result<Vec<Session>>>()
        .map_err(AppError::from)?;
    Ok(rows)
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn row_to_session(row: &rusqlite::Row<'_>) -> rusqlite::Result<Session> {
    Ok(Session {
        id: row.get(0)?,
        started_at: row.get(1)?,
        ended_at: row.get(2)?,
        duration_ms: row.get(3)?,
        language: row.get(4)?,
        model_id: row.get(5)?,
        trigger_type: row.get(6)?,
        input_device_id: row.get(7)?,
        raw_text: row.get(8)?,
        cleaned_text: row.get(9)?,
        word_count: row.get(10)?,
        char_count: row.get(11)?,
        avg_confidence: row.get(12)?,
        estimated_wpm: row.get(13)?,
        output_mode: row.get(14)?,
        output_target_app: row.get(15)?,
        inserted_successfully: row.get::<_, i64>(16)? != 0,
        error_message: row.get(17)?,
        created_at: row.get(18)?,
        audio_path: row.get(19)?,
        original_raw_text: row.get(20)?,
        reprocessed_count: row.get(21)?,
        original_model_id: row.get(22)?,
        original_language: row.get(23)?,
        original_avg_confidence: row.get(24)?,
    })
}

/// Updates a session's text fields after reprocessing.
pub fn update_session_reprocess(
    db: &DbConn,
    session_id: &str,
    raw_text: &str,
    cleaned_text: &str,
    language: &str,
    model_id: &str,
) -> CmdResult<()> {
    let conn = db.lock().unwrap();
    let word_count = cleaned_text.split_whitespace().count() as i64;
    let char_count = cleaned_text.chars().count() as i64;
    let n = conn
        .execute(
            "UPDATE sessions SET
                raw_text = ?1, cleaned_text = ?2, word_count = ?3, char_count = ?4,
                language = ?5, model_id = ?6,
                reprocessed_count = reprocessed_count + 1
             WHERE id = ?7",
            params![
                raw_text,
                cleaned_text,
                word_count,
                char_count,
                language,
                model_id,
                session_id
            ],
        )
        .map_err(AppError::from)?;
    if n == 0 {
        return Err(AppError(format!("Session '{session_id}' not found")));
    }
    Ok(())
}
