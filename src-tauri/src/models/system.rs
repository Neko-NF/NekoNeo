use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowInfo {
    pub title: String,
    pub process_name: String,
    pub pid: u32,
    pub path: String,
}
