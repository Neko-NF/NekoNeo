use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerMode {
    Production,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CloseAction {
    Ask,
    Minimize,
    Exit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IncognitoScope {
    Screenshot,
    Title,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateChannel {
    Stable,
    Beta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub device_key: String,
    pub device_id: Option<u64>,
    pub report_interval: u64,
    pub server_mode: ServerMode,
    pub server_url_prod: String,
    pub server_url_local: String,
    pub enable_screenshot: bool,
    pub screenshot_interval: u64,
    pub sync_screenshot_interval: bool,
    pub enable_auto_start: bool,
    pub minimize_on_auto_start: bool,
    pub startup_delay_ms: u64,
    pub enable_auto_service_start: bool,
    pub close_action: CloseAction,
    pub theme_mode: ThemeMode,
    pub seed_color: String,
    pub ui_scale: u32,
    pub ui_font: String,
    pub enable_notification: bool,
    pub do_not_disturb: bool,
    pub enable_incognito: bool,
    pub incognito_scope: IncognitoScope,
    pub blur_all_screenshots: bool,
    pub privacy_rules: Vec<String>,
    pub enable_auto_restart: bool,
    pub max_restarts: u32,
    pub restart_interval_sec: u64,
    pub watchdog_timeout_sec: u64,
    pub auto_check_update: bool,
    pub update_channel: UpdateChannel,
    pub auto_download: bool,
    pub skipped_version: String,
}

impl AppConfig {
    pub fn active_server_url(&self) -> &str {
        match self.server_mode {
            ServerMode::Production => &self.server_url_prod,
            ServerMode::Local => &self.server_url_local,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            device_key: String::new(),
            device_id: None,
            report_interval: 30,
            server_mode: ServerMode::Production,
            server_url_prod: "https://api.example.com".into(),
            server_url_local: "http://127.0.0.1:3000".into(),
            enable_screenshot: true,
            screenshot_interval: 60,
            sync_screenshot_interval: true,
            enable_auto_start: false,
            minimize_on_auto_start: true,
            startup_delay_ms: 0,
            enable_auto_service_start: false,
            close_action: CloseAction::Minimize,
            theme_mode: ThemeMode::Dark,
            seed_color: "#06b6d4".into(),
            ui_scale: 100,
            ui_font: "Segoe UI".into(),
            enable_notification: true,
            do_not_disturb: false,
            enable_incognito: false,
            incognito_scope: IncognitoScope::Both,
            blur_all_screenshots: false,
            privacy_rules: Vec::new(),
            enable_auto_restart: true,
            max_restarts: 3,
            restart_interval_sec: 30,
            watchdog_timeout_sec: 120,
            auto_check_update: true,
            update_channel: UpdateChannel::Stable,
            auto_download: false,
            skipped_version: String::new(),
        }
    }
}
