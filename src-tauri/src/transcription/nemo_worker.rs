use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Manager};

use crate::errors::{AppError, CmdResult};
use crate::transcription::types::{TranscriptSegment, TranscriptWord};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeHealth {
    pub runtime: String,
    pub available: bool,
    pub configured: bool,
    pub message: String,
    pub python_path: Option<String>,
    pub detail: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NemoTranscript {
    pub text: String,
    pub segments: Vec<TranscriptSegment>,
    pub words: Vec<TranscriptWord>,
    pub detected_language: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkerResponse {
    #[serde(rename = "type")]
    kind: String,
    ok: Option<bool>,
    message: Option<String>,
    text: Option<String>,
    segments: Option<Vec<WorkerSegment>>,
    words: Option<Vec<WorkerWord>>,
    detected_language: Option<String>,
    python_path: Option<String>,
    detail: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkerSegment {
    start_ms: i64,
    end_ms: i64,
    text: String,
    confidence: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkerWord {
    start_ms: i64,
    end_ms: i64,
    text: String,
    confidence: Option<f32>,
}

pub fn check_health(app: &AppHandle, configured_python: Option<&str>) -> RuntimeHealth {
    let script = match resolve_worker_script(app) {
        Ok(path) => path,
        Err(e) => {
            return RuntimeHealth {
                runtime: "optional-nemo".to_string(),
                available: false,
                configured: configured_python.is_some(),
                message: "NeMo worker script was not found.".to_string(),
                python_path: configured_python.map(str::to_string),
                detail: Some(e.to_string()),
            };
        }
    };

    let python = match resolve_python(configured_python) {
        Ok(path) => path,
        Err(e) => {
            return RuntimeHealth {
                runtime: "optional-nemo".to_string(),
                available: false,
                configured: configured_python.is_some(),
                message: "Python was not found for the optional NeMo runtime.".to_string(),
                python_path: configured_python.map(str::to_string),
                detail: Some(e.to_string()),
            };
        }
    };

    match run_health_command(&python, &script) {
        Ok(resp) => RuntimeHealth {
            runtime: "optional-nemo".to_string(),
            available: resp.ok.unwrap_or(false),
            configured: true,
            message: resp
                .message
                .unwrap_or_else(|| "NeMo runtime health check completed.".to_string()),
            python_path: resp
                .python_path
                .or_else(|| Some(python.display().to_string())),
            detail: resp.detail,
        },
        Err(e) => RuntimeHealth {
            runtime: "optional-nemo".to_string(),
            available: false,
            configured: true,
            message: "NeMo runtime health check failed.".to_string(),
            python_path: Some(python.display().to_string()),
            detail: Some(e.to_string()),
        },
    }
}

pub fn transcribe_file(
    app: &AppHandle,
    model_path: &Path,
    wav_path: &Path,
    language: &str,
    configured_python: Option<&str>,
) -> CmdResult<NemoTranscript> {
    let script = resolve_worker_script(app)?;
    let python = resolve_python(configured_python)?;

    let mut command = Command::new(&python);
    command
        .arg(&script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);
    let mut child = command
        .spawn()
        .map_err(|e| format!("Failed to start NeMo worker: {e}"))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| AppError("NeMo worker stdin was not available.".to_string()))?;
        writeln!(
            stdin,
            "{}",
            json!({
                "type": "load",
                "modelPath": model_path.to_string_lossy(),
                "language": language
            })
        )
        .map_err(|e| format!("Failed to send NeMo load request: {e}"))?;
        writeln!(
            stdin,
            "{}",
            json!({
                "type": "transcribe_file",
                "audioPath": wav_path.to_string_lossy(),
                "language": language
            })
        )
        .map_err(|e| format!("Failed to send NeMo transcription request: {e}"))?;
    }

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| AppError("NeMo worker stdout was not available.".to_string()))?;
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read NeMo worker response: {e}"))?;
        if line.trim().is_empty() {
            continue;
        }
        let resp: WorkerResponse = serde_json::from_str(&line)
            .map_err(|e| format!("Invalid NeMo worker JSON response: {e}; line={line}"))?;
        match resp.kind.as_str() {
            "loaded" => continue,
            "transcription" => {
                let _ = child.kill();
                return Ok(response_to_transcript(resp));
            }
            "error" => {
                let _ = child.kill();
                return Err(resp
                    .message
                    .unwrap_or_else(|| "NeMo worker returned an error.".to_string())
                    .into());
            }
            _ => continue,
        }
    }

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for NeMo worker: {e}"))?;
    Err(format!("NeMo worker exited before returning a transcript: {status}").into())
}

