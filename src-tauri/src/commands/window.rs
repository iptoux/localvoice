use crate::errors::{AppError, CmdResult};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

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
        let win = WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
            .title("LocalVoice")
            .inner_size(1100.0, 720.0)
            .min_inner_size(800.0, 500.0)
            .build()
            .map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| AppError(e.to_string()))?;
    }
    Ok(())
}

/// Expands the pill window to show the expanded view.
#[tauri::command]
pub fn expand_pill(app: AppHandle) -> CmdResult<()> {
    if let Some(w) = app.get_webview_window("pill") {
        w.set_max_size(Some(tauri::LogicalSize::new(220.0, 280.0)))
            .map_err(|e| e.to_string())?;
        w.set_size(tauri::LogicalSize::new(220.0, 280.0))
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Pill window not found".into())
    }
}

/// Collapses the pill window back to compact mode.
#[tauri::command]
pub fn collapse_pill(app: AppHandle) -> CmdResult<()> {
    if let Some(w) = app.get_webview_window("pill") {
        // Set size first, then constrain — avoids max_size blocking the resize on Windows.
        w.set_size(tauri::LogicalSize::new(220.0, 70.0))
            .map_err(|e| e.to_string())?;
        w.set_max_size(Some(tauri::LogicalSize::new(220.0, 70.0)))
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Pill window not found".into())
    }
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
