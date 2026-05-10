use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TickResult {
    pub success: bool,
    pub timestamp: String,
    pub app_name: String,
    pub battery_level: u8,
    pub is_charging: bool,
    pub has_battery: bool,
    pub user_status: String,
    pub idle_ms: u64,
    pub has_screenshot: bool,
    pub screenshot_blurred: bool,
    pub screenshot_path: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    pub running: bool,
    pub uptime_sec: u64,
    pub consecutive_failures: u32,
    pub auto_restart_count: u32,
}
