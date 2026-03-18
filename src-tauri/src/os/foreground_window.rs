/// Returns the title of the currently focused window, or `None` if detection fails.
#[cfg(target_os = "windows")]
pub fn get_foreground_window_title() -> Option<String> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return None;
        }

        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
        if len <= 0 {
            return None;
        }

        Some(String::from_utf16_lossy(&buf[..len as usize]))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_foreground_window_title() -> Option<String> {
    None
}
