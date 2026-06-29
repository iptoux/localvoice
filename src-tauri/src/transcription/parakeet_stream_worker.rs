use std::path::{Path, PathBuf};

use tauri::{AppHandle, Manager};

use crate::errors::CmdResult;

#[cfg(target_os = "windows")]
const WORKER_EXE_NAMES: &[&str] = &["parakeet-stream-worker.exe"];
#[cfg(not(target_os = "windows"))]
const WORKER_EXE_NAMES: &[&str] = &["parakeet-stream-worker"];

pub fn resolve_binary(app: &AppHandle) -> CmdResult<PathBuf> {
    let mut searched: Vec<String> = Vec::new();

    if let Ok(env_path) = std::env::var("PARAKEET_STREAM_WORKER_PATH") {
        let path = PathBuf::from(&env_path);
        if path.exists() {
            return Ok(path);
        }
        return Err(format!(
            "PARAKEET_STREAM_WORKER_PATH is set to '{env_path}' but the file does not exist."
        )
        .into());
    }

    if let Ok(exe) = std::env::current_exe() {
        let exe_dir = exe.parent().unwrap_or(Path::new("."));
        for name in WORKER_EXE_NAMES {
            let path = exe_dir.join(name);
            searched.push(path.display().to_string());
            if path.exists() {
                return Ok(path);
            }
        }
    }

    let manifest_binaries = Path::new(env!("CARGO_MANIFEST_DIR")).join("binaries");
    for name in WORKER_EXE_NAMES {
        let path = manifest_binaries.join(name);
        searched.push(path.display().to_string());
        if path.exists() {
            return Ok(path);
        }
    }
    if let Some(found) = scan_dir_for_worker(&manifest_binaries) {
        return Ok(found);
    }

    if let Ok(resource_dir) = app.path().resource_dir() {
        for name in WORKER_EXE_NAMES {
            let path = resource_dir.join(name);
            searched.push(path.display().to_string());
            if path.exists() {
                return Ok(path);
            }
        }

        let binaries_dir = resource_dir.join("binaries");
        for name in WORKER_EXE_NAMES {
            let path = binaries_dir.join(name);
            searched.push(path.display().to_string());
            if path.exists() {
                return Ok(path);
            }
        }
        if let Some(found) = scan_dir_for_worker(&binaries_dir) {
            return Ok(found);
        }
    }

    for name in WORKER_EXE_NAMES {
        if which_in_path(name) {
            return Ok(PathBuf::from(name));
        }
    }

    Err(format!(
        "parakeet-stream-worker binary not found.\nSearched:\n{}\nOptions:\n  - Run the setup-parakeet-cpp action or local bootstrap\n  - Set PARAKEET_STREAM_WORKER_PATH=/full/path/to/parakeet-stream-worker\n  - Use non-streaming Parakeet.cpp transcription as fallback",
        searched
            .iter()
            .map(|p| format!("  - {p}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
    .into())
}

fn scan_dir_for_worker(dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut subdirs: Vec<PathBuf> = Vec::new();
    let mut candidate: Option<PathBuf> = None;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path);
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let lower = name.to_lowercase();
            let matches = WORKER_EXE_NAMES
                .iter()
                .any(|known| lower == known.to_lowercase());
            if matches {
                candidate.get_or_insert(path);
            }
        }
    }

    for subdir in subdirs {
        if let Some(found) = scan_dir_for_worker(&subdir) {
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
