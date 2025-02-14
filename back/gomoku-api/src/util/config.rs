use crate::config::app_config::{DynamoSettings, JwtKeys, APP_CONFIG};

// pub fn get_config_app_server_port() -> &'static u32 {
//     &APP_CONFIG.get().unwrap().settings.app.server_port
// }

// pub fn get_config_app_log_filter() -> &'static str {
//     &APP_CONFIG.get().unwrap().settings.app.log_filter
// }

// pub fn get_config_db_url() -> &'static str {
//     &APP_CONFIG.get().unwrap().settings.database.database_url
// }

// pub fn get_config_redis_url() -> &'static str {
//     &APP_CONFIG.get().unwrap().settings.redis.redis_url
// }

pub fn get_config_jwt_access_keys() -> &'static JwtKeys {
    &APP_CONFIG.get().unwrap().jwt_access_keys
}

pub fn get_config_jwt_refresh_keys() -> &'static JwtKeys {
    &APP_CONFIG.get().unwrap().jwt_refresh_keys
}
pub fn get_config_jwt_access_time() -> &'static i64 {
    &APP_CONFIG.get().unwrap().settings.jwt.jwt_access_time
}

pub fn get_config_jwt_refresh_time() -> &'static i64 {
    &APP_CONFIG.get().unwrap().settings.jwt.jwt_refresh_time
}

// pub fn get_config_oauth2() -> &'static Oauth2Settings {
//     &APP_CONFIG.get().unwrap().settings.oauth2
// }

pub fn get_dynamo_settings() -> &'static DynamoSettings {
    &APP_CONFIG.get().unwrap().settings.dynamo
}

pub fn get_aws_config() -> &'static aws_config::SdkConfig {
    &APP_CONFIG.get().unwrap().aws_config
}

pub fn get_cookie_secure() -> &'static bool {
    &APP_CONFIG.get().unwrap().settings.cookie.secure
}

pub fn get_aes128key() -> &'static str {
    &APP_CONFIG.get().unwrap().settings.sec.aes128key
}
