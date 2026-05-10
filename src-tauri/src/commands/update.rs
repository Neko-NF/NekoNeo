use tauri::{AppHandle, State};

use crate::{
    app_state::AppState,
    errors::AppError,
    models::update::UpdateInfo,
    services::update_service::UpdateService,
};

#[tauri::command]
pub async fn update_check(
    state: State<'_, AppState>,
    channel: String,
) -> Result<Option<UpdateInfo>, AppError> {
    let skipped_version = state.config.lock().await.skipped_version.clone();
    UpdateService::check(&channel, Some(skipped_version.as_str())).await
}

#[tauri::command]
pub async fn update_download(app: AppHandle, channel: String) -> Result<String, AppError> {
    UpdateService::download(&app, &channel).await
}

#[tauri::command]
pub fn update_install(app: AppHandle) -> Result<(), AppError> {
    UpdateService::install(&app)
}
