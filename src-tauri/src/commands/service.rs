use tauri::{AppHandle, State};

use crate::{
    app_state::AppState,
    errors::AppError,
    models::service::{ServiceStatus, TickResult},
    services::config_store::ConfigStore,
};

#[tauri::command]
pub async fn service_start(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ServiceStatus, AppError> {
    let config_snapshot = state.config.lock().await.clone();
    let active_server_url = config_snapshot.active_server_url().trim().to_string();
    let device_key = config_snapshot.device_key.trim().to_string();

    if active_server_url.is_empty() || active_server_url.contains("api.example.com") {
        return Err(AppError::MissingServerConfig("请先配置正确的上报服务器地址".into()));
    }

    if device_key.is_empty() {
        return Err(AppError::MissingDeviceKey("请先配置设备密钥".into()));
    }

    let validation = {
        let api_client = state.api_client.lock().await.clone();
        match api_client
            .validate_device_key(Some(&active_server_url), Some(&device_key))
            .await
        {
            Ok(v) => Some(v),
            Err(AppError::TakeoverRequired(_)) => {
                // User already confirmed via Settings; allow start.
                // The server will finalise takeover during reporting.
                None
            }
            Err(e) => return Err(e),
        }
    };

    if let Some(ref validation) = validation {
        if let Some(device_id) = validation.device_id {
            let mut config = state.config.lock().await;
            if config.device_id != Some(device_id) {
                config.device_id = Some(device_id);
                ConfigStore::save(&config)
                    .map_err(|error| AppError::ConfigError(error.to_string()))?;
            }
        }
    }

    Ok(state
        .reporter
        .start(app, state.config.clone(), state.api_client.clone())
        .await)
}

#[tauri::command]
pub async fn service_stop(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ServiceStatus, AppError> {
    Ok(state.reporter.stop(&app).await)
}

#[tauri::command]
pub async fn service_status(state: State<'_, AppState>) -> Result<ServiceStatus, AppError> {
    Ok(state.reporter.status().await)
}

#[tauri::command]
pub async fn service_last_result(
    state: State<'_, AppState>,
) -> Result<Option<TickResult>, AppError> {
    Ok(state.reporter.last_result().await)
}
