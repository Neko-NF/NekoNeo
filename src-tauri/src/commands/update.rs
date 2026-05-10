use crate::{errors::AppError, models::update::UpdateInfo};

#[tauri::command]
pub async fn update_check(channel: String) -> Result<Option<UpdateInfo>, AppError> {
    Ok(Some(UpdateInfo {
        version: "2.0.0-beta.1".into(),
        release_notes: format!("Scaffold channel: {channel}"),
        mandatory: false,
    }))
}
