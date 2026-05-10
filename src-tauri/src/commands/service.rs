use tauri::{AppHandle, State};

use crate::{app_state::AppState, errors::AppError, models::service::{ServiceStatus, TickResult}};

#[tauri::command]
pub async fn service_start(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ServiceStatus, AppError> {
    Ok(state.reporter.start(app).await)
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
