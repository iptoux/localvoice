use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use tauri::{AppHandle, Manager};

use crate::errors::CmdResult;
use crate::transcription::parakeet_runtime;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
const PARAKEET_EXE_NAMES: &[&str] = &["parakeet-cli.exe"];
#[cfg(not(target_os = "windows"))]
const PARAKEET_EXE_NAMES: &[&str] = &["parakeet-cli"];

pub struct ParakeetOutput {
    pub stdout: String,
}

pub fn resolve_binary(app: &AppHandle) -> CmdResult<PathBuf> {
    let mut searched: Vec<String> = Vec::new();

    if let Ok(env_path) = std::env::var("PARAKEET_BIN_PATH") {
        let p = PathBuf::from(&env_path);
        if p.exists() {
            return Ok(p);
        }
        return Err(format!(
            "PARAKEET_BIN_PATH is set to '{env_path}' but the file does not exist."
        )
        .into());
    }

    if let Ok(exe) = std::env::current_exe() {
        let exe_dir = exe.parent().unwrap_or(Path::new("."));
        for name in PARAKEET_EXE_NAMES {
            let p = exe_dir.join(name);
            searched.push(p.display().to_string());
            if p.exists() {
                return Ok(p);
            }
        }
    }

    let manifest_binaries = Path::new(env!("CARGO_MANIFEST_DIR")).join("binaries");
    for name in PARAKEET_EXE_NAMES {
        let p = manifest_binaries.join(name);
        searched.push(p.display().to_string());
        if p.exists() {
            return Ok(p);
        }
    }
    if let Some(found) = scan_dir_for_parakeet(&manifest_binaries) {
        return Ok(found);
    }

    if let Ok(res_dir) = app.path().resource_dir() {
        for name in PARAKEET_EXE_NAMES {
            let p = res_dir.join(name);
            searched.push(p.display().to_string());
            if p.exists() {
                return Ok(p);
            }
        }

        let binaries_dir = res_dir.join("binaries");
        for name in PARAKEET_EXE_NAMES {
            let p = binaries_dir.join(name);
            searched.push(p.display().to_string());
            if p.exists() {
                return Ok(p);
            }
        }
        if let Some(found) = scan_dir_for_parakeet(&binaries_dir) {
            return Ok(found);
        }
    }

    for name in PARAKEET_EXE_NAMES {
        if which_in_path(name) {
            return Ok(PathBuf::from(name));
        }
    }

    Err(format!(
        "parakeet-cli binary not found.\nSearched:\n{}\nOptions:\n  - Place parakeet-cli into src-tauri/binaries/\n  - Set PARAKEET_BIN_PATH=/full/path/to/parakeet-cli\n  - Use the setup-parakeet-cpp CI action for packaged builds",
        searched
            .iter()
            .map(|p| format!("  - {p}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
    .into())
}

pub fn resolve_model(model_path_override: Option<&str>) -> CmdResult<PathBuf> {
    if let Ok(env_path) = std::env::var("PARAKEET_MODEL_PATH") {
        let p = PathBuf::from(&env_path);
        if p.exists() {
            return Ok(p);
        }
    }

    if let Some(path_str) = model_path_override.filter(|s| !s.is_empty()) {
        let p = PathBuf::from(path_str);
        if p.exists() {
            return Ok(p);
        }
        return Err(format!("Parakeet model path does not exist: {path_str}").into());
    }

    Err(
        "No Parakeet model configured. Download a GGUF or .nemo model and set it as default."
            .into(),
    )
}

pub fn invoke(
    binary: &Path,
    model: &Path,
    wav: &Path,
    language: &str,
    device: Option<&str>,
) -> CmdResult<ParakeetOutput> {
    let model_str = model.to_string_lossy().into_owned();
    let wav_str = wav.to_string_lossy().into_owned();
    let binary_dir = binary.parent().unwrap_or(Path::new("."));

    #[allow(unused_mut)]
    let mut cmd = Command::new(binary);
    parakeet_runtime::configure_command_environment(&mut cmd, binary);
    cmd.args([
        "transcribe",
        "--model",
        &model_str,
        "--input",
        &wav_str,
        "--json",
        "--timestamps",
    ])
    .current_dir(binary_dir);

    if !language.is_empty() {
        cmd.args(["--lang", language]);
    }

    if let Some(device) = device.filter(|d| !d.is_empty()) {
        cmd.env("PARAKEET_DEVICE", device);
    }

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let out = cmd
        .output()
        .map_err(|e| format!("Failed to spawn parakeet-cli: {e}"))?;

    if out.status.success() {
        return Ok(ParakeetOutput {
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        });
    }

    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    Err(format!(
        "parakeet-cli failed ({}).\nstderr: {stderr}\nstdout: {stdout}",
        out.status
    )
    .into())
}

pub fn smoke_test(app: &AppHandle) -> CmdResult<()> {
    let binary = resolve_binary(app)?;
    let mut cmd = Command::new(&binary);
    parakeet_runtime::configure_command_environment(&mut cmd, &binary);
    cmd.current_dir(binary.parent().unwrap_or(Path::new(".")));
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let out = cmd
        .output()
        .map_err(|e| format!("Failed to run parakeet-cli smoke test: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    if out.status.success()
        || stdout.contains("parakeet-cli transcribe")
        || stderr.contains("parakeet-cli transcribe")
    {
        Ok(())
    } else {
        Err(format!(
            "parakeet-cli smoke test failed: {}\nstderr: {}\nstdout: {}",
            out.status, stderr, stdout
        )
        .into())
    }
}

fn scan_dir_for_parakeet(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut subdirs: Vec<PathBuf> = Vec::new();
    let mut candidate: Option<PathBuf> = None;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let lower = name.to_lowercase();
            let is_parakeet = PARAKEET_EXE_NAMES
                .iter()
                .any(|known| lower == known.to_lowercase());
            if is_parakeet {
                candidate.get_or_insert(path);
            }
        }
    }

    for sub in subdirs {
        if let Some(found) = scan_dir_for_parakeet(&sub) {
            return Some(found);
        }
    }

    candidate
}

fn which_in_path(bin: &str) -> bool {
    std::env::var_os("PATH")
        .map(|path_var| std::env::split_paths(&path_var).any(|dir| dir.join(bin).exists()))
        .unwrap_or(false)
}
