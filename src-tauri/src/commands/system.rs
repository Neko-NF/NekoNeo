use std::collections::HashMap;

use crate::{
    errors::AppError,
    models::system::{DeviceProfile, MediaInfo, SystemMetrics},
    services::{autostart::AutostartService, device_fingerprint, system_info::SystemInfo},
};

#[tauri::command]
pub async fn system_get_metrics() -> Result<SystemMetrics, AppError> {
    Ok(SystemInfo::get_metrics().await)
}

#[tauri::command]
pub async fn system_get_device_profile() -> Result<DeviceProfile, AppError> {
    Ok(SystemInfo::get_device_profile().await)
}

#[tauri::command]
pub async fn system_health_check() -> Result<HashMap<String, String>, AppError> {
    let mut health = HashMap::new();
    health.insert("frontend".into(), "ready".into());
    health.insert("backend".into(), "ready".into());
    health.insert("ipc".into(), "ready".into());
    health.insert(
        "autostart".into(),
        if AutostartService::is_enabled()? {
            "enabled".into()
        } else {
            "disabled".into()
        },
    );
    Ok(health)
}

#[tauri::command]
pub fn system_get_fonts() -> Result<Vec<String>, AppError> {
    Ok(SystemInfo::get_fonts())
}

#[tauri::command]
pub async fn system_get_media_info() -> Result<Option<MediaInfo>, AppError> {
    Ok(SystemInfo::get_media_info().await)
}

#[tauri::command]
pub fn system_get_device_fingerprint() -> Result<String, AppError> {
    Ok(device_fingerprint::compute_fingerprint())
}

#[tauri::command]
pub fn system_get_process_icon(exe_path: String) -> Result<Option<Vec<u8>>, AppError> {
    Ok(SystemInfo::get_process_icon(&exe_path))
}
