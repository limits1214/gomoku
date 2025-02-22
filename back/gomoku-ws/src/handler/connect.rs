use lambda_http::{aws_lambda_events::query_map::QueryMap, tracing};
use serde_json::{json, Value};

use crate::config::APP_CONFIG;

pub async fn ws_connect(
    qm: QueryMap,
    connection_id: &str,
    http_client: reqwest::Client,
) -> anyhow::Result<bool> {
    let token = qm.first("token");
    let Some(token) = token else {
        tracing::error!("qm token is null");
        return Ok(false);
    };

    let base_url = &APP_CONFIG.get().unwrap().settings.api.url;
    let url = format!("{base_url}/ws/temptoken/verify");
    let res = http_client
        .post(url)
        .json(&json!({
            "token": token,
            "connectionId": connection_id,
        }))
        .send()
        .await?;

    if !res.status().is_success() {
        tracing::error!("verify fail");
        return Ok(false);
    }
    Ok(true)
}
