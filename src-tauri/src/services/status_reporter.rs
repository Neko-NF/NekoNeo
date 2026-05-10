use std::{
    sync::Arc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use tokio::sync::Mutex;

use crate::models::service::{ServiceStatus, TickResult};

#[derive(Clone)]
pub struct StatusReporter {
    inner: Arc<Mutex<ReporterInner>>,
}

struct ReporterInner {
    running: bool,
    started_at: Option<Instant>,
    consecutive_failures: u32,
    auto_restart_count: u32,
    last_result: Option<TickResult>,
}

impl StatusReporter {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ReporterInner {
                running: false,
                started_at: None,
                consecutive_failures: 0,
                auto_restart_count: 0,
                last_result: None,
            })),
        }
    }

    pub async fn start(&self) -> ServiceStatus {
        let mut inner = self.inner.lock().await;
        inner.running = true;
        inner.started_at = Some(Instant::now());
        inner.last_result = Some(TickResult {
            success: true,
            timestamp: now_iso_like(),
            app_name: "bootstrap".into(),
            battery_level: 85,
            is_charging: true,
            has_battery: true,
            user_status: "online".into(),
            has_screenshot: true,
            screenshot_blurred: false,
            error: None,
        });
        self.status_from_inner(&inner)
    }

    pub async fn stop(&self) -> ServiceStatus {
        let mut inner = self.inner.lock().await;
        inner.running = false;
        self.status_from_inner(&inner)
    }

    pub async fn status(&self) -> ServiceStatus {
        let inner = self.inner.lock().await;
        self.status_from_inner(&inner)
    }

    pub async fn last_result(&self) -> Option<TickResult> {
        let inner = self.inner.lock().await;
        inner.last_result.clone()
    }

    fn status_from_inner(&self, inner: &ReporterInner) -> ServiceStatus {
        ServiceStatus {
            running: inner.running,
            uptime_sec: inner.started_at.map(|value| value.elapsed().as_secs()).unwrap_or(0),
            consecutive_failures: inner.consecutive_failures,
            auto_restart_count: inner.auto_restart_count,
        }
    }
}

fn now_iso_like() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or_default();
    format!("{ts}")
}
