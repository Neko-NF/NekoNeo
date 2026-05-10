use std::{
    sync::LazyLock,
    time::Instant,
};

use sysinfo::{Networks, System};
use tokio::sync::Mutex;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HWND, LPARAM, TRUE},
        Graphics::Gdi::{
            CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, SelectObject,
            BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, RGBQUAD,
        },
        System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS},
        UI::WindowsAndMessaging::{
            EnumWindows, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW,
            GetWindowThreadProcessId, IsWindowVisible,
        },
        UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
        UI::Shell::ExtractIconExW,
        UI::WindowsAndMessaging::GetWindowRect,
    },
};

use crate::models::{
    config::AppConfig,
    service::TickResult,
    system::{DeviceProfile, MediaInfo, SystemMetrics, WindowInfo},
};

pub struct SystemInfo;

static SYSTEM_SAMPLER: LazyLock<Mutex<SystemSampler>> =
    LazyLock::new(|| Mutex::new(SystemSampler::new()));

impl SystemInfo {
    pub async fn get_metrics() -> SystemMetrics {
        let mut sampler = SYSTEM_SAMPLER.lock().await;
        sampler.refresh_metrics()
    }

    pub async fn create_tick_result(tick_count: u64, config: &AppConfig) -> TickResult {
        let mut sampler = SYSTEM_SAMPLER.lock().await;
        sampler.refresh_tick_result(tick_count, config)
    }

    pub fn get_fonts() -> Vec<String> {
        collect_fonts()
    }

    pub fn list_visible_windows() -> Vec<WindowInfo> {
        collect_visible_windows()
    }

    pub fn get_idle_ms() -> u64 {
        let mut info = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            ..Default::default()
        };
        if !unsafe { GetLastInputInfo(&mut info) }.as_bool() {
            return 0;
        }
        let tick = unsafe { windows::Win32::System::SystemInformation::GetTickCount() };
        (tick.saturating_sub(info.dwTime)) as u64
    }

    pub async fn get_media_info() -> Option<MediaInfo> {
        let manager = windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .ok()?
            .get()
            .ok()?;

        let session = manager.GetCurrentSession().ok()?;
        let props = session.TryGetMediaPropertiesAsync().ok()?.get().ok()?;

        let title = props.Title().map(|s| s.to_string()).unwrap_or_default();
        if title.is_empty() {
            return None;
        }

        Some(MediaInfo {
            title,
            artist: props.Artist().map(|s| s.to_string()).unwrap_or_default(),
            album: props.AlbumTitle().map(|s| s.to_string()).unwrap_or_default(),
            thumbnail_path: None,
        })
    }

    pub fn get_process_icon(exe_path: &str) -> Option<Vec<u8>> {
        extract_icon_png(exe_path)
    }

    pub fn should_blur_screenshot(config: &AppConfig) -> bool {
        let mut system = System::new_all();
        system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        collect_foreground_window(&system)
            .as_ref()
            .map(|window| matches_privacy_rule(window, &config.privacy_rules))
            .unwrap_or(false)
    }

    pub async fn get_device_profile() -> DeviceProfile {
        let metrics = Self::get_metrics().await;

        DeviceProfile {
            hostname: metrics.hostname,
            os_friendly_name: metrics.os_friendly_name,
            cpu_model: metrics.cpu_model,
            cpu_cores: metrics.cpu_cores,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        }
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

    fn refresh_tick_result(&mut self, _tick_count: u64, config: &AppConfig) -> TickResult {
        let _metrics = self.refresh_metrics();
        let foreground_window = collect_foreground_window(&self.system);
        let top_process = self.system.processes().values().max_by(|left, right| {
            left.cpu_usage()
                .partial_cmp(&right.cpu_usage())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let privacy_hit = foreground_window
            .as_ref()
            .map(|window| matches_privacy_rule(window, &config.privacy_rules))
            .unwrap_or(false);

        let app_name = foreground_window
            .as_ref()
            .map(|window| {
                if privacy_hit {
                    "hidden-by-privacy-rule".to_string()
                } else {
                    window.process_name.clone()
                }
            })
            .filter(|name| !name.is_empty())
            .or_else(|| {
                top_process
                    .map(|process| process.name().to_string_lossy().to_string())
                    .filter(|name| !name.is_empty())
            })
            .unwrap_or_else(|| "unknown-process".into());

        let power = read_power_status();
        let idle_ms = SystemInfo::get_idle_ms();
        let user_status = if idle_ms > 120_000 {
            "away"
        } else {
            "online"
        };

        TickResult {
            success: true,
            timestamp: super::status_reporter::now_iso_like(),
            app_name,
            battery_level: power.battery_level,
            is_charging: power.is_charging,
            has_battery: power.has_battery,
            user_status: user_status.into(),
            idle_ms,
            has_screenshot: false,
            screenshot_blurred: privacy_hit || config.blur_all_screenshots,
            screenshot_path: None,
            error: None,
        }
    }
}

fn matches_privacy_rule(window: &WindowInfo, rules: &[String]) -> bool {
    let process_name = window.process_name.trim();
    let title = window.title.trim();
    let composite = build_privacy_rule_key(process_name, title);

    rules.iter().any(|rule| {
        let normalized = rule.trim();
        normalized.eq_ignore_ascii_case(composite.as_str())
            || normalized.eq_ignore_ascii_case(process_name)
            || normalized.eq_ignore_ascii_case(title)
    })
}

pub fn build_privacy_rule_key(process_name: &str, title: &str) -> String {
    format!("{}::{}", process_name.trim(), title.trim())
}

fn collect_fonts() -> Vec<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let fonts_key =
        match hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts") {
            Ok(value) => value,
            Err(_) => return default_fonts(),
        };

    let mut fonts = fonts_key
        .enum_values()
        .filter_map(Result::ok)
        .map(|(name, _)| strip_font_suffix(&name))
        .filter(|name| !name.is_empty())
        .collect::<Vec<_>>();

    fonts.sort();
    fonts.dedup();

    if fonts.is_empty() {
        default_fonts()
    } else {
        fonts
    }
}

