use std::sync::Arc;

use axum::extract::FromRef;

use super::app_config::APP_CONFIG;

#[derive(Debug, Clone)]
pub struct AppState {
    pub dynamo_client: aws_sdk_dynamodb::Client,
}
impl AppState {
    pub async fn new() -> Self {
        let dynamo_client = make_dynamo_client().await;
        Self { dynamo_client }
    }
}
pub struct ArcAppState(pub Arc<AppState>);
impl ArcAppState {
    pub async fn new() -> Self {
        Self(Arc::new(AppState::new().await))
    }
}
impl Clone for ArcAppState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl FromRef<ArcAppState> for aws_sdk_dynamodb::Client {
    fn from_ref(input: &ArcAppState) -> Self {
        input.0.dynamo_client.clone()
    }
}

async fn make_dynamo_client() -> aws_sdk_dynamodb::Client {
    let shared_config = &APP_CONFIG.get().unwrap().aws_config;
    aws_sdk_dynamodb::Client::new(shared_config)
}
