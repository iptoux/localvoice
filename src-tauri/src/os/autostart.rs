/// Returns the path to the current executable.
fn exe_path() -> Result<String, String> {
    std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

/// Enables or disables launching LocalVoice on OS login.
///
/// - Windows: writes/removes a registry value under `HKCU\...\Run`
/// - macOS:   writes/removes a launchd plist in `~/Library/LaunchAgents/`
/// - Linux:   writes/removes an XDG autostart `.desktop` file in `~/.config/autostart/`
///
/// In debug builds this always returns an error to prevent registering the dev
/// binary (which requires a running Vite dev server).
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    #[cfg(debug_assertions)]
    {
        let _ = enabled;
        return Err(
            "Autostart cannot be enabled in development builds. Build a release binary first."
                .to_string(),
        );
    }

    #[allow(unreachable_code)]
    set_autostart_impl(enabled)
}

/// Returns `true` if the LocalVoice autostart entry is present.
/// Always returns `false` in debug builds.
pub fn get_autostart() -> bool {
    #[cfg(debug_assertions)]
    {
        return false;
    }

    #[allow(unreachable_code)]
    get_autostart_impl()
}

// ── Platform dispatch ─────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn set_autostart_impl(enabled: bool) -> Result<(), String> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu
        .open_subkey_with_flags(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            KEY_SET_VALUE,
        )
        .map_err(|e| format!("Cannot open Run registry key: {e}"))?;

    if enabled {
        let path = exe_path()?;
        run_key
            .set_value("LocalVoice", &path)
            .map_err(|e| format!("Cannot write registry value: {e}"))?;
    } else {
        // Deleting a non-existent value is silently ignored.
        run_key.delete_value("LocalVoice").unwrap_or(());
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn get_autostart_impl() -> bool {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run_key) = hkcu.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run") {
        run_key.get_value::<String, _>("LocalVoice").is_ok()
    } else {
        false
    }
}

// ── macOS — launchd plist ─────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
fn macos_plist_path() -> Result<std::path::PathBuf, String> {
    let home =
        std::env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?;
    Ok(std::path::Path::new(&home).join("Library/LaunchAgents/com.localvoice.app.plist"))
}

#[cfg(target_os = "macos")]
fn set_autostart_impl(enabled: bool) -> Result<(), String> {
    let plist_path = macos_plist_path()?;

    if enabled {
        let exe = exe_path()?;
        if let Some(plist_dir) = plist_path.parent() {
            std::fs::create_dir_all(plist_dir)
                .map_err(|e| format!("Cannot create LaunchAgents directory: {e}"))?;
        }
        let contents = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.localvoice.app</string>
    <key>ProgramArguments</key>
    <array>
        <string>{exe}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
    <key>StandardOutPath</key>
    <string>/tmp/localvoice.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/localvoice.err</string>
</dict>
</plist>
"#
        );
        std::fs::write(&plist_path, contents)
            .map_err(|e| format!("Cannot write launchd plist: {e}"))?;
    } else if plist_path.exists() {
        std::fs::remove_file(&plist_path)
            .map_err(|e| format!("Cannot remove launchd plist: {e}"))?;
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn get_autostart_impl() -> bool {
    macos_plist_path().map(|p| p.exists()).unwrap_or(false)
}

// ── Linux — XDG autostart .desktop file ──────────────────────────────────────

#[cfg(target_os = "linux")]
fn linux_desktop_path() -> Result<std::path::PathBuf, String> {
    let config_home = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{home}/.config")
    });
    Ok(std::path::Path::new(&config_home).join("autostart/localvoice.desktop"))
}

#[cfg(target_os = "linux")]
fn set_autostart_impl(enabled: bool) -> Result<(), String> {
    let desktop_path = linux_desktop_path()?;

    if enabled {
        let exe = exe_path()?;
        if let Some(autostart_dir) = desktop_path.parent() {
            std::fs::create_dir_all(autostart_dir)
                .map_err(|e| format!("Cannot create autostart directory: {e}"))?;
        }
        let contents = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=LocalVoice\n\
             Comment=Offline-first desktop voice dictation\n\
             Exec={exe}\n\
             Hidden=false\n\
             NoDisplay=false\n\
             X-GNOME-Autostart-enabled=true\n"
        );
        std::fs::write(&desktop_path, contents)
            .map_err(|e| format!("Cannot write .desktop file: {e}"))?;
    } else if desktop_path.exists() {
        std::fs::remove_file(&desktop_path)
            .map_err(|e| format!("Cannot remove .desktop file: {e}"))?;
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn get_autostart_impl() -> bool {
    linux_desktop_path().map(|p| p.exists()).unwrap_or(false)
}

// ── Fallback for other Unix-like platforms ────────────────────────────────────

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn set_autostart_impl(_enabled: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn get_autostart_impl() -> bool {
    false
}
