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

/// Locates the whisper-cli binary.
///
/// Search order:
/// 1. `WHISPER_BIN_PATH` environment variable (development override).
/// 2. Alongside the app executable (production bundle).
/// 3. Tauri resource directory / `binaries/` sub-folder (dev mode via `tauri dev`).
/// 4. `whisper-cli` on the system `PATH` (convenience fallback).
pub fn resolve_binary(app: &AppHandle) -> CmdResult<PathBuf> {
    #[cfg(target_os = "windows")]
    let bin_name = "whisper-cli-x86_64-pc-windows-msvc.exe";
    #[cfg(target_os = "macos")]
    let bin_name = if cfg!(target_arch = "aarch64") {
        "whisper-cli-aarch64-apple-darwin"
    } else {
        "whisper-cli-x86_64-apple-darwin"
    };
    #[cfg(target_os = "linux")]
    let bin_name = "whisper-cli-x86_64-unknown-linux-gnu";

    // 1. Explicit override — useful for CI and local dev without the sidecar.
    if let Ok(env_path) = std::env::var("WHISPER_BIN_PATH") {
        let p = PathBuf::from(&env_path);
        if p.exists() {
            return Ok(p);
        }
        return Err(
            format!("WHISPER_BIN_PATH is set to '{env_path}' but the file does not exist.").into(),
        );
    }

    // 2. Next to the running executable (installed app).
    if let Ok(exe) = std::env::current_exe() {
        let sibling = exe
            .parent()
            .unwrap_or(Path::new("."))
            .join(bin_name);
        if sibling.exists() {
            return Ok(sibling);
        }
    }

    // 3. Tauri resource directory → binaries/ (works with `tauri dev` when the
    //    binary is placed at src-tauri/binaries/{bin_name}).
    if let Ok(res_dir) = app.path().resource_dir() {
        let candidate = res_dir.join("binaries").join(bin_name);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    // 4. System PATH fallback.
    #[cfg(target_os = "windows")]
    let path_bin = "whisper-cli.exe";
    #[cfg(not(target_os = "windows"))]
    let path_bin = "whisper-cli";

    if which_in_path(path_bin) {
        return Ok(PathBuf::from(path_bin));
    }

    Err(format!(
        "whisper-cli binary not found.\n\
         Download a pre-built release from https://github.com/ggerganov/whisper.cpp/releases \
         and place it at src-tauri/binaries/{bin_name},\n\
         or set the WHISPER_BIN_PATH environment variable."
    )
    .into())
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
/// Writes a JSON sidecar file alongside `output_prefix.json` when `--output-json` is
/// supported. Falls back gracefully if the flag is not understood.
pub fn invoke(
    binary: &Path,
    model: &Path,
    wav: &Path,
    language: &str,
    output_prefix: &Path,
) -> CmdResult<SidecarOutput> {
    let output = Command::new(binary)
        .args([
            "-m",
            &model.to_string_lossy(),
            "-f",
            &wav.to_string_lossy(),
            "-l",
            language,
            "-oj",                                  // write JSON file
            "-of",
            &output_prefix.to_string_lossy(),
            "--no-prints",                          // suppress info banners
        ])
        .output()
        .map_err(|e| format!("Failed to spawn whisper-cli: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("whisper-cli exited with status {}: {stderr}", output.status).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();

    let json_path = {
        let candidate = output_prefix.with_extension("json");
        if candidate.exists() {
            Some(candidate)
        } else {
            None
        }
    };

    Ok(SidecarOutput { stdout, json_path })
}

fn which_in_path(bin: &str) -> bool {
    std::env::var_os("PATH")
        .map(|path_var| {
            std::env::split_paths(&path_var).any(|dir| dir.join(bin).exists())
        })
        .unwrap_or(false)
}
