use reqwest::Client;

use crate::{errors::AppError, models::service::TickResult};

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub client: Client,
    pub server_url: String,
    pub device_key: String,
}

impl ApiClient {
    pub fn new(server_url: impl Into<String>, device_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            server_url: server_url.into(),
            device_key: device_key.into(),
        }
    }

    pub fn reconfigure(&mut self, server_url: impl Into<String>, device_key: impl Into<String>) {
        self.server_url = server_url.into();
        self.device_key = device_key.into();
    }

    pub async fn report_status_v2(&self, payload: &TickResult) -> Result<(), AppError> {
        let base_url = self.server_url.trim();
        if base_url.is_empty() {
            return Err(AppError::NetworkError("server url is empty".into()));
        }

        let url = format!("{}/api/v1/status/report", base_url.trim_end_matches('/'));

        self.client
            .post(url)
            .header("x-device-key", self.device_key.as_str())
            .json(payload)
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?
            .error_for_status()
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_report_status_v2_posts_json() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/v1/status/report")
            .match_header("x-device-key", "device-key")
            .with_status(200)
            .create_async()
            .await;

        let client = ApiClient::new(server.url(), "device-key");
        let payload = TickResult {
            success: true,
            timestamp: "1".into(),
            app_name: "Code.exe".into(),
            battery_level: 80,
            is_charging: true,
            has_battery: true,
            user_status: "online".into(),
            has_screenshot: false,
            screenshot_blurred: false,
            error: None,
        };

        client.report_status_v2(&payload).await.unwrap();
        mock.assert_async().await;
    }
}
