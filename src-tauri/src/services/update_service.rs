use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tauri::{AppHandle, Emitter};

use crate::{errors::AppError, models::update::UpdateInfo};

const REPO_API_BASE: &str = "https://api.github.com/repos/Neko-NF/NekoNeo";
const FORCE_UPDATE_MARKER: &str = "<!-- FORCE_UPDATE -->";

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    prerelease: bool,
    draft: bool,
    body: Option<String>,
    published_at: Option<String>,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize, Clone)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadedUpdate {
    pub version: String,
    pub asset_name: String,
    pub path: String,
    pub sha256: Option<String>,
}

pub struct UpdateService;

impl UpdateService {
    // ── Check ────────────────────────────────────────────────────────

    pub async fn check(
        channel: &str,
        skipped_version: Option<&str>,
    ) -> Result<Option<UpdateInfo>, AppError> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{REPO_API_BASE}/releases"))
            .header("User-Agent", "NekoNeo-Updater")
            .send()
            .await
            .map_err(|e| AppError::UpdateError(e.to_string()))?;

        if !response.status().is_success() {
            return Ok(None); // Silently fail for auto-checks
        }

        let releases: Vec<GithubRelease> = response
            .json()
            .await
            .map_err(|e| AppError::UpdateError(e.to_string()))?;

        let Some(release) = pick_release(channel, &releases) else {
            return Ok(None);
        };

        let version = release.tag_name.trim_start_matches('v').to_string();
        if version == env!("CARGO_PKG_VERSION") {
            return Ok(None);
        }

        if skipped_version.is_some_and(|v| v == version) {
            return Ok(None);
        }

        let mandatory = release
            .body
            .as_deref()
            .is_some_and(|b| b.contains(FORCE_UPDATE_MARKER));

        let selected = select_asset(&release.assets);
        let downloaded = Self::read_downloaded()
            .ok()
            .flatten()
            .is_some_and(|d| d.version == version);

