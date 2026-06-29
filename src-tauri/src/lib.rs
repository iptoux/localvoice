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
    benchmark as cmd_benchmark, dictionary as cmd_dictionary, filler_words as cmd_filler_words,
    history as cmd_history, logs as cmd_logs, models as cmd_models, recording, settings,
    stats as cmd_stats, system as cmd_system, transcription as cmd_transcription,
    updater as cmd_updater, window,
};
use db::repositories::settings_repo;
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    os::hotkeys::handle(app, shortcut, event);
                })
                .build(),
        )
        .setup(|app| {
            // Open / create SQLite database and run migrations.
            let db = db::open(app.handle()).map_err(|e| Box::<dyn std::error::Error>::from(e))?;

            // Read persisted settings before moving the DB into AppState.
            let persisted = settings_repo::get_all(&db).unwrap_or_default();

            // Initialise SQLite-backed logger, respecting the persisted setting (TASK-247).
            let logging_enabled = persisted
                .get("logging.enabled")
                .map(|v| v != "false")
                .unwrap_or(true);
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;
            logging::init(logging_enabled, db.clone(), app_data_dir);

            // Register shared state.
            app.manage(AppState::new(db));
            app.manage(cmd_updater::PendingUpdate::default());

            // Build system tray (critical — needed before window shows).
            os::tray::setup(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

            // Register the global recording shortcut (critical — user expects it immediately).
            os::hotkeys::setup(app.handle()).map_err(|e| Box::<dyn std::error::Error>::from(e))?;

            // Defer non-critical work (audio cleanup) to after the window is rendered
            // so it does not delay the initial display.  TASK-245.
            {
                let db_handle = app.state::<AppState>().db.clone();
                let data_dir = app.path().app_data_dir().ok();
                tauri::async_runtime::spawn(async move {
                    if let Some(dir) = data_dir {
                        audio::cleanup::cleanup_old_audio(&db_handle, &dir.join("audio"));
                    }
                });
            }

            // ── Restore pill position ─────────────────────────────────────────
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
                    if window::is_valid_main_window_size(w, h) {
                        let _ = main_win.set_size(tauri::PhysicalSize::new(w, h));
                    } else {
                        log::warn!(
                            "Ignoring invalid persisted main window size during startup: {w}x{h}"
                        );
                    }
                }
                if let (Some(x), Some(y)) = (
                    persisted
                        .get("ui.main_window.x")
                        .and_then(|v| v.parse::<i32>().ok()),
                    persisted
                        .get("ui.main_window.y")
                        .and_then(|v| v.parse::<i32>().ok()),
                ) {
                    if window::is_valid_main_window_position(x, y) {
                        let _ = main_win.set_position(tauri::PhysicalPosition::new(x, y));
                    } else {
                        log::warn!(
                            "Ignoring invalid persisted main window position during startup: {x},{y}"
                        );
                    }
                }
            }

            // ── Apply start-hidden and default-mode settings ──────────────────
            let start_hidden = persisted
                .get("app.start_hidden")
                .map(|v| v == "true")
                .unwrap_or(false);
            let pill_mode = window::pill_mode_from_settings(&persisted);
            let default_mode = persisted
                .get("ui.default_mode")
                .cloned()
                .unwrap_or_else(|| "main".to_string());

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
                if let Err(e) = window::show_main_window(app.handle(), "startup onboarding") {
                    log::error!("Startup main window open failed: {e}");
                }
                if let Some(pill) = app.get_webview_window("pill") {
                    let _ = pill.hide();
                }
                log::info!("Starting in main window mode (onboarding)");
            } else if default_mode == "main" {
                // Main window mode: show main, hide pill.
                if let Err(e) = window::show_main_window(app.handle(), "startup default main") {
                    log::error!("Startup main window open failed: {e}");
                }
                if let Some(pill) = app.get_webview_window("pill") {
                    let _ = pill.hide();
                }
                log::info!("Starting in main window mode");
            } else if pill_mode == "classic" && default_mode == "pill" {
                let _ = window::show_classic_pill(app.handle());
                log::info!("Starting in classic pill mode");
            } else {
                if let Some(pill) = app.get_webview_window("pill") {
                    let _ = pill.hide();
                }
                log::info!("Starting in tray mode");
            }

            cmd_updater::spawn_startup_check(app.handle().clone());

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
            cmd_transcription::list_transcription_engines,
            cmd_transcription::check_transcription_runtime,
            // Updater
            cmd_updater::check_for_update,
            cmd_updater::get_update_status,
            cmd_updater::install_pending_update,
            // History
            cmd_history::list_sessions,
            cmd_history::get_session,
            cmd_history::delete_session,
            cmd_history::bulk_delete_sessions,
            cmd_history::bulk_export_sessions,
            cmd_history::get_audio_file_path,
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
                    if window.label() == "main"
                        && crate::commands::window::is_classic_pill_mode(window.app_handle())
                    {
                        let _ = crate::commands::window::show_classic_pill(window.app_handle());
                    }
                    api.prevent_close();
                }
                tauri::WindowEvent::Moved(pos) => {
                    let app = window.app_handle();
                    let state = app.state::<AppState>();
                    let x = pos.x.to_string();
                    let y = pos.y.to_string();
                    match window.label() {
                        "pill" if crate::commands::window::is_classic_pill_mode(app) => {
                            settings_repo::upsert(&state.db, "ui.pill.position_x", &x).ok();
                            settings_repo::upsert(&state.db, "ui.pill.position_y", &y).ok();
                        }
                        "main" => {
                            if crate::commands::window::is_valid_main_window_position(pos.x, pos.y)
                            {
                                settings_repo::upsert(&state.db, "ui.main_window.x", &x).ok();
                                settings_repo::upsert(&state.db, "ui.main_window.y", &y).ok();
                            } else {
                                log::warn!(
                                    "Ignoring invalid main window move event position: {},{}",
                                    pos.x,
                                    pos.y
                                );
                            }
                        }
                        _ => {}
                    }
                }
                tauri::WindowEvent::Resized(size) => {
                    if window.label() == "main" {
                        let app = window.app_handle();
                        let state = app.state::<AppState>();
                        if crate::commands::window::is_valid_main_window_size(
                            size.width,
                            size.height,
                        ) {
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
                        } else {
                            log::warn!(
                                "Ignoring invalid main window resize event size: {}x{}",
                                size.width,
                                size.height
                            );
                        }
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running LocalVoice");
}
