use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::{errors::AppError, models::system::WindowInfo, services::system_info::SystemInfo};

#[tauri::command]
pub fn privacy_get_windows() -> Result<Vec<WindowInfo>, AppError> {
    Ok(SystemInfo::list_visible_windows())
}

#[tauri::command]
pub async fn privacy_open_picker(app: AppHandle) -> Result<(), AppError> {
    if let Some(w) = app.get_webview_window("privacy-picker") {
        let _ = w.close();
    }

    let picker = WebviewWindowBuilder::new(
        &app,
        "privacy-picker",
        WebviewUrl::App("index.html#/picker".into()),
    )
    .title("隐私选择器")
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .visible_on_all_workspaces(true)
    .shadow(false)
    .build()
    .map_err(|e| AppError::SystemError(e.to_string()))?;

    let _ = picker.set_fullscreen(true);
    // Let clicks pass through so user can interact with windows behind
    let _ = picker.set_ignore_cursor_events(true);

    Ok(())
}

#[tauri::command]
pub fn privacy_get_cursor_pos() -> Result<(i32, i32), AppError> {
    let mut pt = windows::Win32::Foundation::POINT::default();
    unsafe { GetCursorPos(&mut pt) }
        .map_err(|e| AppError::SystemError(e.to_string()))?;
    Ok((pt.x, pt.y))
}

#[tauri::command]
pub async fn privacy_close_picker(app: AppHandle, window_json: Option<String>) -> Result<(), AppError> {
    let window: Option<WindowInfo> = match window_json {
        Some(json) => serde_json::from_str(&json).ok(),
        None => None,
    };
    let _ = app.emit("picker:selected", &window);
    if let Some(w) = app.get_webview_window("privacy-picker") {
        let _ = w.close();
    }
    Ok(())
}
