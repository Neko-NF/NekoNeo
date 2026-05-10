use std::path::Path;

use reqwest::{multipart, Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    errors::AppError,
    models::{
        auth::{AuthCredentials, AuthResponse, DeviceKeyRequest, DeviceKeyResponse, UserInfo},
        service::TickResult,
        system::{ConnectivityStatus, DeviceProfile},
    },
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidateResponse {
    pub valid: bool,
    pub device_id: Option<u64>,
    pub warning: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ReportOutcome {
    pub takeover_occurred: bool,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ErrorBody {
    code: Option<String>,
    error_code: Option<String>,
    message: Option<String>,
}

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

    pub async fn validate_device_key(
        &self,
        server_url_override: Option<&str>,
        device_key_override: Option<&str>,
    ) -> Result<ValidateResponse, AppError> {
        let base_url = server_url_override.unwrap_or(&self.server_url).trim();
        if base_url.is_empty() {
            return Err(AppError::MissingServerConfig("请先配置上报服务器地址".into()));
        }

        let device_key = device_key_override.unwrap_or(&self.device_key).trim();
        if device_key.is_empty() {
            return Err(AppError::MissingDeviceKey("请先配置设备密钥".into()));
        }

        let url = format!("{}/api/device/validate", base_url.trim_end_matches('/'));
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {device_key}"))
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(map_http_error(status, &body));
        }

        let validation = serde_json::from_str::<ValidateResponse>(&body).unwrap_or_else(|_| ValidateResponse {
            valid: true,
            ..ValidateResponse::default()
        });

        if validation.warning.as_deref() == Some("KEY_BOUND_TO_OTHER_DEVICE") {
            return Err(AppError::TakeoverRequired(
                validation
                    .message
                    .clone()
                    .unwrap_or_else(|| "该密钥已绑定到其他设备，请确认是否接管".into()),
            ));
        }

        Ok(validation)
    }

    pub async fn report_status_v2(&self, payload: &TickResult) -> Result<ReportOutcome, AppError> {
        let base_url = self.server_url.trim();
        if base_url.is_empty() {
            return Err(AppError::MissingServerConfig("请先配置上报服务器地址".into()));
        }

        if self.device_key.trim().is_empty() {
            return Err(AppError::MissingDeviceKey("请先配置设备密钥".into()));
        }

        let url = format!("{}/api/v2/status/report", base_url.trim_end_matches('/'));
        let mut data = serde_json::json!({
            "deviceKey": self.device_key,
            "appName": payload.app_name,
            "packageName": payload.app_name,
            "status": payload.user_status,
            "batteryLevel": payload.battery_level,
            "isCharging": payload.is_charging,
            "screenStatus": "on",
            "hasScreenshot": payload.has_screenshot,
            "screenshotBlurred": payload.screenshot_blurred,
        });

        if let Some(device_data) = data.as_object_mut() {
            if let Some(path) = &payload.screenshot_path {
                device_data.insert("screenshotPath".into(), serde_json::Value::String(path.clone()));
            }
        }

        let mut form = multipart::Form::new().text(
            "data",
            serde_json::to_string(&data).map_err(|error| AppError::NetworkError(error.to_string()))?,
        );

        if let Some(path) = payload.screenshot_path.as_deref() {
            if Path::new(path).exists() {
                let bytes = std::fs::read(path)
                    .map_err(|error| AppError::NetworkError(error.to_string()))?;
                let screenshot = multipart::Part::bytes(bytes)
                    .file_name("screenshot.png")
                    .mime_str("image/png")
                    .map_err(|error| AppError::NetworkError(error.to_string()))?;
                form = form.part("screenshot", screenshot);
            }
        }

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.device_key))
            .multipart(form)
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(map_http_error(status, &body));
        }

        let json = serde_json::from_str::<serde_json::Value>(&body).unwrap_or_default();
        let takeover_occurred = json
            .get("takeover")
            .and_then(|value| value.get("occurred"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        let message = json
            .get("message")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string);

        Ok(ReportOutcome {
            takeover_occurred,
            message,
        })
    }

    pub async fn test_connectivity(
        &self,
        server_url_override: Option<&str>,
        device_key_override: Option<&str>,
    ) -> Result<ConnectivityStatus, AppError> {
        let base_url = server_url_override.unwrap_or(&self.server_url).trim();
        if base_url.is_empty() {
            return Err(AppError::MissingServerConfig("请先配置上报服务器地址".into()));
        }

        let url = format!("{}/api/health", base_url.trim_end_matches('/'));
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if status.is_success() {
            let detail = if body.trim().is_empty() {
                "health endpoint reachable".to_string()
            } else {
                body
            };

            return Ok(ConnectivityStatus {
                reachable: true,
                url,
                detail,
                status_code: Some(status.as_u16()),
            });
        }

        let detail = if !body.trim().is_empty() {
            body
        } else {
            format!("HTTP {}", status.as_u16())
        };

        if status == StatusCode::NOT_FOUND {
            let device_key = device_key_override.unwrap_or(&self.device_key).trim();
            if !device_key.is_empty() {
                self.validate_device_key(server_url_override, Some(device_key)).await?;
                return Ok(ConnectivityStatus {
                    reachable: true,
                    url,
                    detail: "health endpoint missing, but device key validation succeeded".into(),
                    status_code: Some(status.as_u16()),
                });
            }
        }

        Ok(ConnectivityStatus {
            reachable: false,
            url,
            detail,
            status_code: Some(status.as_u16()),
        })
    }

    pub async fn sync_device_metadata(
        &self,
        profile: &DeviceProfile,
    ) -> Result<DeviceProfile, AppError> {
        let base_url = self.server_url.trim();
        if base_url.is_empty() {
            return Err(AppError::MissingServerConfig("请先配置上报服务器地址".into()));
        }

        if self.device_key.trim().is_empty() {
            return Err(AppError::MissingDeviceKey("请先配置设备密钥".into()));
        }

        let url = format!("{}/api/device/sync", base_url.trim_end_matches('/'));
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.device_key))
            .json(profile)
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(map_http_error(status, &body));
        }

        Ok(profile.clone())
    }

    pub async fn auth_register(
        &self,
        credentials: &AuthCredentials,
    ) -> Result<AuthResponse, AppError> {
        let base_url = self.server_url.trim();
        if base_url.is_empty() {
            return Err(AppError::MissingServerConfig(
                "请先配置上报服务器地址".into(),
            ));
        }

        let url = format!("{}/api/auth/register", base_url.trim_end_matches('/'));
        let response = self
            .client
            .post(&url)
            .json(credentials)
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(map_http_error(status, &body));
        }

        serde_json::from_str(&body).map_err(|error| AppError::NetworkError(error.to_string()))
    }

    pub async fn auth_login(
        &self,
        credentials: &AuthCredentials,
    ) -> Result<AuthResponse, AppError> {
        let base_url = self.server_url.trim();
        if base_url.is_empty() {
            return Err(AppError::MissingServerConfig(
                "请先配置上报服务器地址".into(),
            ));
        }

        let url = format!("{}/api/auth/login", base_url.trim_end_matches('/'));
        let response = self
            .client
            .post(&url)
            .json(credentials)
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(map_http_error(status, &body));
        }

        serde_json::from_str(&body).map_err(|error| AppError::NetworkError(error.to_string()))
    }

    pub async fn auth_get_me(&self, token: &str) -> Result<UserInfo, AppError> {
        let base_url = self.server_url.trim();
        if base_url.is_empty() {
            return Err(AppError::MissingServerConfig(
                "请先配置上报服务器地址".into(),
            ));
        }

        let url = format!("{}/api/auth/me", base_url.trim_end_matches('/'));
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(map_http_error(status, &body));
        }

        serde_json::from_str(&body).map_err(|error| AppError::NetworkError(error.to_string()))
    }

    pub async fn auth_generate_device_key(
        &self,
        token: &str,
        request: &DeviceKeyRequest,
    ) -> Result<DeviceKeyResponse, AppError> {
        let base_url = self.server_url.trim();
        if base_url.is_empty() {
            return Err(AppError::MissingServerConfig(
                "请先配置上报服务器地址".into(),
            ));
        }

        let url = format!("{}/api/auth/device-key", base_url.trim_end_matches('/'));
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {token}"))
            .json(request)
            .send()
            .await
            .map_err(|error| AppError::NetworkError(error.to_string()))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(map_http_error(status, &body));
        }

        serde_json::from_str(&body).map_err(|error| AppError::NetworkError(error.to_string()))
    }
}

