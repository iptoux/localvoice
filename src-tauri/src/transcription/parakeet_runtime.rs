use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn configure_command_environment(command: &mut Command, binary: &Path) {
    let runtime_dirs = candidate_runtime_dirs(binary);
    if runtime_dirs.is_empty() {
        return;
    }

    #[cfg(target_os = "windows")]
    prepend_path_env(command, "PATH", &runtime_dirs);

    #[cfg(target_os = "linux")]
    prepend_path_env(command, "LD_LIBRARY_PATH", &runtime_dirs);

    #[cfg(target_os = "macos")]
    {
        prepend_path_env(command, "DYLD_LIBRARY_PATH", &runtime_dirs);
        prepend_path_env(command, "DYLD_FALLBACK_LIBRARY_PATH", &runtime_dirs);
    }
}

fn candidate_runtime_dirs(binary: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(binary_dir) = binary.parent() {
        dirs.push(binary_dir.to_path_buf());
        dirs.push(binary_dir.join("parakeet-runtime"));
        if let Some(parent) = binary_dir.parent() {
            dirs.push(parent.join("parakeet-runtime"));
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            dirs.push(exe_dir.join("parakeet-runtime"));
        }
    }

    dirs.push(Path::new(env!("CARGO_MANIFEST_DIR")).join("parakeet-runtime"));

    let mut seen = HashSet::new();
    dirs.into_iter()
        .filter(|dir| dir.exists())
        .filter(|dir| seen.insert(dir.clone()))
        .collect()
}

fn prepend_path_env(command: &mut Command, key: &str, dirs: &[PathBuf]) {
    let separator = if cfg!(target_os = "windows") {
        ";"
    } else {
        ":"
    };

    let mut value = dirs
        .iter()
        .map(|dir| dir.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(separator);

    if let Some(existing) = std::env::var_os(key) {
        if !value.is_empty() {
            value.push_str(separator);
        }
        value.push_str(&existing.to_string_lossy());
    }

    command.env(key, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_manifest_runtime_dir_when_present() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("parakeet-runtime");
        let _ = std::fs::create_dir_all(&dir);

        let dirs = candidate_runtime_dirs(Path::new("missing/parakeet-stream-worker"));

        assert!(dirs.iter().any(|candidate| candidate == &dir));
    }
}
