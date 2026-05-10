use std::{
    sync::Arc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Emitter};
use tokio::{
    sync::{oneshot, Mutex},
    time::{sleep, Duration},
};

use crate::{
    errors::AppError,
    models::{
        config::AppConfig,
        service::{ServiceStatus, TickResult},
    },
    services::{
        api_client::ApiClient, config_store::ConfigStore, screenshot::ScreenshotService,
        system_info::SystemInfo,
    },
};

#[derive(Clone)]
pub struct StatusReporter {
    inner: Arc<Mutex<ReporterInner>>,
}

struct ReporterInner {
    running: bool,
    started_at: Option<Instant>,
    consecutive_failures: u32,
    auto_restart_count: u32,
    last_success_at: Option<Instant>,
    last_result: Option<TickResult>,
    tick_count: u64,
    cancel_tx: Option<oneshot::Sender<()>>,
    config: Option<Arc<Mutex<AppConfig>>>,
    api_client: Option<Arc<Mutex<ApiClient>>>,
}

impl StatusReporter {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ReporterInner {
                running: false,
                started_at: None,
                consecutive_failures: 0,
                auto_restart_count: 0,
                last_success_at: None,
                last_result: None,
                tick_count: 0,
                cancel_tx: None,
                config: None,
                api_client: None,
            })),
        }
    }

    pub async fn start(
        &self,
        app: AppHandle,
        config: Arc<Mutex<AppConfig>>,
        api_client: Arc<Mutex<ApiClient>>,
    ) -> ServiceStatus {
        let mut inner = self.inner.lock().await;
        if inner.running {
            return self.status_from_inner(&inner);
        }

        inner.running = true;
        inner.started_at = Some(Instant::now());
        inner.tick_count = 0;
        inner.consecutive_failures = 0;
        inner.last_success_at = Some(Instant::now());
        inner.config = Some(config);
        inner.api_client = Some(api_client);

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
        inner.last_success_at = None;

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
            uptime_sec: inner
                .started_at
                .map(|value| value.elapsed().as_secs())
                .unwrap_or(0),
            consecutive_failures: inner.consecutive_failures,
            auto_restart_count: inner.auto_restart_count,
        }
    }

    async fn run_loop(&self, app: AppHandle, mut cancel_rx: oneshot::Receiver<()>) {
        loop {
            let interval = self.current_interval().await;

            tokio::select! {
                _ = &mut cancel_rx => {
                    break;
                }
                _ = sleep(Duration::from_secs(interval)) => {
                    let tick = self.generate_tick(&app).await;
                    let _ = app.emit("service:tick", &tick);
                }
            }
        }
    }

    async fn current_interval(&self) -> u64 {
        let config = {
            let inner = self.inner.lock().await;
            inner.config.clone()
        };

        if let Some(config) = config {
            let report_interval = config.lock().await.report_interval;
            return report_interval.max(5);
        }

        5
    }

    async fn generate_tick(&self, app: &AppHandle) -> TickResult {
        let mut inner = self.inner.lock().await;
        inner.tick_count += 1;
        let tick_count = inner.tick_count;
        let config = inner.config.clone();
        let api_client = inner.api_client.clone();
        drop(inner);

        let config_snapshot = match config {
            Some(config) => config.lock().await.clone(),
            None => AppConfig::default(),
        };

        let mut result = SystemInfo::create_tick_result(tick_count, &config_snapshot).await;
        let metrics = SystemInfo::get_metrics().await;

        let _ = app.emit("metrics:update", &metrics);

        match ScreenshotService::capture_if_due(tick_count, &config_snapshot).await {
            Ok(Some(capture)) => {
                result.has_screenshot = true;
                result.screenshot_blurred = capture.blurred;
                result.screenshot_path = Some(capture.path.clone());
                self.emit_log(app, "info", &format!("截图已保存: {}", capture.path));
            }
            Ok(None) => {}
            Err(error) => {
                self.emit_log(app, "warn", &format!("截图采集失败: {error}"));
            }
        }

        if let Some(api_client) = api_client {
            let client = api_client.lock().await.clone();
            match client.report_status_v2(&result).await {
                Ok(outcome) => {
                    self.emit_log(app, "success", "状态已上报到服务端");
                    self.reset_failures().await;
                    if outcome.takeover_occurred {
                        self.emit_key_status(
                            app,
                            "TAKEOVER_SUCCESS",
                            outcome.message.as_deref().unwrap_or("当前密钥已被新设备接管"),
                        );
                    }
                }
                Err(error) => {
                    self.bump_failures().await;
                    self.emit_log(app, "error", &format!("状态上报失败: {error}"));

                    if Self::is_fatal_report_error(&error) {
                        self.handle_report_error(app, error).await;
                    } else {
                        self.maybe_trigger_watchdog_restart(app, &config_snapshot).await;
                    }
                }
            }
        }

        let mut inner = self.inner.lock().await;
        inner.last_result = Some(result.clone());
        result
    }

    async fn reset_failures(&self) {
        let mut inner = self.inner.lock().await;
        inner.consecutive_failures = 0;
        inner.last_success_at = Some(Instant::now());
    }

    async fn bump_failures(&self) {
        let mut inner = self.inner.lock().await;
        inner.consecutive_failures += 1;
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

    fn emit_key_status(&self, app: &AppHandle, code: &str, message: &str) {
        let _ = app.emit(
            "service:key_status",
            serde_json::json!({
                "code": code,
                "message": message,
            }),
        );
    }

    async fn handle_report_error(&self, app: &AppHandle, error: AppError) {
        match error {
            AppError::InvalidKey(message) => {
                self.emit_key_status(app, "INVALID_KEY", &message);
                self.clear_device_key().await;
                self.stop(app).await;
            }
            AppError::KeyRevoked(message) => {
                self.emit_key_status(app, "KEY_REVOKED", &message);
                self.clear_device_key().await;
                self.stop(app).await;
            }
            AppError::DeviceNotFound(message) => {
                self.emit_key_status(app, "DEVICE_NOT_FOUND", &message);
                self.clear_device_key().await;
                self.stop(app).await;
            }
            AppError::TakeoverRequired(message) => {
                self.emit_key_status(app, "TAKEOVER_REQUIRED", &message);
                self.stop(app).await;
            }
            _ => {}
        }
    }

    fn is_fatal_report_error(error: &AppError) -> bool {
        matches!(
            error,
            AppError::InvalidKey(_)
                | AppError::KeyRevoked(_)
                | AppError::DeviceNotFound(_)
                | AppError::TakeoverRequired(_)
        )
    }

    async fn maybe_trigger_watchdog_restart(&self, app: &AppHandle, config: &AppConfig) {
        let snapshot = {
            let inner = self.inner.lock().await;
            (
                inner.running,
                inner.consecutive_failures,
                inner.auto_restart_count,
                inner.last_success_at.map(|value| value.elapsed().as_secs()).unwrap_or_default(),
                inner.cancel_tx.is_some(),
            )
        };

        if !snapshot.0 || !snapshot.4 {
            return;
        }

        if !Self::should_auto_restart(config, snapshot.1, snapshot.2, snapshot.3) {
            if config.enable_auto_restart && snapshot.2 >= config.max_restarts {
                self.emit_log(app, "warn", "看门狗已达到最大自动恢复次数，未再执行重启");
            }
            return;
        }

        let restart_delay = config.restart_interval_sec.max(1);

        {
            let mut inner = self.inner.lock().await;
            if !inner.running || inner.cancel_tx.is_none() {
                return;
            }

            inner.running = false;
            inner.started_at = None;
            inner.consecutive_failures = 0;
            inner.auto_restart_count += 1;
        }

        let paused_status = self.status().await;
        self.emit_status(app, &paused_status);
        self.emit_log(
            app,
            "warn",
            &format!("看门狗触发自动恢复，{restart_delay} 秒后尝试重启上报服务"),
        );

        sleep(Duration::from_secs(restart_delay)).await;

        {
            let mut inner = self.inner.lock().await;
            if inner.cancel_tx.is_none() {
                return;
            }

            inner.running = true;
            inner.started_at = Some(Instant::now());
            inner.last_success_at = Some(Instant::now());
        }

        let resumed_status = self.status().await;
        self.emit_status(app, &resumed_status);
        self.emit_log(app, "success", "看门狗已重新启动上报服务");
    }

    fn should_auto_restart(
        config: &AppConfig,
        consecutive_failures: u32,
        auto_restart_count: u32,
        seconds_since_success: u64,
    ) -> bool {
        const FAILURE_THRESHOLD: u32 = 3;

        if !config.enable_auto_restart || auto_restart_count >= config.max_restarts {
            return false;
        }

        consecutive_failures >= FAILURE_THRESHOLD
            || (config.watchdog_timeout_sec > 0 && seconds_since_success >= config.watchdog_timeout_sec)
    }

    async fn clear_device_key(&self) {
        let config = {
            let inner = self.inner.lock().await;
            inner.config.clone()
        };

        if let Some(config) = config {
            let mut config = config.lock().await;
            config.device_key.clear();
            let _ = ConfigStore::save(&config);
        }
    }
}

pub(crate) fn now_iso_like() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or_default();
    format!("{ts}")
}

#[cfg(test)]
mod tests {
    use super::StatusReporter;
    use crate::models::config::AppConfig;

    #[test]
    fn should_restart_when_failures_reach_threshold() {
        let config = AppConfig::default();
        assert!(StatusReporter::should_auto_restart(&config, 3, 0, 10));
    }

    #[test]
    fn should_restart_when_watchdog_timeout_is_hit() {
        let config = AppConfig::default();
        assert!(StatusReporter::should_auto_restart(
            &config,
            1,
            0,
            config.watchdog_timeout_sec,
        ));
    }

    #[test]
    fn should_not_restart_when_limit_is_exhausted() {
        let config = AppConfig::default();
        assert!(!StatusReporter::should_auto_restart(
            &config,
            5,
            config.max_restarts,
            config.watchdog_timeout_sec + 1,
        ));
    }
}
