use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub version: String,
    pub release_notes: String,
    pub mandatory: bool,
    pub channel: String,
    pub asset_name: Option<String>,
    pub published_at: Option<String>,
    pub downloaded: bool,
}
