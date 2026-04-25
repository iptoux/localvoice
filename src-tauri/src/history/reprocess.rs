use tauri::{AppHandle, Manager};

use crate::db::repositories::{
    dictionary_repo, filler_words_repo, models_repo, sessions_repo, settings_repo,
};
use crate::errors::CmdResult;
use crate::state::AppState;
use crate::transcription::{language, parser, pipeline, whisper_sidecar};

/// Re-runs transcription on a previously saved session using its persisted audio file.
///
/// - If this is the first reprocess, stores current `raw_text` in `original_raw_text`.
/// - Increments `reprocessed_count`.
/// - Updates `raw_text`, `cleaned_text`, `word_count`, `char_count`, `language`, `model_id`.
pub fn reprocess_session(
    app: &AppHandle,
    session_id: &str,
    language_override: Option<&str>,
    model_id_override: Option<&str>,
) -> CmdResult<()> {
    let state = app.state::<AppState>();

    // Load the session to get audio_path.
    let detail = sessions_repo::get_session(&state.db, session_id)?;
    let session = &detail.session;

    let audio_path = session.audio_path.as_deref().ok_or_else(|| {
        "No audio file available for this session — reprocessing requires kept audio".to_string()
    })?;

    // Verify the audio file still exists.
    if !std::path::Path::new(audio_path).exists() {
        return Err(format!("Audio file not found: {audio_path}").into());
    }

    // Preserve original metadata on first reprocess, or backfill if columns were added later.
    if session.reprocessed_count == 0 || session.original_model_id.is_none() {
        let conn = state.db.lock().unwrap();
        conn.execute(
            "UPDATE sessions SET
                original_raw_text      = COALESCE(original_raw_text, raw_text),
                original_model_id      = COALESCE(original_model_id, model_id),
                original_language      = COALESCE(original_language, language),
                original_avg_confidence = COALESCE(original_avg_confidence, avg_confidence)
             WHERE id = ?1",
            rusqlite::params![session_id],
        )
        .map_err(|e| format!("Failed to preserve original metadata: {e}"))?;
    }

    let settings = settings_repo::get_all(&state.db).unwrap_or_default();

    let lang_code = language_override
        .map(|s| s.to_string())
        .unwrap_or_else(|| session.language.clone());
    let whisper_lang = language::to_whisper_lang(&lang_code).to_string();

    // Resolve model path.
    let effective_model_path = if let Some(mid) = model_id_override {
        // Look up the model by key to get its local_path.
        models_repo::get_model_path(&state.db, mid)?
    } else {
        models_repo::get_default_path(&state.db, &lang_code).unwrap_or(None)
    };

    let binary = whisper_sidecar::resolve_binary(app)?;
    let model = whisper_sidecar::resolve_model(app, effective_model_path.as_deref())?;

    let model_id = model
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Output prefix for whisper JSON.
    let wav_dir = std::path::Path::new(audio_path)
        .parent()
        .unwrap_or(std::path::Path::new("."));
    let output_prefix = wav_dir.join(format!("localvoice_reprocess_{}", uuid::Uuid::new_v4()));

    let output = whisper_sidecar::invoke(
        &binary,
        &model,
        std::path::Path::new(audio_path),
        &whisper_lang,
        &output_prefix,
    )?;

    let segments = output
        .json_path
        .as_deref()
        .and_then(parser::parse_json_file)
        .unwrap_or_else(|| parser::parse_stdout(&output.stdout));

    // Clean up temporary JSON output.
    if let Some(json_path) = &output.json_path {
        let _ = std::fs::remove_file(json_path);
    }

    let raw_text = parser::segments_to_text(&segments);

    let active_rules =
        dictionary_repo::list_active_rules(&state.db, Some(&lang_code)).unwrap_or_default();

    let filler_words =
        filler_words_repo::list_words_for_language(&state.db, &lang_code).unwrap_or_default();

    let (cleaned_text, cleaned_segments, _fired_ids, _removed_fillers) = pipeline::run(
        &raw_text,
        segments,
        &settings,
        &active_rules,
        &lang_code,
        &filler_words,
    );

    // Update session record.
    sessions_repo::update_session_reprocess(
        &state.db,
        session_id,
        &raw_text,
        &cleaned_text,
        &lang_code,
        &model_id,
    )?;

    // Replace segments.
    {
        let conn = state.db.lock().unwrap();
        conn.execute(
            "DELETE FROM session_segments WHERE session_id = ?1",
            rusqlite::params![session_id],
        )
        .map_err(|e| format!("Failed to delete old segments: {e}"))?;
    }
    sessions_repo::insert_segments(&state.db, session_id, &cleaned_segments)?;

    log::info!(
        "Reprocessed session {session_id} with model '{model_id}' lang '{lang_code}' — {} words",
        cleaned_text.split_whitespace().count()
    );

    Ok(())
}