fn response_to_transcript(resp: WorkerResponse) -> NemoTranscript {
    let words = resp
        .words
        .unwrap_or_default()
        .into_iter()
        .map(|w| TranscriptWord {
            start_ms: w.start_ms,
            end_ms: w.end_ms,
            text: w.text,
            confidence: w.confidence,
        })
        .collect::<Vec<_>>();

    let text = resp.text.unwrap_or_else(|| {
        words
            .iter()
            .map(|w| w.text.trim())
            .filter(|w| !w.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    });

    let mut segments = resp
        .segments
        .unwrap_or_default()
        .into_iter()
        .map(|s| TranscriptSegment {
            start_ms: s.start_ms,
            end_ms: s.end_ms,
            text: s.text,
            confidence: s.confidence,
        })
        .collect::<Vec<_>>();

    if segments.is_empty() && !text.trim().is_empty() {
        let start_ms = words.first().map(|w| w.start_ms).unwrap_or(0);
        let end_ms = words.last().map(|w| w.end_ms).unwrap_or(0);
        segments.push(TranscriptSegment {
            start_ms,
            end_ms,
            text: text.clone(),
            confidence: None,
        });
    }

    NemoTranscript {
        text,
        segments,
        words,
        detected_language: resp.detected_language,
    }
}

pub(crate) fn resolve_worker_script(app: &AppHandle) -> CmdResult<PathBuf> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("nemo_worker")
        .join("localvoice_nemo_worker.py");
    if manifest_path.exists() {
        return Ok(manifest_path);
    }

    let resource_path = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to resolve resource dir: {e}"))?
        .join("nemo_worker")
        .join("localvoice_nemo_worker.py");
    if resource_path.exists() {
        return Ok(resource_path);
    }

    Err(format!(
        "NeMo worker script not found at {} or {}",
        manifest_path.display(),
        resource_path.display()
    )
    .into())
}

pub(crate) fn resolve_python(configured_python: Option<&str>) -> CmdResult<PathBuf> {
    if let Some(path) = configured_python.filter(|p| !p.is_empty()) {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
        return Err(format!("Configured Python path does not exist: {path}").into());
    }

    for candidate in python_candidates() {
        if command_exists(candidate) {
            return Ok(PathBuf::from(candidate));
        }
    }

    Err("No Python executable found. Configure transcription.nemo.python_path.".into())
}

fn run_health_command(python: &Path, script: &Path) -> CmdResult<WorkerResponse> {
    let mut cmd = Command::new(python);
    cmd.arg(script).arg("--health");
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run NeMo health check: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("NeMo health check failed: {stderr}").into());
    }
    serde_json::from_str(stdout.trim())
        .map_err(|e| format!("Invalid NeMo health response: {e}; stdout={stdout}").into())
}

#[cfg(target_os = "windows")]
fn python_candidates() -> &'static [&'static str] {
    &["python.exe", "py.exe", "python3.exe"]
}

#[cfg(not(target_os = "windows"))]
fn python_candidates() -> &'static [&'static str] {
    &["python3", "python"]
}

fn command_exists(bin: &str) -> bool {
    std::env::var_os("PATH")
        .map(|path_var| std::env::split_paths(&path_var).any(|dir| dir.join(bin).exists()))
        .unwrap_or(false)
}
