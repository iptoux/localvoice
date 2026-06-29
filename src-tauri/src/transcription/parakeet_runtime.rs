use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

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

pub fn verify_worker_health(binary: &Path) -> Result<(), String> {
    let mut command = Command::new(binary);
    configure_command_environment(&mut command, binary);
    command
        .arg("--health")
        .current_dir(binary.parent().unwrap_or_else(|| Path::new(".")));
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);

    let output = output_with_suppressed_dialogs(&mut command)
        .map_err(|e| format!("Failed to start Parakeet streaming worker health check: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if output.status.success()
        && (stdout.contains("parakeet.cpp streaming worker")
            || stderr.contains("parakeet.cpp streaming worker"))
    {
        return Ok(());
    }

    Err(format!(
        "Parakeet streaming worker health check failed: {}\nstderr: {}\nstdout: {}",
        output.status, stderr, stdout
    ))
}

#[cfg(target_os = "windows")]
fn output_with_suppressed_dialogs(command: &mut Command) -> io::Result<Output> {
    use windows_sys::Win32::System::Diagnostics::Debug::{
        SetErrorMode, SEM_FAILCRITICALERRORS, SEM_NOGPFAULTERRORBOX, SEM_NOOPENFILEERRORBOX,
    };

    unsafe {
        let previous = SetErrorMode(SEM_FAILCRITICALERRORS);
        let suppressed =
            previous | SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX | SEM_NOOPENFILEERRORBOX;
        SetErrorMode(suppressed);
        let result = command.output();
        SetErrorMode(previous);
        result
    }
}

#[cfg(not(target_os = "windows"))]
fn output_with_suppressed_dialogs(command: &mut Command) -> io::Result<Output> {
    command.output()
}

fn candidate_runtime_dirs(binary: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(binary_dir) = binary.parent() {
        dirs.push(binary_dir.to_path_buf());
        push_runtime_resource_dirs(&mut dirs, binary_dir);
        if let Some(parent) = binary_dir.parent() {
            push_runtime_resource_dirs(&mut dirs, parent);
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            push_runtime_resource_dirs(&mut dirs, exe_dir);
        }
    }

    dirs.push(Path::new(env!("CARGO_MANIFEST_DIR")).join("parakeet-runtime"));

    let mut seen = HashSet::new();
    dirs.into_iter()
        .filter(|dir| dir.exists())
        .filter(|dir| seen.insert(dir.clone()))
        .collect()
}

fn push_runtime_resource_dirs(dirs: &mut Vec<PathBuf>, base: &Path) {
    dirs.push(base.join("parakeet-runtime"));
    dirs.push(base.join("resources").join("parakeet-runtime"));
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

    #[test]
    fn includes_installed_resources_runtime_dir_when_present() {
        let root = std::env::temp_dir().join(format!(
            "localvoice-parakeet-runtime-test-{}",
            std::process::id()
        ));
        let runtime = root.join("resources").join("parakeet-runtime");
        std::fs::create_dir_all(&runtime).unwrap();

        let binary = root.join("parakeet-stream-worker.exe");
        let dirs = candidate_runtime_dirs(&binary);

        assert!(dirs.iter().any(|candidate| candidate == &runtime));

        let _ = std::fs::remove_dir_all(root);
    }
}
