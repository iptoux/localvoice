use crate::errors::{AppError, CmdResult};
use tauri::{AppHandle, Manager, WebviewWindowBuilder, WebviewUrl};

/// Shows the pill window if hidden.
#[tauri::command]
pub fn show_pill(app: AppHandle) -> CmdResult<()> {
    if let Some(w) = app.get_webview_window("pill") {
        w.show().map_err(|e| e.to_string().into())
    } else {
        Err("Pill window not found".into())
    }
}

/// Hides the pill window.
#[tauri::command]
pub fn hide_pill(app: AppHandle) -> CmdResult<()> {
    if let Some(w) = app.get_webview_window("pill") {
        w.hide().map_err(|e| e.to_string().into())
    } else {
        Err("Pill window not found".into())
    }
}

/// Opens the main window, creating it if it does not yet exist.
#[tauri::command]
pub fn open_main_window(app: AppHandle) -> CmdResult<()> {
    if let Some(w) = app.get_webview_window("main") {
        w.show().map_err(|e| AppError(e.to_string()))?;
        w.set_focus().map_err(|e| AppError(e.to_string()))?;
    } else {
        WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
            .title("LocalVoice")
            .inner_size(1100.0, 720.0)
            .min_inner_size(800.0, 500.0)
            .build()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Moves the pill window to the given screen coordinates.
#[tauri::command]
pub fn set_pill_position(x: i32, y: i32, app: AppHandle) -> CmdResult<()> {
    if let Some(w) = app.get_webview_window("pill") {
        w.set_position(tauri::PhysicalPosition::new(x, y))
            .map_err(|e| e.to_string().into())
    } else {
        Err("Pill window not found".into())
    }
}
