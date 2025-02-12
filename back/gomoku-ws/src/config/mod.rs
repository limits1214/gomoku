use std::sync::{Arc, OnceLock};

use config::{Config, Environment};
use serde::Deserialize;

pub static APP_CONFIG: OnceLock<Arc<AppConfig>> = OnceLock::new();

#[derive(Debug)]
pub struct AppConfig {
    pub settings: Settings,
    // pub aws_config: aws_config::SdkConfig,
}
impl AppConfig {
    pub async fn init() {
        APP_CONFIG.get_or_init(|| {
            let settings = Settings::new();
            // let app_config = Self::new(settings, aws_config);
            Arc::new(Self::new(settings))
        });
    }
    fn new(settings: Settings) -> Self {
        Self { settings }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiSettings {
    pub url: String,
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api: ApiSettings,
}

impl Settings {
    pub fn new() -> Self {
        let builder = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .add_source(config::File::with_name("Settings.local").required(false))
            .add_source(Environment::default().prefix("ENV").separator("__"))
            .build()
            .unwrap();
        builder.try_deserialize().unwrap()
    }
}
