use std::path::{Path, PathBuf};
use std::process::Command;

use tauri::{AppHandle, Manager};

use crate::errors::CmdResult;

/// Raw output from running whisper-cli.
pub struct SidecarOutput {
    /// Combined stdout from the process.
    pub stdout: String,
    /// Path to the `.json` output file if `--output-json` was requested and the file exists.
    pub json_path: Option<PathBuf>,
}

/// Known whisper CLI executable names (checked in order, both in dirs and on PATH).
#[cfg(target_os = "windows")]
const WHISPER_EXE_NAMES: &[&str] = &[
    "whisper-cli-x86_64-pc-windows-msvc.exe",
    "whisper-cli.exe",
    "main.exe",
];
#[cfg(not(target_os = "windows"))]
const WHISPER_EXE_NAMES: &[&str] = &["whisper-cli", "main"];

/// Locates the whisper-cli binary.
///
/// Search order:
/// 1. `WHISPER_BIN_PATH` environment variable (explicit override, highest priority).
/// 2. Alongside the running executable (installed/bundled app).
/// 3. Tauri resource dir → `binaries/` flat lookup (Tauri sidecar convention).
/// 4. Recursive scan of `binaries/` subdirectories — handles unpacked release zips
///    like `binaries/whisper-bin-x64/Release/whisper-cli.exe`.
/// 5. `whisper-cli` / `main` on the system PATH.
pub fn resolve_binary(app: &AppHandle) -> CmdResult<PathBuf> {
    // 1. Explicit override.
    if let Ok(env_path) = std::env::var("WHISPER_BIN_PATH") {
        let p = PathBuf::from(&env_path);
        if p.exists() {
            return Ok(p);
        }
        return Err(
            format!("WHISPER_BIN_PATH is set to '{env_path}' but the file does not exist.").into(),
        );
    }

    // 2. Alongside the running executable (production bundle).
    if let Ok(exe) = std::env::current_exe() {
        let exe_dir = exe.parent().unwrap_or(Path::new("."));
        for name in WHISPER_EXE_NAMES {
            let p = exe_dir.join(name);
            if p.exists() {
                return Ok(p);
            }
        }
    }

    // 3. Tauri resource dir → binaries/ flat (Tauri sidecar convention).
    // 4. Recursive scan of binaries/ subdirectories (unpacked release zip layout).
    if let Ok(res_dir) = app.path().resource_dir() {
        let binaries_dir = res_dir.join("binaries");

        // 3. Flat lookup first.
        for name in WHISPER_EXE_NAMES {
            let p = binaries_dir.join(name);
            if p.exists() {
                return Ok(p);
            }
        }

        // 4. Recursive scan — walks all subdirs, returns first match.
        if let Some(found) = scan_dir_for_whisper(&binaries_dir) {
            return Ok(found);
        }
    }

    // 5. System PATH.
    for name in WHISPER_EXE_NAMES {
        if which_in_path(name) {
            return Ok(PathBuf::from(name));
        }
    }

    Err(
        "whisper-cli binary not found.\n\
         Options:\n\
         • Unzip a whisper.cpp release into src-tauri/binaries/ (any subdirectory layout).\n\
         • Set WHISPER_BIN_PATH=/full/path/to/whisper-cli.exe\n\
         • Download from https://github.com/ggerganov/whisper.cpp/releases"
            .into(),
    )
}

/// Walks `dir` recursively and returns the first path whose filename matches
/// a known whisper CLI executable name.
fn scan_dir_for_whisper(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut subdirs: Vec<PathBuf> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // Check against all known names (case-insensitive on Windows).
            let name_lower = name.to_lowercase();
            for known in WHISPER_EXE_NAMES {
                if name_lower == known.to_lowercase() {
                    return Some(path);
                }
            }
        }
    }

    // Recurse into subdirectories after checking this level.
    for sub in subdirs {
        if let Some(found) = scan_dir_for_whisper(&sub) {
            return Some(found);
        }
    }

    None
}

