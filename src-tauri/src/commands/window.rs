use std::collections::HashMap;
use std::fmt::Display;

use crate::db::repositories::settings_repo;
use crate::errors::{AppError, CmdResult};
use crate::state::recording_state::RecordingState;
use crate::state::AppState;
use tauri::{
    AppHandle, LogicalSize, Manager, PhysicalPosition, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};

const PILL_MODE_KEY: &str = "ui.pill.mode";
const PILL_MODE_OVERLAY: &str = "overlay";
const PILL_MODE_CLASSIC: &str = "classic";
const CLASSIC_WIDTH: f64 = 220.0;
const CLASSIC_HEIGHT: f64 = 70.0;
const CLASSIC_EXPANDED_HEIGHT: f64 = 280.0;
const OVERLAY_WIDTH: f64 = 180.0;
const OVERLAY_HEIGHT: f64 = 56.0;
const OVERLAY_BOTTOM_MARGIN: f64 = 96.0;
pub const MAIN_DEFAULT_WIDTH: f64 = 1100.0;
pub const MAIN_DEFAULT_HEIGHT: f64 = 720.0;
pub const MAIN_MIN_WIDTH: u32 = 800;
pub const MAIN_MIN_HEIGHT: u32 = 500;
const WINDOWS_HIDDEN_SENTINEL: i32 = -30000;

pub fn pill_mode_from_settings(settings: &HashMap<String, String>) -> &'static str {
    match settings.get(PILL_MODE_KEY).map(String::as_str) {
        Some(PILL_MODE_CLASSIC) => PILL_MODE_CLASSIC,
        _ => PILL_MODE_OVERLAY,
    }
}

pub fn is_classic_pill_mode(app: &AppHandle) -> bool {
    let state = app.state::<AppState>();
    let settings = settings_repo::get_all(&state.db).unwrap_or_default();
    pill_mode_from_settings(&settings) == PILL_MODE_CLASSIC
}

fn pill_window(app: &AppHandle) -> CmdResult<WebviewWindow> {
    app.get_webview_window("pill")
        .ok_or_else(|| AppError("Pill window not found".to_string()))
}

/// Shows the pill window if hidden.
#[tauri::command]
pub fn show_pill(app: AppHandle) -> CmdResult<()> {
    if is_classic_pill_mode(&app) {
        show_classic_pill(&app)
    } else {
        let state = app.state::<AppState>();
        if *state.recording_state.lock().unwrap() == RecordingState::Listening {
            show_recording_overlay_pill(&app)
        } else {
            Ok(())
        }
    }
}

/// Hides the pill window.
#[tauri::command]
pub fn hide_pill(app: AppHandle) -> CmdResult<()> {
    hide_pill_window(&app)
}

fn hide_pill_window(app: &AppHandle) -> CmdResult<()> {
    let w = pill_window(app)?;
    w.hide().map_err(|e| e.to_string().into())
}

pub fn hide_pill_if_overlay_mode(app: &AppHandle) {
    if !is_classic_pill_mode(app) {
        let _ = hide_pill_window(app);
    }
}

pub fn show_recording_pill_for_mode(app: &AppHandle) {
    let result = if is_classic_pill_mode(app) {
        show_classic_pill(app)
    } else {
        show_recording_overlay_pill(app)
    };

    if let Err(e) = result {
        log::warn!("Failed to show recording pill: {e}");
    }
}

pub fn show_classic_pill(app: &AppHandle) -> CmdResult<()> {
    let w = pill_window(app)?;
    if let Err(e) = w.set_focusable(true) {
        log::warn!("Failed to make classic pill focusable: {e}");
    }
    configure_pill_size(&w, CLASSIC_WIDTH, CLASSIC_HEIGHT, CLASSIC_EXPANDED_HEIGHT)?;
    w.set_size(LogicalSize::new(CLASSIC_WIDTH, CLASSIC_HEIGHT))
        .map_err(|e| e.to_string())?;
    restore_classic_pill_position(app, &w);
    w.show().map_err(|e| e.to_string().into())
}

