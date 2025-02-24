use chrono::Local;
use lambda_http::{aws_lambda_events::query_map::QueryMap, tracing};
use serde_json::{json, Value};

use crate::{
    config::APP_CONFIG,
    util::{self, dynamo::DynamoMapHelper},
};

// return true: connection ok
// return false: no connect
pub async fn ws_connect(
    dynamo_client: &aws_sdk_dynamodb::Client,
    qm: QueryMap,
    connection_id: &str,
    http_client: &reqwest::Client,
) -> anyhow::Result<bool> {
    let token = qm.first("token");
    let Some(token) = token else {
        tracing::error!("qm token is null");
        return Ok(false);
    };

    // token 검증
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

    let j = res.json::<Value>().await?;
    let sub = j
        .get("data")
        .and_then(|j| j.get("sub"))
        .and_then(|sub| sub.as_str());
    let Some(sub) = sub else { return Ok(false) };

    // Empty 면 user 가 아닌 사용자
    // WS_CONN#{}, USER>WS_CONN{}
    if sub == "Empty" {
        gomoku_lib::service::put_empty_conn(connection_id).await?;
        // let ws_conn_pk = format!("WS_CONN#{connection_id}");
        // let ws_conn_sk = format!("INFO");
        // let time = Local::now().to_rfc3339();
        // let map = DynamoMapHelper::new()
        //     .insert_pk(ws_conn_pk)
        //     .insert_sk(ws_conn_sk)
        //     .insert_attr_s("createdAt", &time)
        //     .build();

        // let _output = dynamo_client
        //     .put_item()
        //     .table_name(util::dynamo::get_table_name())
        //     .set_item(Some(map))
        //     .send()
        //     .await?;
    } else {
        let user_id = sub;
        gomoku_lib::service::put_conn(connection_id, user_id).await?;
        // let ws_conn_pk = format!("WS_CONN#{connection_id}");
        // let ws_conn_sk = format!("INFO");
        // let time = Local::now().to_rfc3339();
        // let map = DynamoMapHelper::new()
        //     .insert_pk(ws_conn_pk)
        //     .insert_sk(ws_conn_sk)
        //     .insert_attr_s("createdAt", &time)
        //     .insert_attr_s("userId", &user_id)
        //     .build();

        // let _output = dynamo_client
        //     .put_item()
        //     .table_name(util::dynamo::get_table_name())
        //     .set_item(Some(map))
        //     .send()
        //     .await?;

        // let user_pk = format!("USER#{user_id}");
        // let user_sk = format!("WS_CONN#{connection_id}");
        // let map = DynamoMapHelper::new()
        //     .insert_pk(user_pk)
        //     .insert_sk(user_sk)
        //     .insert_attr_s("createdAt", &time)
        //     .build();

        // let _output = dynamo_client
        //     .put_item()
        //     .table_name(util::dynamo::get_table_name())
        //     .set_item(Some(map))
        //     .send()
        //     .await?;
    }
    //
    Ok(true)
}
