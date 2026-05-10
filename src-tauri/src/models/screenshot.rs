use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LatestScreenshot {
    pub path: String,
    pub blurred: bool,
    pub captured_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_url: Option<String>,
}
