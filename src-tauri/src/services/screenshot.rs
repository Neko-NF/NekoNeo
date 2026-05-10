use std::{
    fs,
    path::{Path, PathBuf},
};

use screenshots::{image::imageops, Screen};

use crate::{
    errors::AppError,
    models::{
        config::{AppConfig, IncognitoScope},
        screenshot::LatestScreenshot,
    },
};

use super::system_info::SystemInfo;

#[derive(Debug, Clone)]
pub struct ScreenshotCapture {
    pub path: String,
    pub blurred: bool,
    pub captured_at: String,
}

pub struct ScreenshotService;

impl ScreenshotService {
    pub async fn capture_if_due(
        tick_count: u64,
        config: &AppConfig,
    ) -> Result<Option<ScreenshotCapture>, AppError> {
        if !should_capture_on_tick(tick_count, config.report_interval, config) {
            return Ok(None);
        }

        Self::capture_now(config).map(Some)
    }

    pub fn capture_now(config: &AppConfig) -> Result<ScreenshotCapture, AppError> {
        let screen = Screen::all()
            .map_err(|error| AppError::SystemError(error.to_string()))?
            .into_iter()
            .next()
            .ok_or_else(|| AppError::SystemError("未检测到可用屏幕".into()))?;

        let mut image = screen
            .capture()
            .map_err(|error| AppError::SystemError(error.to_string()))?;

        let blurred = should_blur_image(config);
        if blurred {
            image = imageops::blur(&image, 12.0);
        }

        let output_path = latest_screenshot_path();
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| AppError::SystemError(error.to_string()))?;
        }

        image
            .save(&output_path)
            .map_err(|error| AppError::SystemError(error.to_string()))?;

        let capture = ScreenshotCapture {
            path: output_path.display().to_string(),
            blurred,
            captured_at: crate::services::status_reporter::now_iso_like(),
        };

        save_metadata(&capture)?;

        Ok(capture)
    }

    pub fn latest() -> Result<Option<LatestScreenshot>, AppError> {
        let metadata_path = latest_metadata_path();
        if !metadata_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(metadata_path)
            .map_err(|error| AppError::SystemError(error.to_string()))?;
        let metadata = serde_json::from_str::<LatestScreenshot>(&content)
            .map_err(|error| AppError::SystemError(error.to_string()))?;

        if !Path::new(&metadata.path).exists() {
            return Ok(None);
        }

        Ok(Some(metadata))
    }
}

fn save_metadata(capture: &ScreenshotCapture) -> Result<(), AppError> {
    let metadata = LatestScreenshot {
        path: capture.path.clone(),
        blurred: capture.blurred,
        captured_at: capture.captured_at.clone(),
        data_url: None,
    };
    let content = serde_json::to_string_pretty(&metadata)
        .map_err(|error| AppError::SystemError(error.to_string()))?;

    fs::write(latest_metadata_path(), content)
        .map_err(|error| AppError::SystemError(error.to_string()))
}

fn latest_screenshot_path() -> PathBuf {
    screenshot_dir().join("latest.png")
}

fn latest_metadata_path() -> PathBuf {
    screenshot_dir().join("latest.json")
}

fn screenshot_dir() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let base = cwd.canonicalize().unwrap_or(cwd);
    base.join(".local").join("screenshots")
}

fn should_blur_image(config: &AppConfig) -> bool {
    if config.blur_all_screenshots {
        return true;
    }

    if !config.enable_incognito {
        return false;
    }

    if !matches!(config.incognito_scope, IncognitoScope::Screenshot | IncognitoScope::Both) {
        return false;
    }

    SystemInfo::should_blur_screenshot(config)
}

pub fn should_capture_on_tick(tick_count: u64, report_interval: u64, config: &AppConfig) -> bool {
    if !config.enable_screenshot {
        return false;
    }

    if config.sync_screenshot_interval {
        return true;
    }

    let report_interval = report_interval.max(1);
    let screenshot_interval = config.screenshot_interval.max(report_interval);
    let every_n_ticks = screenshot_interval.div_ceil(report_interval).max(1);

    tick_count % every_n_ticks == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_capture_on_tick_when_synced() {
        let config = AppConfig::default();
        assert!(should_capture_on_tick(1, 30, &config));
        assert!(should_capture_on_tick(5, 30, &config));
    }

    #[test]
    fn test_should_capture_on_tick_when_independent_interval() {
        let mut config = AppConfig::default();
        config.sync_screenshot_interval = false;
        config.report_interval = 30;
        config.screenshot_interval = 90;

        assert!(!should_capture_on_tick(1, 30, &config));
        assert!(!should_capture_on_tick(2, 30, &config));
        assert!(should_capture_on_tick(3, 30, &config));
    }
}
