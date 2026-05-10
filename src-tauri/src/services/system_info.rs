use std::{
    sync::LazyLock,
    time::Instant,
};

use sysinfo::{Networks, System};
use tokio::sync::Mutex;
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, TRUE},
        UI::WindowsAndMessaging::{
            EnumWindows, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW,
            GetWindowThreadProcessId, IsWindowVisible,
        },
    },
};

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
        collect_visible_windows()
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
        let foreground_window = collect_foreground_window(&self.system);
        let top_process = self.system.processes().values().max_by(|left, right| {
            left.cpu_usage()
                .partial_cmp(&right.cpu_usage())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let app_name = foreground_window
            .as_ref()
            .map(|window| window.process_name.clone())
            .filter(|name| !name.is_empty())
            .or_else(|| {
                top_process
                    .map(|process| process.name().to_string_lossy().to_string())
                    .filter(|name| !name.is_empty())
            })
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

fn collect_visible_windows() -> Vec<WindowInfo> {
    let mut system = System::new_all();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let mut handles = Vec::<isize>::new();

    unsafe {
        let _ = EnumWindows(
            Some(enum_window_callback),
            LPARAM((&mut handles as *mut Vec<isize>) as isize),
        );
    }

    let mut windows = handles
        .into_iter()
        .filter_map(|raw| build_window_info(HWND(raw as *mut _), &system))
        .collect::<Vec<_>>();

    windows.sort_by(|left, right| left.title.to_lowercase().cmp(&right.title.to_lowercase()));
    windows
}

fn collect_foreground_window(system: &System) -> Option<WindowInfo> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        return None;
    }

    build_window_info(hwnd, system)
}

fn build_window_info(hwnd: HWND, system: &System) -> Option<WindowInfo> {
    let title = get_window_title(hwnd)?;
    let pid = get_window_pid(hwnd)?;
    let process = system.process(sysinfo::Pid::from_u32(pid))?;
    let process_name = process.name().to_string_lossy().to_string();
    let path = process
        .exe()
        .map(|value| value.display().to_string())
        .unwrap_or_default();

    Some(WindowInfo {
        title,
        process_name,
        pid,
        path,
    })
}

unsafe extern "system" fn enum_window_callback(
    hwnd: HWND,
    lparam: LPARAM,
) -> windows_core::BOOL {
    if !unsafe { IsWindowVisible(hwnd) }.as_bool() {
        return TRUE;
    }

    if get_window_title(hwnd).is_none() {
        return TRUE;
    }

    let handles = unsafe { &mut *(lparam.0 as *mut Vec<isize>) };
    handles.push(hwnd.0 as isize);
    TRUE
}

fn get_window_title(hwnd: HWND) -> Option<String> {
    let len = unsafe { GetWindowTextLengthW(hwnd) };
    if len <= 0 {
        return None;
    }

    let mut buffer = vec![0u16; len as usize + 1];
    let written = unsafe { GetWindowTextW(hwnd, &mut buffer) };
    if written <= 0 {
        return None;
    }

    let title = String::from_utf16_lossy(&buffer[..written as usize])
        .trim()
        .to_string();

    if title.is_empty() {
        None
    } else {
        Some(title)
    }
}

fn get_window_pid(hwnd: HWND) -> Option<u32> {
    let mut pid = 0u32;
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
    }

    if pid == 0 {
        None
    } else {
        Some(pid)
    }
}
