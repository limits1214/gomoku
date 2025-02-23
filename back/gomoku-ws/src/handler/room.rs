use chrono::Utc;
use lambda_http::tracing::{self, info};
use serde_json::json;

use crate::repository;

use super::topic::topic_post;

pub async fn room_chat(
    dynamo_client: &aws_sdk_dynamodb::Client,
    gw_ws_client: &aws_sdk_apigatewaymanagement::Client,
    connection_id: &str,
    msg: &str,
    room_id: &str,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let time = now.to_rfc3339();
    let user_info = repository::user_info_by_connection_id(dynamo_client, connection_id).await?;
    let topic_msg = if let Some(user_info) = user_info {
        let nick_name = user_info.get("nickName").unwrap().as_s().unwrap();
        json!({"msg": msg, "time": time, "nickName": nick_name})
    } else {
        json!({"msg": msg, "time": time})
    };

    // post msg
    let ws_topic_pk = format!("WS_TOPIC#ROOM#{room_id}");
    topic_post(&dynamo_client, &gw_ws_client, &ws_topic_pk, topic_msg).await?;
    Ok(())
}
