use std::collections::HashMap;

use crate::{errors::AppError, models::system::SystemMetrics, services::system_info::SystemInfo};

#[tauri::command]
pub async fn system_get_metrics() -> Result<SystemMetrics, AppError> {
    Ok(SystemInfo::get_metrics().await)
}

#[tauri::command]
pub async fn system_health_check() -> Result<HashMap<String, String>, AppError> {
    let mut health = HashMap::new();
    health.insert("frontend".into(), "ready".into());
    health.insert("backend".into(), "ready".into());
    health.insert("ipc".into(), "ready".into());
    Ok(health)
}

#[tauri::command]
pub fn system_get_fonts() -> Result<Vec<String>, AppError> {
    Ok(SystemInfo::get_fonts())
}