fn default_fonts() -> Vec<String> {
    vec![
        "Segoe UI".into(),
        "Microsoft YaHei UI".into(),
        "JetBrains Mono".into(),
    ]
}

fn strip_font_suffix(name: &str) -> String {
    name.replace(" (TrueType)", "")
        .replace(" (OpenType)", "")
        .trim()
        .to_string()
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

    let bounds = get_window_bounds(hwnd);

    Some(WindowInfo {
        title,
        process_name,
        pid,
        path,
        bounds,
    })
}

fn get_window_bounds(hwnd: HWND) -> Option<crate::models::system::WindowBounds> {
    let mut rect = windows::Win32::Foundation::RECT::default();
    if unsafe { GetWindowRect(hwnd, &mut rect) }.is_err() {
        return None;
    }
    Some(crate::models::system::WindowBounds {
        x: rect.left,
        y: rect.top,
        width: (rect.right - rect.left).max(0) as u32,
        height: (rect.bottom - rect.top).max(0) as u32,
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

#[derive(Debug, Clone, Copy)]
struct PowerSnapshot {
    battery_level: u8,
    is_charging: bool,
    has_battery: bool,
}

fn read_power_status() -> PowerSnapshot {
    let mut status = SYSTEM_POWER_STATUS::default();
    if unsafe { GetSystemPowerStatus(&mut status) }.is_err() {
        return PowerSnapshot {
            battery_level: 0,
            is_charging: false,
            has_battery: false,
        };
    }

    let has_battery = status.BatteryFlag != 128 && status.BatteryFlag != 255;
    let battery_level = if has_battery && status.BatteryLifePercent != 255 {
        status.BatteryLifePercent
    } else {
        0
    };
    let is_charging = status.ACLineStatus == 1;

    PowerSnapshot {
        battery_level,
        is_charging,
        has_battery,
    }
}

fn extract_icon_png(exe_path: &str) -> Option<Vec<u8>> {
    use std::os::windows::ffi::OsStrExt;

    let path_wide: Vec<u16> = std::ffi::OsStr::new(exe_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut icon_handle = windows::Win32::UI::WindowsAndMessaging::HICON::default();
    let extracted = unsafe {
        ExtractIconExW(
            PCWSTR::from_raw(path_wide.as_ptr()),
            0,
            None,
            Some(&mut icon_handle),
            1,
        )
    };

    if extracted == 0 || icon_handle.is_invalid() {
        return None;
    }

    let png = bitmap_to_png(icon_handle);
    let _ = unsafe { windows::Win32::UI::WindowsAndMessaging::DestroyIcon(icon_handle) };
    png
}

fn bitmap_to_png(
    hicon: windows::Win32::UI::WindowsAndMessaging::HICON,
) -> Option<Vec<u8>> {
    use image::ImageEncoder;
    use windows::Win32::{
        Graphics::Gdi::BITMAP,
        UI::WindowsAndMessaging::{GetIconInfo, ICONINFO},
    };

    // Helper: call a GDI cleanup fn and ignore the result.
    macro_rules! gdi_drop {
        ($e:expr) => {{
            let _ = unsafe { $e };
        }};
    }

    let mut info = ICONINFO::default();
    if unsafe { GetIconInfo(hicon, &mut info) }.is_err() {
        return None;
    }

    let hbm_color = info.hbmColor;
    if hbm_color.is_invalid() {
        gdi_drop!(DeleteObject(info.hbmMask.into()));
        return None;
    }

    let mut bmp: BITMAP = unsafe { std::mem::zeroed() };
    let written = unsafe {
        GetObjectW(
            hbm_color.into(),
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut BITMAP as *mut _),
        )
    };

    if written == 0 {
        gdi_drop!(DeleteObject(hbm_color.into()));
        gdi_drop!(DeleteObject(info.hbmMask.into()));
        return None;
    }

    let width = bmp.bmWidth as u32;
    let height = bmp.bmHeight as u32;
    let row_size = ((width * 32 + 31) / 32) * 4;
    let buf_size = (row_size * height) as usize;
    let mut pixels = vec![0u8; buf_size];

    let dc = unsafe { CreateCompatibleDC(None) };
    if dc.is_invalid() {
        gdi_drop!(DeleteObject(hbm_color.into()));
        gdi_drop!(DeleteObject(info.hbmMask.into()));
        return None;
    }

    let old = unsafe { SelectObject(dc, hbm_color.into()) };

    let header = BITMAPINFOHEADER {
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width as i32,
        biHeight: -(height as i32),
        biPlanes: 1,
        biBitCount: 32,
        biCompression: 0,
        biSizeImage: buf_size as u32,
        ..Default::default()
    };

    let mut bmi = BITMAPINFO {
        bmiHeader: header,
        bmiColors: [RGBQUAD::default(); 1],
    };

    let copied = unsafe {
        GetDIBits(
            dc,
            hbm_color,
            0,
            height,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        )
    };

    gdi_drop!(SelectObject(dc, old));
    gdi_drop!(DeleteDC(dc));
    gdi_drop!(DeleteObject(hbm_color.into()));
    gdi_drop!(DeleteObject(info.hbmMask.into()));

    if copied == 0 {
        return None;
    }

    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            let offset = (y * row_size + x * 4) as usize;
            rgba.push(pixels[offset + 2]);
            rgba.push(pixels[offset + 1]);
            rgba.push(pixels[offset]);
            rgba.push(pixels[offset + 3]);
        }
    }

    let mut out = std::io::Cursor::new(Vec::new());
    image::codecs::png::PngEncoder::new(&mut out)
        .write_image(&rgba, width, height, image::ColorType::Rgba8)
        .ok()?;

    Some(out.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_privacy_rule_key() {
        assert_eq!(
            build_privacy_rule_key("Code.exe", "Visual Studio Code"),
            "Code.exe::Visual Studio Code"
        );
    }

    #[test]
    fn test_strip_font_suffix() {
        assert_eq!(strip_font_suffix("Segoe UI (TrueType)"), "Segoe UI");
        assert_eq!(strip_font_suffix("Cascadia Code"), "Cascadia Code");
    }

    #[test]
    fn test_power_status_no_panic() {
        let snapshot = read_power_status();
        assert!(snapshot.battery_level <= 100);
    }
}
