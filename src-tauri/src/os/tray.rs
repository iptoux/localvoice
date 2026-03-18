use tauri::{
    menu::{Menu, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::commands::{recording::{ start_recording_internal, stop_recording_internal }, window};
use crate::state::recording_state::RecordingState;
use crate::state::AppState;

/// Builds and registers the system tray icon with its context menu.
pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let toggle_recording =
        MenuItemBuilder::with_id("toggle_recording", "Start Recording").build(app)?;
    let sep1 = PredefinedMenuItem::separator(app)?;
    let dashboard = MenuItemBuilder::with_id("dashboard", "Dashboard").build(app)?;
    let history = MenuItemBuilder::with_id("history", "History").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    let menu = Menu::with_items(
        app,
        &[
            &toggle_recording,
            &sep1,
            &dashboard,
            &history,
            &settings,
            &sep2,
            &quit,
        ],
    )?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "toggle_recording" => {
                let state = app.state::<AppState>();
                let current = state.recording_state.lock().unwrap().clone();
                match current {
                    RecordingState::Idle => {
                        if let Err(e) = start_recording_internal(app, &state) {
                            log::error!("Tray: start_recording failed: {e}");
                        }
                    }
                    RecordingState::Listening => {
                        if let Err(e) = stop_recording_internal(app, &state) {
                            log::error!("Tray: stop_recording failed: {e}");
                        }
                    }
                    _ => {}
                }
            }
            "dashboard" => {
                let _ = window::open_main_window(app.clone());
            }
            "history" => {
                let _ = window::open_main_window(app.clone());
            }
            "settings" => {
                let _ = window::open_main_window(app.clone());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(pill) = app.get_webview_window("pill") {
                    if pill.is_visible().unwrap_or(false) {
                        let _ = pill.hide();
                    } else {
                        let _ = pill.show();
                        let _ = pill.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
