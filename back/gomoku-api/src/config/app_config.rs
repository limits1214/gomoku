use std::sync::{Arc, OnceLock};

use aws_config::Region;
use config::{Config, Environment};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub static APP_CONFIG: OnceLock<Arc<AppConfig>> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct DynamoSettings {
    pub table_name: String,
}
#[derive(Debug, Deserialize)]
pub struct JwtSettings {
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_time: i64,
    pub jwt_refresh_time: i64,
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub dynamo: DynamoSettings,
    pub jwt: JwtSettings,
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

#[derive(Debug)]
pub struct AppConfig {
    pub settings: Settings,
    pub aws_config: aws_config::SdkConfig,
    pub jwt_access_keys: JwtKeys,
    pub jwt_refresh_keys: JwtKeys,
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
        let jwt_access_keys = JwtKeys::new(&settings.jwt.jwt_access_secret);
        let jwt_refresh_keys = JwtKeys::new(&settings.jwt.jwt_refresh_secret);
        Self {
            settings,
            aws_config,
            jwt_access_keys,
            jwt_refresh_keys,
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

#[derive(Clone)]
pub struct JwtKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}
impl std::fmt::Debug for JwtKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtKeys")
            .field("encoding", &"EncodingKey(...)")
            .field("decoding", &"DecodingKey(...)")
            .finish()
    }
}
impl JwtKeys {
    pub fn new(secret: &str) -> Self {
        let bsecret = secret.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(bsecret),
            decoding: DecodingKey::from_secret(bsecret),
        }
    }

    pub fn encode<T>(&self, claims: &T) -> Result<String, jsonwebtoken::errors::Error>
    where
        T: Serialize,
    {
        jsonwebtoken::encode(&Header::default(), claims, &self.encoding)
    }

    pub fn decode<T>(&self, token: &str) -> Result<TokenData<T>, jsonwebtoken::errors::Error>
    where
        T: DeserializeOwned,
    {
        jsonwebtoken::decode::<T>(token, &self.decoding, &Validation::default())
    }
}
