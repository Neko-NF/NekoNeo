use std::{env, path::PathBuf};

use winreg::{enums::*, RegKey};

use crate::{errors::AppError, models::config::AppConfig};

const RUN_KEY_PATH: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE_NAME: &str = "NekoNeo";

pub struct AutostartService;

impl AutostartService {
    pub fn sync(config: &AppConfig) -> Result<(), AppError> {
        if config.enable_auto_start {
            Self::enable(config)
        } else {
            Self::disable()
        }
    }

    pub fn is_enabled() -> Result<bool, AppError> {
        Ok(Self::current_command()?.is_some())
    }

    pub fn current_command() -> Result<Option<String>, AppError> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let run_key = match hkcu.open_subkey_with_flags(RUN_KEY_PATH, KEY_READ) {
            Ok(key) => key,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(AppError::SystemError(error.to_string())),
        };

        match run_key.get_value(RUN_VALUE_NAME) {
            Ok(value) => Ok(Some(value)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(AppError::SystemError(error.to_string())),
        }
    }

    fn enable(config: &AppConfig) -> Result<(), AppError> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (run_key, _) = hkcu
            .create_subkey(RUN_KEY_PATH)
            .map_err(|error| AppError::SystemError(error.to_string()))?;

        let command = Self::build_command(config)?;
        run_key
            .set_value(RUN_VALUE_NAME, &command)
            .map_err(|error| AppError::SystemError(error.to_string()))?;

        Ok(())
    }

    fn disable() -> Result<(), AppError> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let run_key = match hkcu.open_subkey_with_flags(RUN_KEY_PATH, KEY_SET_VALUE) {
            Ok(key) => key,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(AppError::SystemError(error.to_string())),
        };

        match run_key.delete_value(RUN_VALUE_NAME) {
            Ok(_) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(AppError::SystemError(error.to_string())),
        }
    }

    fn build_command(config: &AppConfig) -> Result<String, AppError> {
        let exe_path = current_exe_path()?;
        let mut args = vec!["--autostart".to_string()];

        if config.minimize_on_auto_start {
            args.push("--minimized".to_string());
        }

        if config.startup_delay_ms > 0 {
            args.push(format!("--startup-delay-ms={}", config.startup_delay_ms));
        }

        Ok(format!("\"{}\" {}", exe_path.display(), args.join(" ")).trim().to_string())
    }
}

fn current_exe_path() -> Result<PathBuf, AppError> {
    env::current_exe().map_err(|error| AppError::SystemError(error.to_string()))
}