fn map_http_error(status: StatusCode, body: &str) -> AppError {
    let payload = serde_json::from_str::<ErrorBody>(body).unwrap_or_default();
    let code = payload
        .code
        .as_deref()
        .or(payload.error_code.as_deref())
        .unwrap_or_default();
    let message = payload
        .message
        .unwrap_or_else(|| format!("HTTP {}", status.as_u16()));

    match status {
        StatusCode::UNAUTHORIZED => AppError::InvalidKey(message),
        StatusCode::FORBIDDEN => match code {
            "KEY_REVOKED" => AppError::KeyRevoked(message),
            "INVALID_KEY" => AppError::InvalidKey(message),
            _ => AppError::KeyRevoked(message),
        },
        StatusCode::NOT_FOUND => match code {
            "DEVICE_NOT_FOUND" | "KEY_NOT_FOUND" => AppError::DeviceNotFound(message),
            _ => AppError::NetworkError(message),
        },
        _ => AppError::NetworkError(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_report_status_v2_posts_multipart() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/v2/status/report")
            .match_header("authorization", "Bearer device-key")
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
            idle_ms: 0,
            has_screenshot: false,
            screenshot_blurred: false,
            screenshot_path: None,
            error: None,
        };

        client.report_status_v2(&payload).await.unwrap();
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_validate_device_key_conflict_returns_takeover_required() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/device/validate")
            .match_header("authorization", "Bearer device-key")
            .with_status(200)
            .with_body(r#"{"valid":true,"warning":"KEY_BOUND_TO_OTHER_DEVICE","message":"already bound"}"#)
            .create_async()
            .await;

        let client = ApiClient::new(server.url(), "device-key");
        let result = client.validate_device_key(None, None).await;

        assert!(matches!(result, Err(AppError::TakeoverRequired(_))));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_validate_device_key_401_returns_invalid_key() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/device/validate")
            .match_header("authorization", "Bearer bad-key")
            .with_status(401)
            .with_body(r#"{"code":"INVALID_KEY","message":"bad key"}"#)
            .create_async()
            .await;

        let client = ApiClient::new(server.url(), "bad-key");
        let result = client.validate_device_key(None, None).await;

        assert!(matches!(result, Err(AppError::InvalidKey(_))));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_connectivity_uses_health_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/health")
            .with_status(200)
            .with_body("ok")
            .create_async()
            .await;

        let client = ApiClient::new(server.url(), "device-key");
        let result = client.test_connectivity(None, None).await.unwrap();

        assert!(result.reachable);
        assert_eq!(result.status_code, Some(200));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_sync_device_metadata_posts_profile() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/device/sync")
            .match_header("authorization", "Bearer device-key")
            .with_status(200)
            .create_async()
            .await;

        let client = ApiClient::new(server.url(), "device-key");
        let profile = DeviceProfile {
            hostname: "host".into(),
            os_friendly_name: "Windows".into(),
            cpu_model: "CPU".into(),
            cpu_cores: 8,
            app_version: "2.0.0-alpha".into(),
        };

        let result = client.sync_device_metadata(&profile).await.unwrap();
        assert_eq!(result.hostname, "host");
        mock.assert_async().await;
    }
}