/// Locates the whisper model file.
///
/// Search order:
/// 1. `WHISPER_MODEL_PATH` environment variable.
/// 2. `transcription.model_path` setting in the database (via `model_path_override`).
/// 3. First `*.bin` file found in `{app_data_dir}/models/`.
pub fn resolve_model(
    app: &AppHandle,
    model_path_override: Option<&str>,
) -> CmdResult<PathBuf> {
    // 1. Env override.
    if let Ok(env_path) = std::env::var("WHISPER_MODEL_PATH") {
        let p = PathBuf::from(&env_path);
        if p.exists() {
            return Ok(p);
        }
    }

    // 2. Explicit path from settings.
    if let Some(path_str) = model_path_override.filter(|s| !s.is_empty()) {
        let p = PathBuf::from(path_str);
        if p.exists() {
            return Ok(p);
        }
        return Err(
            format!("Model path from settings does not exist: {path_str}").into(),
        );
    }

    // 3. Scan {app_data_dir}/models/ for the first .bin file.
    if let Ok(data_dir) = app.path().app_data_dir() {
        let models_dir = data_dir.join("models");
        if let Ok(entries) = std::fs::read_dir(&models_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("bin") {
                    return Ok(path);
                }
            }
        }
    }

    Err(
        "No whisper model found. Download a model .bin file and place it in \
         <app-data>/models/ or set WHISPER_MODEL_PATH."
            .into(),
    )
}

/// Invokes `whisper-cli` and returns the combined output.
///
/// First attempts the full flag set (`-ojf -of <prefix>`). If that exits non-zero,
/// retries without optional flags (`-np`, `-ojf`, `-of`) to support older binaries
/// that do not recognise all flags — falling back to plain stdout parsing.
pub fn invoke(
    binary: &Path,
    model: &Path,
    wav: &Path,
    language: &str,
    output_prefix: &Path,
) -> CmdResult<SidecarOutput> {
    // Build owned strings for args that come from borrowed paths.
    let model_str = model.to_string_lossy().into_owned();
    let wav_str = wav.to_string_lossy().into_owned();
    let prefix_str = output_prefix.to_string_lossy().into_owned();

    // ── Full invocation (JSON output + confidence data) ────────────────────────
    let full_out = Command::new(binary)
        .args([
            "-m", &model_str,
            "-f", &wav_str,
            "-l", language,
            "-ojf",          // full JSON with per-token confidence
            "-of", &prefix_str,
        ])
        .output()
        .map_err(|e| format!("Failed to spawn whisper-cli: {e}"))?;

    if full_out.status.success() {
        let stdout = String::from_utf8_lossy(&full_out.stdout).into_owned();
        let json_path = {
            let candidate = output_prefix.with_extension("json");
            if candidate.exists() { Some(candidate) } else { None }
        };
        return Ok(SidecarOutput { stdout, json_path });
    }

    let full_stderr = String::from_utf8_lossy(&full_out.stderr).into_owned();
    let full_stdout = String::from_utf8_lossy(&full_out.stdout).into_owned();
    log::warn!(
        "whisper-cli full invocation failed ({}); retrying with minimal flags.\nstderr: {full_stderr}\nstdout: {full_stdout}",
        full_out.status
    );

    // ── Minimal fallback (plain stdout — older binaries) ───────────────────────
    let min_out = Command::new(binary)
        .args([
            "-m", &model_str,
            "-f", &wav_str,
            "-l", language,
        ])
        .output()
        .map_err(|e| format!("Failed to spawn whisper-cli (fallback): {e}"))?;

    if min_out.status.success() {
        let stdout = String::from_utf8_lossy(&min_out.stdout).into_owned();
        return Ok(SidecarOutput { stdout, json_path: None });
    }

    let min_stderr = String::from_utf8_lossy(&min_out.stderr).into_owned();
    let min_stdout = String::from_utf8_lossy(&min_out.stdout).into_owned();
    Err(format!(
        "whisper-cli failed ({}).\nstderr: {min_stderr}\nstdout: {min_stdout}",
        min_out.status
    )
    .into())
}

fn which_in_path(bin: &str) -> bool {
    std::env::var_os("PATH")
        .map(|path_var| {
            std::env::split_paths(&path_var).any(|dir| dir.join(bin).exists())
        })
        .unwrap_or(false)
}
