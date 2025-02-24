use std::sync::{Arc, OnceLock};

pub(crate) static APP_CONFIG: OnceLock<Arc<AppConfig>> = OnceLock::new();
#[derive(Debug)]
pub(crate) struct AppConfig {
    // pub settings: Settings,
    // pub aws_config: aws_config::SdkConfig,
    pub table_name: String,
    pub dynamo_client: aws_sdk_dynamodb::Client,
}

pub fn set_lib_config(table_name: String, dynamo_client: aws_sdk_dynamodb::Client) {
    let app_config = AppConfig {
        table_name,
        dynamo_client,
    };
    APP_CONFIG.get_or_init(|| Arc::new(app_config));
}
