use std::{
    sync::LazyLock,
    time::Instant,
};

use sysinfo::{Networks, System};
use tokio::sync::Mutex;

use crate::models::{
    service::TickResult,
    system::{SystemMetrics, WindowInfo},
};

pub struct SystemInfo;

static SYSTEM_SAMPLER: LazyLock<Mutex<SystemSampler>> =
    LazyLock::new(|| Mutex::new(SystemSampler::new()));

impl SystemInfo {
    pub async fn get_metrics() -> SystemMetrics {
        let mut sampler = SYSTEM_SAMPLER.lock().await;
        sampler.refresh_metrics()
    }

    pub async fn create_tick_result(tick_count: u64) -> TickResult {
        let mut sampler = SYSTEM_SAMPLER.lock().await;
        sampler.refresh_tick_result(tick_count)
    }

    pub fn get_fonts() -> Vec<String> {
        vec![
            "Segoe UI".into(),
            "Microsoft YaHei UI".into(),
            "JetBrains Mono".into(),
        ]
    }

    pub fn list_visible_windows() -> Vec<WindowInfo> {
        vec![
            WindowInfo {
                title: "Visual Studio Code".into(),
                process_name: "Code.exe".into(),
                pid: 4242,
                path: "C:/Program Files/Microsoft VS Code/Code.exe".into(),
            },
            WindowInfo {
                title: "Google Chrome".into(),
                process_name: "chrome.exe".into(),
                pid: 5252,
                path: "C:/Program Files/Google/Chrome/Application/chrome.exe".into(),
            },
        ]
    }
}

struct SystemSampler {
    system: System,
    networks: Networks,
    last_network_refresh: Instant,
}

impl SystemSampler {
    fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh(true);

        Self {
            system,
            networks,
            last_network_refresh: Instant::now(),
        }
    }

    fn refresh_metrics(&mut self) -> SystemMetrics {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let elapsed = self.last_network_refresh.elapsed().as_secs_f64().max(1.0);
        self.networks.refresh(true);

        let bytes_down: u64 = self
            .networks
            .values()
            .map(|network| network.received())
            .sum();
        let bytes_up: u64 = self
            .networks
            .values()
            .map(|network| network.transmitted())
            .sum();

        self.last_network_refresh = Instant::now();

        let cpu = self.system.global_cpu_usage();
        let mem_total = self.system.total_memory();
        let mem_used = self.system.used_memory();
        let mem_pct = if mem_total == 0 {
            0.0
        } else {
            (mem_used as f64 / mem_total as f64 * 100.0) as f32
        };

        let cpu_model = self
            .system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".into());

        SystemMetrics {
            cpu_pct: cpu,
            mem_pct,
            mem_used,
            mem_total,
            net_down_bps: (bytes_down as f64 / elapsed) as u64,
            net_up_bps: (bytes_up as f64 / elapsed) as u64,
            network_latency: -1,
            cpu_model,
            cpu_cores: self.system.cpus().len() as u32,
            uptime: System::uptime(),
            hostname: System::host_name().unwrap_or_else(|| "unknown-host".into()),
            os_friendly_name: System::long_os_version()
                .or_else(System::name)
                .unwrap_or_else(|| "Unknown OS".into()),
        }
    }

    fn refresh_tick_result(&mut self, tick_count: u64) -> TickResult {
        let metrics = self.refresh_metrics();

        let top_process = self
            .system
            .processes()
            .values()
            .max_by(|left, right| {
                left.cpu_usage()
                    .partial_cmp(&right.cpu_usage())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        let app_name = top_process
            .map(|process| process.name().to_string_lossy().to_string())
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| "unknown-process".into());

        TickResult {
            success: true,
            timestamp: super::status_reporter::now_iso_like(),
            app_name,
            battery_level: 0,
            is_charging: false,
            has_battery: false,
            user_status: if metrics.cpu_pct < 8.0 && tick_count % 2 == 0 {
                "away".into()
            } else {
                "online".into()
            },
            has_screenshot: false,
            screenshot_blurred: false,
            error: None,
        }
    }
}
