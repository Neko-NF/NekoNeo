#[derive(Debug, Clone)]
pub struct ApiClient {
    pub server_url: String,
    pub device_key: String,
}

impl ApiClient {
    pub fn new(server_url: impl Into<String>, device_key: impl Into<String>) -> Self {
        Self {
            server_url: server_url.into(),
            device_key: device_key.into(),
        }
    }
}
