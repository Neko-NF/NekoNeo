use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMetrics {
    pub cpu_pct: f32,
    pub mem_pct: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub net_down_bps: u64,
    pub net_up_bps: u64,
    pub network_latency: i64,
    pub cpu_model: String,
    pub cpu_cores: u32,
    pub uptime: u64,
    pub hostname: String,
    pub os_friendly_name: String,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_pct: 12.4,
            mem_pct: 45.2,
            mem_used: 7240,
            mem_total: 16384,
            net_down_bps: 128_000,
            net_up_bps: 24_000,
            network_latency: 42,
            cpu_model: "Unknown CPU".into(),
            cpu_cores: 8,
            uptime: 36_000,
            hostname: "localhost".into(),
            os_friendly_name: "Windows".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowInfo {
    pub title: String,
    pub process_name: String,
    pub pid: u32,
    pub path: String,
    pub bounds: Option<WindowBounds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProfile {
    pub hostname: String,
    pub os_friendly_name: String,
    pub cpu_model: String,
    pub cpu_cores: u32,
    pub app_version: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivityStatus {
    pub reachable: bool,
    pub url: String,
    pub detail: String,
    pub status_code: Option<u16>,
}
