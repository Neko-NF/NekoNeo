use crate::models::system::{SystemMetrics, WindowInfo};

pub struct SystemInfo;

impl SystemInfo {
    pub async fn get_metrics() -> SystemMetrics {
        SystemMetrics::default()
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
