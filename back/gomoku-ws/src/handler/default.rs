use std::time::Instant;

use aws_sdk_dynamodb::types::AttributeValue;
use futures::future::join_all;
use lambda_http::tracing::{self, info};
use serde_json::{json, Value};

use crate::{
    config::APP_CONFIG,
    util::{self, ws_get_token},
};

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

pub async fn room_chat(
    dynamo_client: &aws_sdk_dynamodb::Client,
    gw_ws_client: &aws_sdk_apigatewaymanagement::Client,
    connection_id: &str,
    http_client: reqwest::Client,
    msg: &str,
    room_id: &str,
) -> anyhow::Result<()> {
    let start = Instant::now();

    // get all topoics
    let ws_topic_pk = format!("WS_TOPIC#ROOM#{room_id}");
    let ws_topic_sk = format!("WS_CONN#");

    let output = dynamo_client
        .query()
        .table_name(util::dynamo::get_table_name())
        .key_condition_expression("PK = :PK AND begins_with(SK, :SK)")
        .expression_attribute_values(":PK", AttributeValue::S(ws_topic_pk))
        .expression_attribute_values(":SK", AttributeValue::S(ws_topic_sk))
        .send()
        .await?;

    let items = output.items.unwrap_or_default();
    let futures: Vec<_> = items
        .into_iter()
        .map(|item| {
            let connection_id = item
                .get("connectionId")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string();
            let msg = msg;
            let gw_ws_client = gw_ws_client.clone();

            async move {
                let res = gw_ws_client
                    .post_to_connection()
                    .connection_id(&connection_id)
                    .data(util::dynamo::json_value_to_blob(json!({"msg": msg})).unwrap())
                    .send()
                    .await;

                if let Err(err) = res {
                    tracing::error!("post to connection err: {err:?}");
                }
            }
        })
        .collect();
    join_all(futures).await;

    let duration = start.elapsed();
    info!("room_chat 실행 시간: {:.2?}", duration);
    Ok(())
}
