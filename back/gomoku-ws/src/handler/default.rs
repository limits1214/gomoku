use lambda_http::tracing;
use serde_json::{json, Value};

use crate::{config::APP_CONFIG, util::ws_get_token};

pub async fn topic_subscribe(
    connection_id: &str,
    http_client: reqwest::Client,
    topic: &str,
) -> anyhow::Result<()> {
    let token = ws_get_token(connection_id, http_client.clone()).await?;
    let Some(token) = token else {
        tracing::warn!("topic_subscribe token is null");
        return Ok(());
    };

    let base_url = &APP_CONFIG.get().unwrap().settings.api.url;
    let url = format!("{base_url}/ws/topic/subscribe");
    let res = http_client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
            "topic": topic,
        }))
        .send()
        .await?;
    Ok(())
}

pub async fn topic_unsubscribe(
    connection_id: &str,
    http_client: reqwest::Client,
    topic: &str,
) -> anyhow::Result<()> {
    let token = ws_get_token(connection_id, http_client.clone()).await?;
    let Some(token) = token else {
        tracing::warn!("topic_unsubscribe token is null");
        return Ok(());
    };

    let base_url = &APP_CONFIG.get().unwrap().settings.api.url;
    let url = format!("{base_url}/ws/topic/unsubscribe");
    let res = http_client
        .post(url)
        .bearer_auth(token)
        .json(&json!({
            "topic": topic,
        }))
        .send()
        .await?;
    Ok(())
}
