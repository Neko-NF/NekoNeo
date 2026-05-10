use tauri::State;

use crate::{
    app_state::AppState,
    errors::AppError,
    models::auth::{
        AuthCredentials, AuthResponse, DeviceKeyRequest, DeviceKeyResponse, UserInfo,
    },
};

#[tauri::command]
pub async fn auth_register(
    state: State<'_, AppState>,
    credentials: AuthCredentials,
) -> Result<AuthResponse, AppError> {
    let api = state.api_client.lock().await;
    api.auth_register(&credentials).await
}

#[tauri::command]
pub async fn auth_login(
    state: State<'_, AppState>,
    credentials: AuthCredentials,
) -> Result<AuthResponse, AppError> {
    let api = state.api_client.lock().await;
    api.auth_login(&credentials).await
}

#[tauri::command]
pub async fn auth_get_me(
    state: State<'_, AppState>,
    token: String,
) -> Result<UserInfo, AppError> {
    let api = state.api_client.lock().await;
    api.auth_get_me(&token).await
}

#[tauri::command]
pub async fn auth_generate_device_key(
    state: State<'_, AppState>,
    token: String,
    request: DeviceKeyRequest,
) -> Result<DeviceKeyResponse, AppError> {
    let api = state.api_client.lock().await;
    api.auth_generate_device_key(&token, &request).await
}
