use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use tauri::{AppHandle, Manager};

/// Windows: CREATE_NO_WINDOW — prevents a console window from appearing.
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

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
const WHISPER_EXE_NAMES: &[&str] = &["whisper-cli.exe", "main.exe"];
#[cfg(not(target_os = "windows"))]
const WHISPER_EXE_NAMES: &[&str] = &["whisper-cli", "main"];

/// Locates the whisper-cli binary.
///
/// Search order:
/// 1. `WHISPER_BIN_PATH` environment variable (explicit override, highest priority).
/// 2. Alongside the running executable (installed/bundled app, production).
/// 3. `src-tauri/binaries/` via compile-time CARGO_MANIFEST_DIR (dev mode only).
/// 4. Tauri resource dir → `binaries/` flat, then recursive scan (production
///    resource layout and unpacked zip fallback).
/// 5. `whisper-cli` on the system PATH.
pub fn resolve_binary(app: &AppHandle) -> CmdResult<PathBuf> {
    let mut searched: Vec<String> = Vec::new();

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

    // 2. Alongside the running executable (production bundle — binary is in exe dir).
    if let Ok(exe) = std::env::current_exe() {
        let exe_dir = exe.parent().unwrap_or(Path::new("."));
        for name in WHISPER_EXE_NAMES {
            let p = exe_dir.join(name);
            searched.push(p.display().to_string());
            if p.exists() {
                return Ok(p);
            }
        }
    }

    // 3. CARGO_MANIFEST_DIR/binaries/ — compile-time constant, always points to
    //    src-tauri/. Reliable in dev mode where resource_dir() may differ.
    {
        let manifest_binaries = Path::new(env!("CARGO_MANIFEST_DIR")).join("binaries");

        // 3a. Flat lookup (whisper-cli.exe + DLLs co-located).
        for name in WHISPER_EXE_NAMES {
            let p = manifest_binaries.join(name);
            searched.push(p.display().to_string());
            if p.exists() {
                return Ok(p);
            }
        }

        // 3b. Recursive scan (release zip unpacked as a subdir, e.g. whisper-bin-x64/).
        if let Some(found) = scan_dir_for_whisper(&manifest_binaries) {
            return Ok(found);
        }
    }

    // 4. Tauri resource dir — check root first (DLLs co-located with exe via
    //    resources map "": ""), then binaries/ subdir as fallback.
    if let Ok(res_dir) = app.path().resource_dir() {
        // 4a. Root of resource dir (production: whisper-cli.exe + DLLs mapped to "").
        for name in WHISPER_EXE_NAMES {
            let p = res_dir.join(name);
            searched.push(p.display().to_string());
            if p.exists() {
                return Ok(p);
            }
        }

        // 4b. binaries/ subdir (legacy layout).
        let binaries_dir = res_dir.join("binaries");
        for name in WHISPER_EXE_NAMES {
            let p = binaries_dir.join(name);
            searched.push(p.display().to_string());
            if p.exists() {
                return Ok(p);
            }
        }

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

    Err(format!(
        "whisper-cli binary not found.\n\
         Searched:\n{}\n\
         Options:\n\
         • Place whisper-cli.exe and its DLLs into src-tauri/binaries/\n\
         • Set WHISPER_BIN_PATH=/full/path/to/whisper-cli.exe\n\
         • Download from https://github.com/ggerganov/whisper.cpp/releases",
        searched.iter().map(|p| format!("  • {p}")).collect::<Vec<_>>().join("\n")
    )
    .into())
}

/// Walks `dir` recursively and returns the best whisper CLI executable found.
///
/// "Best" means: prefers a binary whose parent directory also contains DLL files
/// (so Windows can find co-located DLLs via the executable's own directory).
/// If no DLL-accompanied binary is found, returns any name-matching binary.
fn scan_dir_for_whisper(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut subdirs: Vec<PathBuf> = Vec::new();
    let mut candidate: Option<PathBuf> = None; // fallback if no DLL-accompanied one found

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let name_lower = name.to_lowercase();
            let is_whisper = WHISPER_EXE_NAMES
                .iter()
                .any(|known| name_lower == known.to_lowercase());
            if is_whisper {
                // Prefer this binary if the directory also contains DLLs.
                let dir_has_dlls = dir_contains_dll(dir);
                if dir_has_dlls {
                    return Some(path); // ideal: co-located DLLs → return immediately
                }
                candidate.get_or_insert(path);
            }
        }
    }

    // Recurse into subdirectories — a subdir result with DLLs wins over our candidate.
    for sub in subdirs {
        if let Some(found) = scan_dir_for_whisper(&sub) {
            // If the found binary is in a dir with DLLs, prefer it unconditionally.
            let found_dir = found.parent().unwrap_or(Path::new("."));
            if dir_contains_dll(found_dir) {
                return Some(found);
            }
            // Otherwise keep as fallback only if we have nothing better yet.
            candidate.get_or_insert(found);
        }
    }

    candidate
}

