mod audio;
pub mod benchmark;
mod commands;
mod db;
mod dictionary;
mod errors;
mod history;
mod logging;
mod models;
mod os;
mod postprocess;
mod state;
mod stats;
mod transcription;

use commands::{
    benchmark as cmd_benchmark,
    dictionary as cmd_dictionary, filler_words as cmd_filler_words, history as cmd_history,
    logs as cmd_logs, models as cmd_models, recording, settings, stats as cmd_stats,
    system as cmd_system, transcription as cmd_transcription, window,
};
use db::repositories::settings_repo;
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(
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

            // Read persisted settings before moving the DB into AppState.
            let persisted = settings_repo::get_all(&db).unwrap_or_default();

            // Initialise in-memory log buffer, respecting the persisted setting.
            let logging_enabled = persisted
                .get("logging.enabled")
                .map(|v| v != "false")
                .unwrap_or(true);
            logging::init(logging_enabled);

            // Register shared state.
            app.manage(AppState::new(db));

            // Clean up old audio files (TASK-209).
            if let Ok(data_dir) = app.path().app_data_dir() {
                audio::cleanup::cleanup_old_audio(
                    &app.state::<AppState>().db,
                    &data_dir.join("audio"),
                );
            }

            // Build system tray.
            os::tray::setup(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

            // Register the global recording shortcut.
            os::hotkeys::setup(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;

            // ── Restore pill position ─────────────────────────────────────────
            if let Some(pill) = app.get_webview_window("pill") {
                if let (Some(x), Some(y)) = (
                    persisted
                        .get("ui.pill.position_x")
                        .and_then(|v| v.parse::<i32>().ok()),
                    persisted
                        .get("ui.pill.position_y")
                        .and_then(|v| v.parse::<i32>().ok()),
                ) {
                    let _ = pill.set_position(tauri::PhysicalPosition::new(x, y));
                }
            }

            // ── Restore main window geometry ──────────────────────────────────
            if let Some(main_win) = app.get_webview_window("main") {
                if let (Some(w), Some(h)) = (
                    persisted
                        .get("ui.main_window.width")
                        .and_then(|v| v.parse::<u32>().ok()),
                    persisted
                        .get("ui.main_window.height")
                        .and_then(|v| v.parse::<u32>().ok()),
                ) {
                    let _ = main_win.set_size(tauri::PhysicalSize::new(w, h));
                }
                if let (Some(x), Some(y)) = (
                    persisted
                        .get("ui.main_window.x")
                        .and_then(|v| v.parse::<i32>().ok()),
                    persisted
                        .get("ui.main_window.y")
                        .and_then(|v| v.parse::<i32>().ok()),
                ) {
                    let _ = main_win.set_position(tauri::PhysicalPosition::new(x, y));
                }
            }

            // ── Apply start-hidden and default-mode settings ──────────────────
            let start_hidden = persisted
                .get("app.start_hidden")
                .map(|v| v == "true")
                .unwrap_or(false);
            let default_mode = persisted
                .get("ui.default_mode")
                .cloned()
                .unwrap_or_else(|| "pill".to_string());

            // ── Check if onboarding/main window is needed ────────────────────────
            let state = app.state::<AppState>();
            let needs_onboarding = db::repositories::models_repo::list_installed(&state.db)
                .map(|v| v.is_empty())
                .unwrap_or(true);
            let has_default = {
                let lang = persisted
                    .get("transcription.default_language")
                    .cloned()
                    .unwrap_or_else(|| "auto".to_string());
                if lang == "auto" {
                    db::repositories::models_repo::get_default_path(&state.db, "de")
                        .map(|p| p.is_some())
                        .unwrap_or(false)
                    || db::repositories::models_repo::get_default_path(&state.db, "en")
                        .map(|p| p.is_some())
                        .unwrap_or(false)
                } else {
                    db::repositories::models_repo::get_default_path(&state.db, &lang)
                        .map(|p| p.is_some())
                        .unwrap_or(false)
                }
            };
            let show_main_on_startup = !start_hidden && (needs_onboarding || !has_default);

            if start_hidden {
                // Tray-only mode: hide everything on startup.
                if let Some(pill) = app.get_webview_window("pill") {
                    let _ = pill.hide();
                }
                // Main window starts hidden by default in tauri.conf.json.
                log::info!("Starting hidden (tray-only mode)");
            } else if show_main_on_startup {
                // First run or no default model: show main window for onboarding/settings.
                if let Some(main_win) = app.get_webview_window("main") {
                    let _ = main_win.show();
                    let _ = main_win.set_focus();
                }
                if let Some(pill) = app.get_webview_window("pill") {
                    let _ = pill.hide();
                }
                log::info!("Starting in main window mode (onboarding)");
            } else if default_mode == "main" {
                // Main window mode: show main, hide pill.
                if let Some(main_win) = app.get_webview_window("main") {
                    let _ = main_win.show();
                }
                if let Some(pill) = app.get_webview_window("pill") {
                    let _ = pill.hide();
                }
                log::info!("Starting in main window mode");
            } else {
                // Default pill mode: pill is already visible via tauri.conf.json.
                log::info!("Starting in pill mode");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Benchmark
            cmd_benchmark::run_transcription_benchmark,
            // Settings
            settings::get_settings,
            settings::update_setting,
            settings::reset_settings,
            settings::update_shortcut,
            // Window
            window::show_pill,
            window::hide_pill,
            window::open_main_window,
            window::set_pill_position,
            window::expand_pill,
            window::collapse_pill,
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
            cmd_history::reprocess_session,
            // Stats
            cmd_stats::get_dashboard_stats,
            cmd_stats::get_usage_timeseries,
            cmd_stats::get_language_breakdown,
            cmd_stats::get_correction_stats,
            cmd_stats::get_wpm_trend,
            cmd_stats::get_daily_comparison,
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
            // Ambiguity
            cmd_dictionary::list_ambiguous_terms,
            cmd_dictionary::accept_ambiguity_suggestion,
            cmd_dictionary::dismiss_ambiguity_suggestion,
            // System
            cmd_system::check_first_run,
            cmd_system::has_default_model,
            cmd_system::set_autostart,
            cmd_system::get_autostart,
            // Logs
            cmd_logs::list_logs,
            cmd_logs::export_logs,
            // Filler Words
            cmd_filler_words::list_filler_words,
            cmd_filler_words::add_filler_word,
            cmd_filler_words::delete_filler_word,
            cmd_filler_words::reset_filler_words,
            cmd_filler_words::get_filler_stats,
            cmd_filler_words::get_filler_total_count,
            cmd_logs::clear_logs,
            cmd_logs::set_logging_enabled,
        ])
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Hide instead of closing — tray keeps the app alive.
                    window.hide().unwrap_or_default();
                    api.prevent_close();
                }
                tauri::WindowEvent::Moved(pos) => {
                    let app = window.app_handle();
                    let state = app.state::<AppState>();
                    let x = pos.x.to_string();
                    let y = pos.y.to_string();
                    match window.label() {
                        "pill" => {
                            settings_repo::upsert(&state.db, "ui.pill.position_x", &x).ok();
                            settings_repo::upsert(&state.db, "ui.pill.position_y", &y).ok();
                        }
                        "main" => {
                            settings_repo::upsert(&state.db, "ui.main_window.x", &x).ok();
                            settings_repo::upsert(&state.db, "ui.main_window.y", &y).ok();
                        }
                        _ => {}
                    }
                }
                tauri::WindowEvent::Resized(size) => {
                    if window.label() == "main" {
                        let app = window.app_handle();
                        let state = app.state::<AppState>();
                        settings_repo::upsert(
                            &state.db,
                            "ui.main_window.width",
                            &size.width.to_string(),
                        )
                        .ok();
                        settings_repo::upsert(
                            &state.db,
                            "ui.main_window.height",
                            &size.height.to_string(),
                        )
                        .ok();
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running LocalVoice");
}
