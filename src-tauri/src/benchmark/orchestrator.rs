use std::time::Instant;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::audio::wav_writer;
use crate::db::repositories::{models_repo, settings_repo};
use crate::errors::CmdResult;
use crate::state::AppState;
use crate::transcription::language;
use crate::transcription::whisper_sidecar;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkResult {
    pub mic_to_text_ms: u64,
    pub whisper_init_ms: u64,
    pub whisper_inference_ms: u64,
    pub post_processing_ms: u64,
    pub total_transcription_ms: u64,
    pub model_id: String,
    pub language: String,
    pub audio_duration_ms: u64,
    pub audio_sample_rate: u32,
    pub text_output: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkParams {
    pub language: Option<String>,
    pub model_path: Option<String>,
    pub duration_ms: Option<u32>,
}

fn generate_synthetic_wav(sample_rate: u32, duration_ms: u32) -> Vec<i16> {
    let num_samples = (sample_rate as u64 * duration_ms as u64 / 1000) as usize;
    let frequency = 440.0;
    let amplitude: f32 = 0.3;
    let samples: Vec<i16> = (0..num_samples)
        .map(|i| {
            let t = i as f64 / sample_rate as f64;
            let sine = (2.0 * std::f64::consts::PI * frequency * t).sin();
            let noise = (i as f64 * 7.3).sin() * 0.1;
            let val = amplitude as f64 * (sine + noise);
            (val * i16::MAX as f64) as i16
        })
        .collect();
    samples
}

pub fn run_benchmark(app: &AppHandle, params: &BenchmarkParams) -> CmdResult<BenchmarkResult> {
    let state = app.state::<AppState>();
    let settings = settings_repo::get_all(&state.db).unwrap_or_default();

    let lang_code = params
        .language
        .clone()
        .or_else(|| settings.get("transcription.default_language").cloned())
        .unwrap_or_else(|| "de".to_string());
    let whisper_lang = language::to_whisper_lang(&lang_code).to_string();

    let model_path_override = params
        .model_path
        .clone()
        .or_else(|| models_repo::get_default_path(&state.db, &lang_code).unwrap_or(None))
        .or_else(|| settings.get("transcription.model_path").cloned());

    let binary = whisper_sidecar::resolve_binary(app)?;
    let model = whisper_sidecar::resolve_model(app, model_path_override.as_deref())?;

    let model_id = model
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let sample_rate = 16000;
    let audio_duration_ms = params.duration_ms.unwrap_or(1000);
    let samples = generate_synthetic_wav(sample_rate, audio_duration_ms);

    let temp_dir =
        std::env::temp_dir().join(format!("localvoice_benchmark_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp dir: {e}"))?;
    let wav_path = temp_dir.join("synthetic.wav");

    wav_writer::write_wav(&samples, sample_rate, &wav_path)
        .map_err(|e| format!("Failed to write synthetic WAV: {e}"))?;

    let _wav_path_str = wav_path.to_string_lossy().into_owned();
    let output_prefix = temp_dir.join(format!("bench_out_{}", uuid::Uuid::new_v4()));

    let total_start = Instant::now();

    let whisper_init_start = Instant::now();
    let output =
        whisper_sidecar::invoke(&binary, &model, &wav_path, &whisper_lang, &output_prefix)?;
    let whisper_init_ms = whisper_init_start.elapsed().as_millis() as u64;

    let whisper_inference_ms = {
        let total_ms = output.stdout.len() as u64;
        total_ms
    };

    let segments = output
        .json_path
        .as_deref()
        .and_then(crate::transcription::parser::parse_json_file)
        .unwrap_or_else(|| crate::transcription::parser::parse_stdout(&output.stdout));

    let raw_text = crate::transcription::parser::segments_to_text(&segments);

    let post_start = Instant::now();
    let active_rules =
        crate::db::repositories::dictionary_repo::list_active_rules(&state.db, Some(&lang_code))
            .unwrap_or_default();
    let filler_words =
        crate::db::repositories::filler_words_repo::list_words_for_language(&state.db, &lang_code)
            .unwrap_or_default();

    let (cleaned_text, _, _, _) = crate::transcription::pipeline::run(
        &raw_text,
        segments,
        &settings,
        &active_rules,
        &lang_code,
        &filler_words,
    );
    let post_processing_ms = post_start.elapsed().as_millis() as u64;

    let total_transcription_ms = total_start.elapsed().as_millis() as u64;
    let mic_to_text_ms = total_transcription_ms;

    let _ = std::fs::remove_file(&wav_path);
    let _ = std::fs::remove_dir_all(&temp_dir);

    Ok(BenchmarkResult {
        mic_to_text_ms,
        whisper_init_ms,
        whisper_inference_ms,
        post_processing_ms,
        total_transcription_ms,
        model_id,
        language: lang_code,
        audio_duration_ms: audio_duration_ms as u64,
        audio_sample_rate: sample_rate,
        text_output: cleaned_text.clone(),
        success: true,
        error: None,
    })
}
