use tauri::{AppHandle, Manager};

use crate::errors::AppError;

#[tauri::command]
pub fn app_resolve_close_request(app: AppHandle, action: String) -> Result<(), AppError> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| AppError::SystemError("main window not found".into()))?;

    match action.as_str() {
        "minimize" => {
            window
                .minimize()
                .map_err(|error| AppError::SystemError(error.to_string()))?;
            window
                .hide()
                .map_err(|error| AppError::SystemError(error.to_string()))?;
        }
        "exit" => {
            app.exit(0);
        }
        "cancel" => {}
        _ => {
            return Err(AppError::SystemError(format!(
                "unsupported close resolution action: {action}"
            )));
        }
    }

    Ok(())
}