        Ok(Some(UpdateInfo {
            version,
            release_notes: release.body.clone().unwrap_or_default(),
            mandatory,
            channel: channel.to_string(),
            asset_name: selected.as_ref().map(|a| a.name.clone()),
            published_at: release.published_at.clone(),
            downloaded,
        }))
    }

    // ── Download ─────────────────────────────────────────────────────

    pub async fn download(app: &AppHandle, channel: &str) -> Result<String, AppError> {
        let client = reqwest::Client::new();
        let releases: Vec<GithubRelease> = client
            .get(format!("{REPO_API_BASE}/releases"))
            .header("User-Agent", "NekoNeo-Updater")
            .send()
            .await
            .map_err(|e| AppError::UpdateError(e.to_string()))?
            .json()
            .await
            .map_err(|e| AppError::UpdateError(e.to_string()))?;

        let release =
            pick_release(channel, &releases)
                .ok_or_else(|| AppError::UpdateError("no release found".into()))?;
        let asset =
            select_asset(&release.assets)
                .ok_or_else(|| AppError::UpdateError("no desktop asset found".into()))?;

        let mut response = client
            .get(&asset.browser_download_url)
            .header("User-Agent", "NekoNeo-Updater")
            .send()
            .await
            .map_err(|e| AppError::UpdateError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::UpdateError(format!(
                "HTTP {}",
                response.status().as_u16()
            )));
        }

        let total = response.content_length();
        let mut downloaded_bytes = 0u64;
        let mut hasher = Sha256::new();
        let path = update_dir()?.join(&asset.name);
        let mut file =
            fs::File::create(&path).map_err(|e| AppError::UpdateError(e.to_string()))?;

        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| AppError::UpdateError(e.to_string()))?
        {
            file.write_all(&chunk)
                .map_err(|e| AppError::UpdateError(e.to_string()))?;
            hasher.update(&chunk);
            downloaded_bytes += chunk.len() as u64;

            let percent = total
                .map(|t| ((downloaded_bytes as f64 / t as f64) * 100.0).round() as u64)
                .unwrap_or(0);

            let _ = app.emit(
                "update:progress",
                serde_json::json!({
                    "downloaded": downloaded_bytes,
                    "total": total,
                    "percent": percent,
                    "assetName": asset.name,
                }),
            );
        }

        let sha256 = format!("{:x}", hasher.finalize());
        let version = release.tag_name.trim_start_matches('v').to_string();

        let meta = DownloadedUpdate {
            version: version.clone(),
            asset_name: asset.name.clone(),
            path: path.display().to_string(),
            sha256: Some(sha256),
        };
        Self::write_downloaded(&meta)?;

        let _ = app.emit(
            "notify",
            serde_json::json!({
                "level": "success",
                "title": "更新已下载",
                "body": format!("{} 已就绪，下次启动应用时安装。", version),
            }),
        );

        Ok(meta.path)
    }

    // ── Install ──────────────────────────────────────────────────────

    pub fn install(app: &AppHandle) -> Result<(), AppError> {
        let meta = Self::read_downloaded()?
            .ok_or_else(|| AppError::UpdateError("没有已下载的更新".into()))?;

        let path = PathBuf::from(&meta.path);
        if !path.exists() {
            return Err(AppError::UpdateError("更新文件丢失".into()));
        }

        // Verify SHA256 if available
        if let Some(ref expected) = meta.sha256 {
            let actual = sha256_file(&path)?;
            if !actual.eq_ignore_ascii_case(expected) {
                return Err(AppError::UpdateError("SHA256 校验失败，请重新下载".into()));
            }
        }

        // Launch installer and schedule relaunch
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        match ext.as_str() {
            "msi" => {
                Command::new("msiexec")
                    .args(["/i", &path.display().to_string(), "/quiet", "/norestart"])
                    .spawn()
                    .map_err(|e| AppError::UpdateError(e.to_string()))?;
            }
            _ => {
                Command::new(&path)
                    .args(["/S"])
                    .spawn()
                    .map_err(|e| AppError::UpdateError(e.to_string()))?;
            }
        }

        // Clean up metadata so we don't re-install on next launch
        let _ = fs::remove_file(metadata_path().unwrap_or_default());

        app.exit(0);
        Ok(())
    }

    // ── Pending install (startup check) ──────────────────────────────

    pub fn has_pending_install() -> bool {
        Self::read_downloaded().ok().flatten().is_some()
    }

    // ── Persistence ──────────────────────────────────────────────────

    fn read_downloaded() -> Result<Option<DownloadedUpdate>, AppError> {
        let path = metadata_path()?;
        if !path.exists() {
            return Ok(None);
        }
        let data =
            fs::read_to_string(&path).map_err(|e| AppError::UpdateError(e.to_string()))?;
        let meta: DownloadedUpdate =
            serde_json::from_str(&data).map_err(|e| AppError::UpdateError(e.to_string()))?;
        Ok(Some(meta))
    }

    fn write_downloaded(meta: &DownloadedUpdate) -> Result<(), AppError> {
        let path = metadata_path()?;
        let data =
            serde_json::to_string_pretty(meta).map_err(|e| AppError::UpdateError(e.to_string()))?;
        fs::write(&path, data).map_err(|e| AppError::UpdateError(e.to_string()))?;
        Ok(())
    }
}

// ── Path helpers ─────────────────────────────────────────────────────

fn update_dir() -> Result<PathBuf, AppError> {
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let dir = base.join(".local").join("updates");
    fs::create_dir_all(&dir).map_err(|e| AppError::UpdateError(e.to_string()))?;
    Ok(dir)
}

fn metadata_path() -> Result<PathBuf, AppError> {
    Ok(update_dir()?.join("latest-download.json"))
}

// ── GitHub release helpers ───────────────────────────────────────────

fn pick_release<'a>(channel: &str, releases: &'a [GithubRelease]) -> Option<&'a GithubRelease> {
    releases.iter().find(|r| {
        if r.draft {
            return false;
        }
        match channel {
            "beta" => true,
            _ => !r.prerelease,
        }
    })
}

fn select_asset(assets: &[GithubAsset]) -> Option<GithubAsset> {
    assets
        .iter()
        .find(|a| a.name.ends_with(".msi"))
        .or_else(|| assets.iter().find(|a| a.name.ends_with(".exe")))
        .cloned()
}

fn sha256_file(path: &Path) -> Result<String, AppError> {
    let data = fs::read(path).map_err(|e| AppError::UpdateError(e.to_string()))?;
    Ok(format!("{:x}", Sha256::digest(&data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_release_prefers_stable_for_stable_channel() {
        let releases = vec![
            GithubRelease {
                tag_name: "v2.0.0-beta.1".into(),
                prerelease: true,
                draft: false,
                body: None,
                published_at: None,
                assets: vec![],
            },
            GithubRelease {
                tag_name: "v2.0.0".into(),
                prerelease: false,
                draft: false,
                body: None,
                published_at: None,
                assets: vec![],
            },
        ];
        assert_eq!(pick_release("stable", &releases).unwrap().tag_name, "v2.0.0");
    }
}
