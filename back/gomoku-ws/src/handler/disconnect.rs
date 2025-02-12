use lambda_http::tracing;
use serde_json::{json, Value};

use crate::config::APP_CONFIG;

pub async fn ws_disconnect(
    connection_id: &str,
    http_client: reqwest::Client,
) -> anyhow::Result<()> {
    // get jwt by connection_id
    let base_url = &APP_CONFIG.get().unwrap().settings.api.url;
    let url = format!("{base_url}/ws/disconnect");
    let res = http_client
        .post(url)
        .json(&json!({
            "connectionId": connection_id,
        }))
        .send()
        .await?;
    let j = res.json::<Value>().await?;

    Ok(())
}