pub fn show_recording_overlay_pill(app: &AppHandle) -> CmdResult<()> {
    let w = pill_window(app)?;
    if let Err(e) = w.set_focusable(false) {
        log::warn!("Failed to make recording overlay non-focusable: {e}");
    }
    configure_pill_size(&w, OVERLAY_WIDTH, OVERLAY_HEIGHT, OVERLAY_HEIGHT)?;
    w.set_size(LogicalSize::new(OVERLAY_WIDTH, OVERLAY_HEIGHT))
        .map_err(|e| e.to_string())?;
    position_overlay_bottom_center(&w)?;
    w.show().map_err(|e| e.to_string().into())
}

fn configure_pill_size(
    w: &WebviewWindow,
    width: f64,
    height: f64,
    max_height: f64,
) -> CmdResult<()> {
    w.set_min_size(Some(LogicalSize::new(width, height)))
        .map_err(|e| e.to_string())?;
    w.set_max_size(Some(LogicalSize::new(width, max_height)))
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn restore_classic_pill_position(app: &AppHandle, w: &WebviewWindow) {
    let state = app.state::<AppState>();
    let settings = settings_repo::get_all(&state.db).unwrap_or_default();
    if let (Some(x), Some(y)) = (
        settings
            .get("ui.pill.position_x")
            .and_then(|v| v.parse::<i32>().ok()),
        settings
            .get("ui.pill.position_y")
            .and_then(|v| v.parse::<i32>().ok()),
    ) {
        let _ = w.set_position(PhysicalPosition::new(x, y));
    }
}

fn position_overlay_bottom_center(w: &WebviewWindow) -> CmdResult<()> {
    let monitor = w
        .current_monitor()
        .map_err(|e| e.to_string())?
        .or(w.primary_monitor().map_err(|e| e.to_string())?)
        .ok_or_else(|| AppError("No monitor available for pill positioning".to_string()))?;
    let scale = monitor.scale_factor();
    let window_width = (OVERLAY_WIDTH * scale).round() as u32;
    let window_height = (OVERLAY_HEIGHT * scale).round() as u32;
    let bottom_margin = (OVERLAY_BOTTOM_MARGIN * scale).round() as u32;
    let pos = calculate_bottom_center_position(
        monitor.position().x,
        monitor.position().y,
        monitor.size().width,
        monitor.size().height,
        window_width,
        window_height,
        bottom_margin,
    );
    w.set_position(pos).map_err(|e| e.to_string().into())
}

pub fn calculate_bottom_center_position(
    monitor_x: i32,
    monitor_y: i32,
    monitor_width: u32,
    monitor_height: u32,
    window_width: u32,
    window_height: u32,
    bottom_margin: u32,
) -> PhysicalPosition<i32> {
    let x = monitor_x + ((monitor_width.saturating_sub(window_width)) / 2) as i32;
    let y = monitor_y
        + monitor_height.saturating_sub(window_height.saturating_add(bottom_margin)) as i32;
    PhysicalPosition::new(x, y)
}

/// Opens the main window, creating it if it does not yet exist.
/// On Windows a window that has never been visible can silently fail to show,
/// so we destroy and recreate it as a fallback.
#[tauri::command]
pub fn open_main_window(app: AppHandle) -> CmdResult<()> {
    show_main_window(&app, "command")
}

pub fn show_main_window(app: &AppHandle, source: &str) -> CmdResult<()> {
    log::info!("Main window open requested from {source}");

    if let Some(w) = app.get_webview_window("main") {
        log_main_window_state(&w, "before open");
        if let Err(e) = w.unminimize() {
            log::warn!("Main window unminimize failed: {e}");
        }
        if ensure_main_window_size(&w) {
            center_main_window(&w, "invalid size");
        } else {
            ensure_main_window_on_screen(&w);
        }
        w.show().map_err(|e| main_window_error("show", e))?;
        log_main_window_state(&w, "after show");
        w.set_focus().map_err(|e| main_window_error("focus", e))?;
        log_main_window_state(&w, "after focus");

        if !w
            .is_visible()
            .map_err(|e| main_window_error("read visibility", e))?
        {
            log::warn!("Main window is still hidden after show; recreating it");
            if let Err(e) = w.destroy() {
                log::warn!("Main window destroy before recreate failed: {e}");
            }
            return create_main_window(app);
        }
    } else {
        log::warn!("Main window handle not found; creating a new window");
        return create_main_window(app);
    }
    Ok(())
}

fn create_main_window(app: &AppHandle) -> CmdResult<()> {
    log::info!("Creating main window");
    let win = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("LocalVoice")
        .inner_size(MAIN_DEFAULT_WIDTH, MAIN_DEFAULT_HEIGHT)
        .min_inner_size(f64::from(MAIN_MIN_WIDTH), f64::from(MAIN_MIN_HEIGHT))
        .decorations(true)
        .build()
        .map_err(|e| main_window_error("create", e))?;
    log_main_window_state(&win, "after create");
    win.show()
        .map_err(|e| main_window_error("show created", e))?;
    win.set_focus()
        .map_err(|e| main_window_error("focus created", e))?;
    log_main_window_state(&win, "created visible");
    Ok(())
}

fn main_window_error(operation: &str, error: impl Display) -> AppError {
    let message = format!("Main window {operation} failed: {error}");
    log::error!("{message}");
    AppError(message)
}

fn log_main_window_state(w: &WebviewWindow, context: &str) {
    let position = w
        .outer_position()
        .map(|p| format!("{},{}", p.x, p.y))
        .unwrap_or_else(|e| format!("error:{e}"));
    let size = w
        .outer_size()
        .map(|s| format!("{}x{}", s.width, s.height))
        .unwrap_or_else(|e| format!("error:{e}"));
    let visible = w
        .is_visible()
        .map(|v| v.to_string())
        .unwrap_or_else(|e| format!("error:{e}"));
    let minimized = w
        .is_minimized()
        .map(|v| v.to_string())
        .unwrap_or_else(|e| format!("error:{e}"));

    log::info!(
        "Main window state ({context}): visible={visible}, minimized={minimized}, position={position}, size={size}"
    );
}

pub fn is_valid_main_window_size(width: u32, height: u32) -> bool {
    width >= MAIN_MIN_WIDTH && height >= MAIN_MIN_HEIGHT
}

pub fn is_valid_main_window_position(x: i32, y: i32) -> bool {
    x > WINDOWS_HIDDEN_SENTINEL && y > WINDOWS_HIDDEN_SENTINEL
}

fn ensure_main_window_size(w: &WebviewWindow) -> bool {
    let size = match w.outer_size() {
        Ok(size) => size,
        Err(e) => {
            log::warn!("Main window size could not be read before show: {e}");
            return false;
        }
    };

    if is_valid_main_window_size(size.width, size.height) {
        return false;
    }

    log::warn!(
        "Main window size is invalid before show ({}x{}); resetting to {}x{}",
        size.width,
        size.height,
        MAIN_DEFAULT_WIDTH,
        MAIN_DEFAULT_HEIGHT
    );
    if let Err(e) = w.set_size(LogicalSize::new(MAIN_DEFAULT_WIDTH, MAIN_DEFAULT_HEIGHT)) {
        log::error!("Main window size reset failed: {e}");
        return false;
    }
    true
}

fn ensure_main_window_on_screen(w: &WebviewWindow) {
    let position = match w.outer_position() {
        Ok(position) => position,
        Err(e) => {
            log::warn!("Main window position could not be read before show: {e}");
            return;
        }
    };
    let size = match w.outer_size() {
        Ok(size) => size,
        Err(e) => {
            log::warn!("Main window size could not be read before show: {e}");
            return;
        }
    };
    let monitors = match w.available_monitors() {
        Ok(monitors) => monitors,
        Err(e) => {
            log::warn!("Main window monitors could not be read before show: {e}");
            return;
        }
    };

    if is_valid_main_window_position(position.x, position.y)
        && monitors.iter().any(|monitor| {
            let monitor_position = monitor.position();
            let monitor_size = monitor.size();
            rects_intersect(
                position.x,
                position.y,
                size.width,
                size.height,
                monitor_position.x,
                monitor_position.y,
                monitor_size.width,
                monitor_size.height,
            )
        })
    {
        return;
    }

    log::warn!(
        "Main window is outside all visible monitors at {},{} ({}x{}); centering",
        position.x,
        position.y,
        size.width,
        size.height
    );
    center_main_window(w, "offscreen position");
}

fn center_main_window(w: &WebviewWindow, reason: &str) {
    if let Err(e) = w.center() {
        log::error!("Main window center fallback failed: {e}");
    } else {
        log::info!("Main window centered after {reason}");
    }
}

fn rects_intersect(
    left_a: i32,
    top_a: i32,
    width_a: u32,
    height_a: u32,
    left_b: i32,
    top_b: i32,
    width_b: u32,
    height_b: u32,
) -> bool {
    let right_a = i64::from(left_a) + i64::from(width_a);
    let bottom_a = i64::from(top_a) + i64::from(height_a);
    let right_b = i64::from(left_b) + i64::from(width_b);
    let bottom_b = i64::from(top_b) + i64::from(height_b);

    i64::from(left_a) < right_b
        && right_a > i64::from(left_b)
        && i64::from(top_a) < bottom_b
        && bottom_a > i64::from(top_b)
}

/// Expands the classic pill window to show the expanded view.
#[tauri::command]
pub fn expand_pill(app: AppHandle) -> CmdResult<()> {
    let w = pill_window(&app)?;
    w.set_max_size(Some(LogicalSize::new(
        CLASSIC_WIDTH,
        CLASSIC_EXPANDED_HEIGHT,
    )))
    .map_err(|e| e.to_string())?;
    w.set_size(LogicalSize::new(CLASSIC_WIDTH, CLASSIC_EXPANDED_HEIGHT))
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Collapses the classic pill window back to compact mode.
#[tauri::command]
pub fn collapse_pill(app: AppHandle) -> CmdResult<()> {
    let w = pill_window(&app)?;
    w.set_size(LogicalSize::new(CLASSIC_WIDTH, CLASSIC_HEIGHT))
        .map_err(|e| e.to_string())?;
    w.set_max_size(Some(LogicalSize::new(CLASSIC_WIDTH, CLASSIC_HEIGHT)))
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Moves the pill window to the given screen coordinates.
#[tauri::command]
pub fn set_pill_position(x: i32, y: i32, app: AppHandle) -> CmdResult<()> {
    let w = pill_window(&app)?;
    w.set_position(PhysicalPosition::new(x, y))
        .map_err(|e| e.to_string().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_center_position_uses_monitor_bounds_and_margin() {
        let pos = calculate_bottom_center_position(100, 50, 1920, 1080, 180, 56, 96);

        assert_eq!(pos.x, 970);
        assert_eq!(pos.y, 978);
    }

    #[test]
    fn pill_mode_defaults_to_overlay() {
        let settings = HashMap::new();

        assert_eq!(pill_mode_from_settings(&settings), PILL_MODE_OVERLAY);
    }

    #[test]
    fn pill_mode_accepts_classic_only_when_explicit() {
        let mut settings = HashMap::new();
        settings.insert(PILL_MODE_KEY.to_string(), PILL_MODE_CLASSIC.to_string());

        assert_eq!(pill_mode_from_settings(&settings), PILL_MODE_CLASSIC);
    }

    #[test]
    fn rects_intersect_detects_visible_overlap() {
        assert!(rects_intersect(100, 100, 800, 600, 0, 0, 1920, 1080));
    }

    #[test]
    fn rects_intersect_rejects_offscreen_window() {
        assert!(!rects_intersect(2500, 100, 800, 600, 0, 0, 1920, 1080));
    }

    #[test]
    fn main_window_size_rejects_hidden_windows_shell_size() {
        assert!(!is_valid_main_window_size(16, 39));
    }

    #[test]
    fn main_window_position_rejects_windows_hidden_sentinel() {
        assert!(!is_valid_main_window_position(-32000, -32000));
    }
}
