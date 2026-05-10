use crate::{errors::AppError, models::system::WindowInfo, services::system_info::SystemInfo};

#[tauri::command]
pub fn privacy_get_windows() -> Result<Vec<WindowInfo>, AppError> {
    Ok(SystemInfo::list_visible_windows())
}
