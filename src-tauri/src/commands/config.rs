use tauri::State;

use crate::{
    app_state::AppState,
    errors::AppError,
    models::config::AppConfig,
    services::{api_client::ValidateResponse, autostart::AutostartService, config_store::ConfigStore},
};

#[tauri::command]
pub async fn config_get_all(state: State<'_, AppState>) -> Result<AppConfig, AppError> {
    Ok(state.config.lock().await.clone())
}

#[tauri::command]
pub async fn config_set(
    state: State<'_, AppState>,
    key: String,
    value: serde_json::Value,
) -> Result<(), AppError> {
    let mut config = state.config.lock().await;

    let mut config_value = serde_json::to_value(config.clone())
        .map_err(|error| AppError::ConfigError(error.to_string()))?;

    let object = config_value
        .as_object_mut()
        .ok_or_else(|| AppError::ConfigError("config serialization failed".into()))?;

    if !object.contains_key(&key) {
        return Err(AppError::ConfigError(format!("unsupported config key: {key}")));
    }

    object.insert(key, value);

    *config = serde_json::from_value::<AppConfig>(config_value)
        .map_err(|error| AppError::ConfigError(error.to_string()))?;

    ConfigStore::save(&config).map_err(|error| AppError::ConfigError(error.to_string()))?;
    AutostartService::sync(&config)?;

    let mut api_client = state.api_client.lock().await;
    api_client.reconfigure(config.active_server_url(), config.device_key.clone());

    Ok(())
}

#[tauri::command]
pub async fn config_validate_device_key(
    state: State<'_, AppState>,
    key: String,
    server_url: Option<String>,
) -> Result<ValidateResponse, AppError> {
    let fallback_server_url = state.config.lock().await.active_server_url().to_string();
    let api_client = state.api_client.lock().await.clone();

    api_client
        .validate_device_key(
            Some(server_url.as_deref().unwrap_or(&fallback_server_url)),
            Some(&key),
        )
        .await
}

#[tauri::command]
pub async fn config_test_connectivity(
    state: State<'_, AppState>,
    server_url: Option<String>,
    key: Option<String>,
) -> Result<crate::models::system::ConnectivityStatus, AppError> {
    let fallback_server_url = state.config.lock().await.active_server_url().to_string();
    let api_client = state.api_client.lock().await.clone();

    api_client
        .test_connectivity(
            Some(server_url.as_deref().unwrap_or(&fallback_server_url)),
            key.as_deref(),
        )
        .await
}

#[tauri::command]
pub async fn config_sync_device_metadata(
    state: State<'_, AppState>,
) -> Result<crate::models::system::DeviceProfile, AppError> {
    let profile = crate::services::system_info::SystemInfo::get_device_profile().await;
    let api_client = state.api_client.lock().await.clone();
    api_client.sync_device_metadata(&profile).await
}
