use tauri::{AppHandle, Manager, State};

use crate::db::repositories::settings_repo;
use crate::errors::CmdResult;
use crate::state::AppState;
use crate::transcription::types::TranscriptionResult;
use crate::transcription::{engine, nemo_worker, orchestrator, parakeet_sidecar, whisper_sidecar};

/// Re-transcribes the most recently recorded WAV file.
///
/// `language` defaults to the `transcription.default_language` setting.
/// `model_id` optionally selects an installed registry model or direct model path.
#[tauri::command]
pub fn transcribe_last_recording(
    app: AppHandle,
    state: State<AppState>,
    language: Option<String>,
    model_id: Option<String>,
) -> CmdResult<TranscriptionResult> {
    let wav_path = state
        .last_wav_path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No recording available to transcribe")?;

    orchestrator::transcribe(&app, &wav_path, language.as_deref(), model_id.as_deref())
}

/// Returns the most recently completed transcription result, if any.
#[tauri::command]
pub fn get_last_transcription(state: State<AppState>) -> CmdResult<Option<TranscriptionResult>> {
    Ok(state.last_transcription.lock().unwrap().clone())
}

/// Lists installed transcription engine capabilities known to LocalVoice.
#[tauri::command]
pub fn list_transcription_engines() -> CmdResult<Vec<engine::TranscriptionEngineInfo>> {
    Ok(engine::list_engines())
}

/// Checks whether a runtime is usable on the current machine.
#[tauri::command]
pub fn check_transcription_runtime(
    app: AppHandle,
    runtime: String,
) -> CmdResult<nemo_worker::RuntimeHealth> {
    match runtime.as_str() {
        engine::ENGINE_WHISPER_CPP => {
            let result = whisper_sidecar::resolve_binary(&app);
            Ok(nemo_worker::RuntimeHealth {
                runtime: engine::ENGINE_WHISPER_CPP.to_string(),
                available: result.is_ok(),
                configured: true,
                message: if result.is_ok() {
                    "Whisper.cpp sidecar is available.".to_string()
                } else {
                    "Whisper.cpp sidecar is not available.".to_string()
                },
                python_path: None,
                detail: result.err().map(|e| e.to_string()),
            })
        }
        engine::ENGINE_PARAKEET_CPP => {
            let result = parakeet_sidecar::smoke_test(&app);
            Ok(nemo_worker::RuntimeHealth {
                runtime: engine::ENGINE_PARAKEET_CPP.to_string(),
                available: result.is_ok(),
                configured: true,
                message: if result.is_ok() {
                    "Parakeet.cpp sidecar is available.".to_string()
                } else {
                    "Parakeet.cpp sidecar is not available.".to_string()
                },
                python_path: None,
                detail: result.err().map(|e| e.to_string()),
            })
        }
        engine::RUNTIME_BUNDLED_SIDECAR => {
            let whisper = whisper_sidecar::resolve_binary(&app);
            let parakeet = parakeet_sidecar::smoke_test(&app);
            let available = whisper.is_ok() && parakeet.is_ok();
            let detail = [whisper.err(), parakeet.err()]
                .into_iter()
                .flatten()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            Ok(nemo_worker::RuntimeHealth {
                runtime: engine::RUNTIME_BUNDLED_SIDECAR.to_string(),
                available,
                configured: true,
                message: if available {
                    "Bundled Whisper.cpp and Parakeet.cpp sidecars are available.".to_string()
                } else {
                    "One or more bundled transcription sidecars are not available.".to_string()
                },
                python_path: None,
                detail: if detail.is_empty() {
                    None
                } else {
                    Some(detail)
                },
            })
        }
        engine::RUNTIME_OPTIONAL_NEMO | engine::ENGINE_NEMO => {
            let state = app.state::<AppState>();
            let settings = settings_repo::get_all(&state.db).unwrap_or_default();
            Ok(nemo_worker::check_health(
                &app,
                settings
                    .get("transcription.nemo.python_path")
                    .map(String::as_str),
            ))
        }
        other => Ok(nemo_worker::RuntimeHealth {
            runtime: other.to_string(),
            available: false,
            configured: false,
            message: format!("Unknown transcription runtime: {other}"),
            python_path: None,
            detail: None,
        }),
    }
}
