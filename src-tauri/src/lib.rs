mod audio;
mod commands;
mod db;
mod errors;
mod models;
mod os;
mod postprocess;
mod state;
mod transcription;

use commands::{settings, window};
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Open / create SQLite database and run migrations.
            let db = db::open(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;

            // Register shared state.
            app.manage(AppState::new(db));

            // Build system tray.
            os::tray::setup(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Settings
            settings::get_settings,
            settings::update_setting,
            settings::reset_settings,
            // Window
            window::show_pill,
            window::hide_pill,
            window::open_main_window,
            window::set_pill_position,
        ])
        // Prevent the app from exiting when the last window is closed —
        // the tray keeps the app alive.
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Hide instead of closing for both windows.
                window.hide().unwrap_or_default();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running LocalVoice");
}
