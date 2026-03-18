use tauri::AppHandle;

use crate::benchmark::orchestrator::{run_benchmark, BenchmarkParams, BenchmarkResult};
use crate::errors::CmdResult;

#[tauri::command]
pub fn run_transcription_benchmark(
    app: AppHandle,
    language: Option<String>,
    model_path: Option<String>,
    duration_ms: Option<u32>,
) -> CmdResult<BenchmarkResult> {
    let params = BenchmarkParams {
        language,
        model_path,
        duration_ms,
    };
    run_benchmark(&app, &params)
}
