use tauri::{AppHandle, Manager};

use crate::db::repositories::sessions_repo;
use crate::errors::CmdResult;
use crate::state::AppState;

/// Re-runs transcription on a previously saved session using its persisted audio file.
///
/// The reprocess path uses the same engine-aware orchestrator as normal
/// transcription, so Whisper, Parakeet.cpp, and optional NeMo models all share
/// post-processing and persistence behavior.
pub fn reprocess_session(
    app: &AppHandle,
    session_id: &str,
    language_override: Option<&str>,
    model_id_override: Option<&str>,
) -> CmdResult<()> {
    let state = app.state::<AppState>();

    let detail = sessions_repo::get_session(&state.db, session_id)?;
    let session = &detail.session;

    let audio_path = session.audio_path.as_deref().ok_or_else(|| {
        "No audio file available for this session - reprocessing requires kept audio".to_string()
    })?;

    if !std::path::Path::new(audio_path).exists() {
        return Err(format!("Audio file not found: {audio_path}").into());
    }

    if session.reprocessed_count == 0 || session.original_model_id.is_none() {
        let conn = state.db.lock().unwrap();
        conn.execute(
            "UPDATE sessions SET
                original_raw_text       = COALESCE(original_raw_text, raw_text),
                original_model_id       = COALESCE(original_model_id, model_id),
                original_language       = COALESCE(original_language, language),
                original_avg_confidence = COALESCE(original_avg_confidence, avg_confidence)
             WHERE id = ?1",
            rusqlite::params![session_id],
        )
        .map_err(|e| format!("Failed to preserve original metadata: {e}"))?;
    }

    let lang_code = language_override.unwrap_or(&session.language);
    let result = crate::transcription::orchestrator::transcribe(
        app,
        audio_path,
        Some(lang_code),
        model_id_override,
    )?;

    sessions_repo::update_session_reprocess(
        &state.db,
        session_id,
        &result.raw_text,
        &result.cleaned_text,
        &result.language,
        &result.model_id,
        &result.engine,
        &result.artifact_format,
        &result.runtime,
    )?;

    {
        let conn = state.db.lock().unwrap();
        conn.execute(
            "DELETE FROM session_segments WHERE session_id = ?1",
            rusqlite::params![session_id],
        )
        .map_err(|e| format!("Failed to delete old segments: {e}"))?;
        conn.execute(
            "DELETE FROM session_words WHERE session_id = ?1",
            rusqlite::params![session_id],
        )
        .map_err(|e| format!("Failed to delete old words: {e}"))?;
    }
    sessions_repo::insert_segments(&state.db, session_id, &result.segments)?;
    sessions_repo::insert_words(&state.db, session_id, &result.words)?;

    log::info!(
        "Reprocessed session {session_id} with model '{}' lang '{}' - {} words",
        result.model_id,
        result.language,
        result.cleaned_text.split_whitespace().count()
    );

    Ok(())
}
