mod audio;
mod commands;
mod db;
mod dictionary;
mod errors;
mod history;
mod models;
mod os;
mod postprocess;
mod state;
mod stats;
mod transcription;

use commands::{dictionary as cmd_dictionary, history as cmd_history, models as cmd_models, recording, settings, stats as cmd_stats, window};
use commands::transcription as cmd_transcription;
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            // The with_handler callback is the single dispatch point for all
            // registered global shortcuts. os::hotkeys::handle routes by state.
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    os::hotkeys::handle(app, shortcut, event);
                })
                .build(),
        )
        .setup(|app| {
            // Open / create SQLite database and run migrations.
            let db = db::open(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;

            // Register shared state.
            app.manage(AppState::new(db));

            // Build system tray.
            os::tray::setup(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

            // Register the global recording shortcut (reads shortcut from DB settings).
            os::hotkeys::setup(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;

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
            // Recording
            recording::start_recording,
            recording::stop_recording,
            recording::cancel_recording,
            recording::get_recording_state,
            recording::list_input_devices,
            // Transcription
            cmd_transcription::transcribe_last_recording,
            cmd_transcription::get_last_transcription,
            // History
            cmd_history::list_sessions,
            cmd_history::get_session,
            cmd_history::delete_session,
            cmd_history::export_sessions,
            // Stats
            cmd_stats::get_dashboard_stats,
            cmd_stats::get_usage_timeseries,
            // Models
            cmd_models::list_available_models,
            cmd_models::download_model,
            cmd_models::delete_model,
            cmd_models::set_default_model,
            // Dictionary
            cmd_dictionary::list_dictionary_entries,
            cmd_dictionary::create_dictionary_entry,
            cmd_dictionary::update_dictionary_entry,
            cmd_dictionary::delete_dictionary_entry,
            cmd_dictionary::list_correction_rules,
            cmd_dictionary::create_correction_rule,
            cmd_dictionary::update_correction_rule,
            cmd_dictionary::delete_correction_rule,
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
