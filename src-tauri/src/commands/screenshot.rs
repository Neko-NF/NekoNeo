use std::fs;

use base64::{engine::general_purpose::STANDARD, Engine};
use tauri::State;

use crate::{
    app_state::AppState,
    errors::AppError,
    models::screenshot::LatestScreenshot,
    services::screenshot::ScreenshotService,
};

#[tauri::command]
pub async fn screenshot_capture_now(state: State<'_, AppState>) -> Result<String, AppError> {
    let config = state.config.lock().await.clone();
    let capture = ScreenshotService::capture_now(&config)?;
    Ok(capture.path)
}

#[tauri::command]
pub fn screenshot_get_latest() -> Result<Option<LatestScreenshot>, AppError> {
    match ScreenshotService::latest()? {
        Some(mut s) => {
            if let Ok(bytes) = fs::read(&s.path) {
                s.data_url = Some(format!(
                    "data:image/png;base64,{}",
                    STANDARD.encode(&bytes)
                ));
            }
            Ok(Some(s))
        }
        None => Ok(None),
    }
}