/// Returns true if `dir` contains at least one `.dll` file (non-recursive).
fn dir_contains_dll(dir: &Path) -> bool {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries.flatten().any(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("dll"))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

/// Locates the whisper model file.
///
/// Search order:
/// 1. `WHISPER_MODEL_PATH` environment variable.
/// 2. `transcription.model_path` setting in the database (via `model_path_override`).
/// 3. First `*.bin` file found in `{app_data_dir}/models/`.
pub fn resolve_model(
    _app: &AppHandle,
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

    // 3. No model configured — hard error, no fallback.
    Err(
        "No transcription model configured. Please go to Settings → Models, \
         download a model and set it as default for your language."
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

    // Set the working directory to the binary's own directory so that Windows
    // DLL search (step: current directory) can find co-located DLLs.
    let binary_dir = binary.parent().unwrap_or(Path::new("."));

    // Prepend the binary's own directory AND every DLL-containing directory
    // under any `binaries/` ancestor to PATH. This covers both the flat layout
    // (DLLs next to exe) and the legacy subdir layout.
    let extended_path = build_extended_path(binary);

    // ── Full invocation (JSON output + confidence data) ────────────────────────
    #[allow(unused_mut)]
    let mut full_cmd = Command::new(binary);
    full_cmd
        .args([
            "-m", &model_str,
            "-f", &wav_str,
            "-l", language,
            "-ojf",
            "-of", &prefix_str,
        ])
        .current_dir(binary_dir)
        .env("PATH", &extended_path);
    #[cfg(target_os = "windows")]
    full_cmd.creation_flags(CREATE_NO_WINDOW);
    let full_out = full_cmd.output()
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
    #[allow(unused_mut)]
    let mut min_cmd = Command::new(binary);
    min_cmd
        .args([
            "-m", &model_str,
            "-f", &wav_str,
            "-l", language,
        ])
        .current_dir(binary_dir)
        .env("PATH", &extended_path);
    #[cfg(target_os = "windows")]
    min_cmd.creation_flags(CREATE_NO_WINDOW);
    let min_out = min_cmd.output()
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

/// Returns a PATH value with the binary's own directory and all DLL-containing
/// directories under any `binaries/` ancestor prepended to the current system PATH.
///
/// On Windows this ensures that `ggml.dll`, `whisper.dll`, and any other co-located
/// DLLs are found even when the resolved binary lives in a different directory from
/// its DLLs.
///
/// On non-Windows platforms the current PATH is returned unchanged.
fn build_extended_path(binary: &Path) -> std::ffi::OsString {
    let current = std::env::var_os("PATH").unwrap_or_default();

    #[cfg(target_os = "windows")]
    {
        let binary_dir = binary.parent().unwrap_or(Path::new("."));
        let mut extra_dirs: Vec<PathBuf> = vec![binary_dir.to_path_buf()];

        // Also collect DLL dirs from any binaries/ ancestor tree.
        if let Some(binaries_root) = binary
            .ancestors()
            .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("binaries"))
        {
            collect_dll_dirs(binaries_root, &mut extra_dirs);
        }

        // Deduplicate while preserving order.
        extra_dirs.dedup();

        if !extra_dirs.is_empty() {
            if let Ok(mut prefix) = std::env::join_paths(&extra_dirs) {
                if !current.is_empty() {
                    prefix.push(";");
                    prefix.push(&current);
                }
                return prefix.to_os_string();
            }
        }
    }

    let _ = binary;
    current
}

/// Recursively collects directories under `dir` that contain at least one `.dll` file.
#[cfg(target_os = "windows")]
fn collect_dll_dirs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut has_dll = false;
    let mut subdirs: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("dll"))
            .unwrap_or(false)
        {
            has_dll = true;
        }
    }
    if has_dll {
        out.push(dir.to_path_buf());
    }
    for sub in subdirs {
        collect_dll_dirs(&sub, out);
    }
}

fn which_in_path(bin: &str) -> bool {
    std::env::var_os("PATH")
        .map(|path_var| {
            std::env::split_paths(&path_var).any(|dir| dir.join(bin).exists())
        })
        .unwrap_or(false)
}
