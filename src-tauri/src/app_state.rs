use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    models::config::AppConfig,
    services::{
        api_client::ApiClient, config_store::ConfigStore, status_reporter::StatusReporter,
    },
};

pub struct AppState {
    pub config: Arc<Mutex<AppConfig>>,
    pub reporter: StatusReporter,
    pub api_client: Arc<Mutex<ApiClient>>,
}

impl AppState {
    pub fn new() -> Self {
        let config = ConfigStore::load();
        let api_client = ApiClient::new(config.active_server_url(), config.device_key.clone());

        Self {
            config: Arc::new(Mutex::new(config)),
            reporter: StatusReporter::new(),
            api_client: Arc::new(Mutex::new(api_client)),
        }
    }
}
