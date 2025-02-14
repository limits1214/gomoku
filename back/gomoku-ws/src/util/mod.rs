// pub async fn ws_get_token(
//     dynamo_client: &aws_sdk_dynamodb::Client,
//     connection_id: String,
// ) -> anyhow::Result<Option<String>> {
//     let ws_conn_pk = format!("WS_CONN#{connection_id}");
//     let ws_conn_sk = format!("INFO");
//     let get_output = dynamo_client
//         .get_item()
//         .table_name(util::dynamo::get_table_name())
//         .key(PK, AttributeValue::S(ws_conn_pk))
//         .key(SK, AttributeValue::S(ws_conn_sk))
//         .send()
//         .await?;

//     let user_id = get_output
//         .item
//         .as_ref()
//         .and_then(
//             |output: &std::collections::HashMap<String, AttributeValue>| output.get("wsToken"),
//         )
//         .and_then(|ws_token| match ws_token {
//             AttributeValue::S(ws_token) => Some(ws_token.to_owned()),
//             _ => None,
//         });
//     Ok(user_id)
// }

use lambda_http::tracing;
use serde_json::{json, Value};

use crate::config::APP_CONFIG;

pub async fn ws_get_token(
    connection_id: &str,
    http_client: reqwest::Client,
) -> anyhow::Result<Option<String>> {
    let base_url = &APP_CONFIG.get().unwrap().settings.api.url;
    let url = format!("{base_url}/ws/token");
    let res = http_client
        .post(url)
        .json(&json!({
            "connectionId": connection_id,
        }))
        .send()
        .await?;
    let j = res.json::<Value>().await?;
    tracing::info!("j: {j:?}");
    let token = j
        .get("data")
        .and_then(|j| j.get("token"))
        .and_then(|token| Some(token.as_str().unwrap_or_default().to_owned()));
    tracing::info!("token: {token:?}");
    Ok(token)
}
