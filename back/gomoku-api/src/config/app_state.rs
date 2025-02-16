use crate::util;
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub dynamo_client: aws_sdk_dynamodb::Client,
    pub gw_ws_client: aws_sdk_apigatewaymanagement::Client,
}
impl AppState {
    pub async fn new() -> Self {
        let dynamo_client = make_dynamo_client().await;
        let gw_ws_client = make_gw_ws_client().await;
        Self {
            dynamo_client,
            gw_ws_client,
        }
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
    let shared_config = util::config::get_aws_config();
    aws_sdk_dynamodb::Client::new(shared_config)
}
async fn make_gw_ws_client() -> aws_sdk_apigatewaymanagement::Client {
    let shared_config = util::config::get_aws_config();

    let connection_url = &util::config::get_gw_ws_settings().connections_url;
    let api_management_config = aws_sdk_apigatewaymanagement::config::Builder::from(shared_config)
        .endpoint_url(connection_url)
        .build();
    let client = aws_sdk_apigatewaymanagement::Client::from_conf(api_management_config);
    client
}
