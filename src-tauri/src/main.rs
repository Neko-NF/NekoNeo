#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, thread, time::Duration};

mod app_state;
mod commands;
mod errors;
mod models;
mod services;

use app_state::AppState;
use models::config::{CloseAction, UpdateChannel};
use services::{autostart::AutostartService, update_service::UpdateService};
use tauri::{
    menu::MenuBuilder,
    tray::TrayIconBuilder,
    Emitter, Manager, WindowEvent,
};

fn main() {
    apply_startup_delay_from_args();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_main_window(app);
        }))
        .manage(AppState::new())
        .setup(|app| {
            let config = tauri::async_runtime::block_on(async {
                app.state::<AppState>().config.lock().await.clone()
            });
            AutostartService::sync(&config)?;
            setup_tray(app)?;

            if should_start_minimized() {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.minimize();
                    let _ = window.hide();
                }
            }

            // ── Update system startup ───────────────────────────────
            schedule_update_checks(app.handle());

            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "tray_show" => {
                let _ = show_main_window(app);
            }
            "tray_hide" => {
                let _ = hide_main_window(app);
            }
            "tray_exit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|app, event| {
            if event.id().as_ref() == "main-tray" {
                let _ = show_main_window(app);
            }
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let app = window.app_handle();
                let config = tauri::async_runtime::block_on(async {
                    app.state::<AppState>().config.lock().await.clone()
                });

                match config.close_action {
                    CloseAction::Exit => {
                        app.exit(0);
                    }
                    CloseAction::Minimize => {
                        let _ = window.minimize();
                        let _ = window.hide();
                    }
                    CloseAction::Ask => {
                        let _ = app.emit(
                            "app:close-requested",
                            serde_json::json!({
                                "source": "window-close",
                                "suggestedAction": "minimize",
                            }),
                        );
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::app_resolve_close_request,
            commands::auth::auth_register,
            commands::auth::auth_login,
            commands::auth::auth_get_me,
            commands::auth::auth_generate_device_key,
            commands::config::config_get_all,
            commands::config::config_set,
            commands::config::config_validate_device_key,
            commands::config::config_test_connectivity,
            commands::config::config_sync_device_metadata,
            commands::screenshot::screenshot_capture_now,
            commands::screenshot::screenshot_get_latest,
            commands::service::service_start,
            commands::service::service_stop,
            commands::service::service_status,
            commands::service::service_last_result,
            commands::system::system_get_metrics,
            commands::system::system_get_device_profile,
            commands::system::system_health_check,
            commands::system::system_get_fonts,
            commands::system::system_get_media_info,
            commands::system::system_get_device_fingerprint,
            commands::system::system_get_process_icon,
            commands::privacy::privacy_get_windows,
            commands::privacy::privacy_open_picker,
            commands::privacy::privacy_get_cursor_pos,
            commands::privacy::privacy_close_picker,
            commands::update::update_check,
            commands::update::update_download,
            commands::update::update_install,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let menu = MenuBuilder::new(app)
        .text("tray_show", "Show NekoNeo")
        .text("tray_hide", "Hide to tray")
        .separator()
        .text("tray_exit", "Exit")
        .build()?;

    let mut tray = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .tooltip("NekoNeo")
        .show_menu_on_left_click(false);

    if let Some(icon) = app.default_window_icon().cloned() {
        tray = tray.icon(icon);
    }

    tray.build(app)?;
    Ok(())
}

fn show_main_window(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }

    Ok(())
}

fn hide_main_window(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.minimize();
        let _ = window.hide();
    }

    Ok(())
}

fn apply_startup_delay_from_args() {
    if let Some(delay_ms) = env::args().find_map(parse_startup_delay_arg) {
        if delay_ms > 0 {
            thread::sleep(Duration::from_millis(delay_ms));
        }
    }
}

fn should_start_minimized() -> bool {
    env::args().any(|arg| arg == "--minimized")
}

fn parse_startup_delay_arg(arg: String) -> Option<u64> {
    arg.strip_prefix("--startup-delay-ms=")
        .and_then(|value| value.parse::<u64>().ok())
}

fn schedule_update_checks(app: &tauri::AppHandle) {
    let handle = app.clone();

    // Check for pending install 5s after startup
    let h = handle.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        if UpdateService::has_pending_install() {
            let _ = h.emit(
                "update:available",
                serde_json::json!({
                    "version": "",
                    "mandatory": false,
                    "channel": "",
                    "releaseNotes": "已下载的更新等待安装。",
                    "downloaded": true,
                }),
            );
        }
    });

    // Check for updates 15s after startup, then every 30min
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        loop {
            let state = handle.state::<AppState>();
            let config = state.config.lock().await.clone();
            drop(state);

            if config.auto_check_update {
                let channel = match config.update_channel {
                    UpdateChannel::Beta => "beta",
                    UpdateChannel::Stable => "stable",
                };
                let skipped = if config.skipped_version.is_empty() {
                    None
                } else {
                    Some(config.skipped_version.as_str())
                };
                match UpdateService::check(channel, skipped)
                .await
                {
                    Ok(Some(info)) => {
                        let _ = handle.emit("update:available", &info);

                        if info.mandatory || config.auto_download {
                            if let Err(e) = UpdateService::download(&handle, channel).await {
                                let _ = handle.emit(
                                    "update:error",
                                    serde_json::json!({ "message": e.to_string() }),
                                );
                            } else if info.mandatory {
                                let _ = UpdateService::install(&handle);
                            }
                        }
                    }
                    Ok(None) => {}
                    Err(_) => { /* silently retry next cycle */ }
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(1800)).await; // 30 min
        }
    });
}
