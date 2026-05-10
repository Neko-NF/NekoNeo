use std::{
    sync::Arc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Emitter};
use tokio::{
    sync::{oneshot, Mutex},
    time::{sleep, Duration},
};

use crate::models::service::{ServiceStatus, TickResult};
use crate::services::system_info::SystemInfo;

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
    tick_count: u64,
    cancel_tx: Option<oneshot::Sender<()>>,
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
                tick_count: 0,
                cancel_tx: None,
            })),
        }
    }

    pub async fn start(&self, app: AppHandle) -> ServiceStatus {
        let mut inner = self.inner.lock().await;
        if inner.running {
            return self.status_from_inner(&inner);
        }

        inner.running = true;
        inner.started_at = Some(Instant::now());
        inner.tick_count = 0;

        let (cancel_tx, cancel_rx) = oneshot::channel();
        inner.cancel_tx = Some(cancel_tx);

        let status = self.status_from_inner(&inner);
        drop(inner);

        self.emit_status(&app, &status);
        self.emit_log(&app, "success", "上报服务已启动");

        let reporter = self.clone();
        tokio::spawn(async move {
            reporter.run_loop(app, cancel_rx).await;
        });

        status
    }

    pub async fn stop(&self, app: &AppHandle) -> ServiceStatus {
        let mut inner = self.inner.lock().await;
        inner.running = false;
        if let Some(cancel_tx) = inner.cancel_tx.take() {
            let _ = cancel_tx.send(());
        }

        let status = self.status_from_inner(&inner);
        drop(inner);

        self.emit_status(app, &status);
        self.emit_log(app, "warn", "上报服务已停止");

        status
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

    async fn run_loop(&self, app: AppHandle, mut cancel_rx: oneshot::Receiver<()>) {
        loop {
            tokio::select! {
                _ = &mut cancel_rx => {
                    break;
                }
                _ = sleep(Duration::from_secs(5)) => {
                    let tick = self.generate_tick(&app).await;
                    let _ = app.emit("service:tick", &tick);
                    self.emit_log(&app, "info", &format!("完成第 {} 次状态上报", self.current_tick_count().await));
                }
            }
        }
    }

    async fn generate_tick(&self, app: &AppHandle) -> TickResult {
        let mut inner = self.inner.lock().await;
        inner.tick_count += 1;
        let tick_count = inner.tick_count;
        drop(inner);

        let result = SystemInfo::create_tick_result(tick_count).await;
        let metrics = SystemInfo::get_metrics().await;

        let _ = app.emit("metrics:update", &metrics);

        let mut inner = self.inner.lock().await;
        inner.last_result = Some(result.clone());
        result
    }

    async fn current_tick_count(&self) -> u64 {
        let inner = self.inner.lock().await;
        inner.tick_count
    }

    fn emit_status(&self, app: &AppHandle, status: &ServiceStatus) {
        let _ = app.emit("service:status", status);
    }

    fn emit_log(&self, app: &AppHandle, level: &str, message: &str) {
        let _ = app.emit(
            "service:log",
            serde_json::json!({
                "level": level,
                "message": message,
                "time": now_iso_like(),
            }),
        );
    }
}

pub(crate) fn now_iso_like() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or_default();
    format!("{ts}")
}
