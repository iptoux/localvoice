/// Returns the path to the current executable.
fn exe_path() -> Result<String, String> {
    std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

/// Enables or disables launching LocalVoice on OS login.
///
/// On Windows this writes / removes a registry value under
/// `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`.
/// On other platforms the operation is a no-op (returns Ok).
#[allow(unused_variables)]
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
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
    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

/// Returns `true` if the LocalVoice autostart entry is present.
pub fn get_autostart() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_CURRENT_USER;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(run_key) =
            hkcu.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")
        {
            run_key.get_value::<String, _>("LocalVoice").is_ok()
        } else {
            false
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}
