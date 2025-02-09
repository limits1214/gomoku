use std::sync::{Arc, OnceLock};

use aws_config::Region;
use config::{Config, Environment};
use serde::Deserialize;

pub static APP_CONFIG: OnceLock<Arc<AppConfig>> = OnceLock::new();
#[derive(Debug, Deserialize)]
pub struct Settings {}
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

#[derive(Debug)]
pub struct AppConfig {
    pub settings: Settings,
    pub aws_config: aws_config::SdkConfig,
}
impl AppConfig {
    pub async fn init() {
        let aws_config = make_aws_config().await;
        APP_CONFIG.get_or_init(|| {
            let settings = Settings::new();
            let app_config = Self::new(settings, aws_config);
            Arc::new(app_config)
        });
    }
    fn new(settings: Settings, aws_config: aws_config::SdkConfig) -> Self {
        Self {
            settings,
            aws_config,
        }
    }
}

async fn make_aws_config() -> aws_config::SdkConfig {
    let shared_config = aws_config::from_env()
        .region(Region::new("ap-northeast-2"))
        .load()
        .await;
    shared_config
}
