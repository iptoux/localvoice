use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::commands::window;

/// Builds and registers the system tray icon with its context menu.
pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let open_app = MenuItem::with_id(app, "open_app", "Open App", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open_app, &settings, &quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open_app" => {
                let _ = window::open_main_window(app.clone());
            }
            "settings" => {
                let _ = window::open_main_window(app.clone());
                // TODO MS-10: navigate to settings page after open
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
