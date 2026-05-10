use tauri::State;

use crate::{
    app_state::AppState,
    errors::AppError,
    models::config::AppConfig,
    services::config_store::ConfigStore,
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

    let mut api_client = state.api_client.lock().await;
    api_client.reconfigure(config.active_server_url(), config.device_key.clone());

    Ok(())
}
