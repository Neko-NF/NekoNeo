#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod commands;
mod errors;
mod models;
mod services;

use app_state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::config::config_get_all,
            commands::config::config_set,
            commands::service::service_start,
            commands::service::service_stop,
            commands::service::service_status,
            commands::service::service_last_result,
            commands::system::system_get_metrics,
            commands::system::system_health_check,
            commands::system::system_get_fonts,
            commands::privacy::privacy_get_windows,
            commands::update::update_check,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
