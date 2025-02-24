use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{Local, Utc};
use futures::future::join_all;
use serde_json::json;

use crate::{
    constant::{PK, SK},
    util::{
        self,
        dynamo::{DynamoMap, DynamoMapHelper},
    },
};

pub async fn put_empty_conn(connection_id: &str) -> anyhow::Result<()> {
    let ws_conn_pk = format!("WS_CONN#{connection_id}");
    let ws_conn_sk = format!("INFO");
    let time = Local::now().to_rfc3339();
    let map = DynamoMapHelper::new()
        .insert_pk(ws_conn_pk)
        .insert_sk(ws_conn_sk)
        .insert_attr_s("createdAt", &time)
        .build();
    let dynamo_client = util::dynamo::get_dynamo_client();
    let _output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;
    Ok(())
}

pub async fn put_conn(connection_id: &str, user_id: &str) -> anyhow::Result<()> {
    let ws_conn_pk = format!("WS_CONN#{connection_id}");
    let ws_conn_sk = format!("INFO");
    let time = Local::now().to_rfc3339();
    let map = DynamoMapHelper::new()
        .insert_pk(ws_conn_pk)
        .insert_sk(ws_conn_sk)
        .insert_attr_s("createdAt", &time)
        .insert_attr_s("userId", &user_id)
        .build();
    let dynamo_client = util::dynamo::get_dynamo_client();
    let _output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;

    let user_pk = format!("USER#{user_id}");
    let user_sk = format!("WS_CONN#{connection_id}");
    let map = DynamoMapHelper::new()
        .insert_pk(user_pk)
        .insert_sk(user_sk)
        .insert_attr_s("createdAt", &time)
        .build();

    let _output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;
    Ok(())
}

pub async fn delete_conn(connection_id: &str) -> anyhow::Result<()> {
    // 지워야하는 목록
    // 1. PK:WS_CONN#{}, SK:INFO
    // 2. PK:WS_CONN#{}, SK:WS_TOPIC#{}
    // 3. PK:WS_TOPIC#{}, WS_CONN#{}
    // 4. PK:USER#{}, SK:WS_CONN#{}

    let delete_ws_conn_pk = format!("WS_CONN#{connection_id}");
    let delete_ws_conn_sk = format!("INFO");
    let delete_ws_conn_topic_sk = format!("WS_TOPIC#");
    let dynamo_client = util::dynamo::get_dynamo_client();
    // ws_conn에 달려 있는 모든 토픽 쿼리
    let output = dynamo_client
        .query()
        .table_name(util::dynamo::get_table_name())
        .key_condition_expression("PK = :PK AND begins_with(SK, :SK)")
        .expression_attribute_values(":PK", AttributeValue::S(delete_ws_conn_pk.clone()))
        .expression_attribute_values(":SK", AttributeValue::S(delete_ws_conn_topic_sk))
        .send()
        .await?;

    for item in output.items.unwrap_or_default() {
        let item_pk = item.get(PK).unwrap().as_s().unwrap().to_owned();
        let item_sk = item.get(SK).unwrap().as_s().unwrap().to_owned();

        // ws_conn에 달려 있는 토픽 제거
        let _delete_output = dynamo_client
            .delete_item()
            .table_name(util::dynamo::get_table_name())
            .key(PK, AttributeValue::S(item_pk))
            .key(SK, AttributeValue::S(item_sk.clone()))
            .send()
            .await?;

        // ws_topic 제거
        let _delete_output = dynamo_client
            .delete_item()
            .table_name(util::dynamo::get_table_name())
            .key(PK, AttributeValue::S(item_sk))
            .key(SK, AttributeValue::S(format!("WS_CONN#{connection_id}")))
            .send()
            .await?;
    }

    let get_output = dynamo_client
        .get_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(delete_ws_conn_pk.clone()))
        .key(SK, AttributeValue::S(delete_ws_conn_sk.clone()))
        .send()
        .await?;

    // ws_conn, info 제거
    let _delete_output = dynamo_client
        .delete_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(delete_ws_conn_pk))
        .key(SK, AttributeValue::S(delete_ws_conn_sk))
        .send()
        .await?;

    // ws_conn에 user_id가 있다면 user_id의 ws_conn 제거
    let user_id = get_output
        .item
        .as_ref()
        .and_then(|output| output.get("userId"))
        .and_then(|user_id| match user_id {
            AttributeValue::S(user_id) => Some(user_id.as_str()),
            _ => None,
        });

    if let Some(user_id) = user_id {
        // USER의 WS_CONN 제거
        let delete_user_pk = format!("USER#{user_id}");
        let delete_user_sk = format!("WS_CONN#{connection_id}");
        let _output = dynamo_client
            .delete_item()
            .table_name(util::dynamo::get_table_name())
            .key(PK, AttributeValue::S(delete_user_pk))
            .key(SK, AttributeValue::S(delete_user_sk))
            .send()
            .await?;
    }

    Ok(())
}
pub async fn topic_subscribe(connection_id: &str, topic: &str) -> anyhow::Result<()> {
    let dynamo_client = util::dynamo::get_dynamo_client();
    let get_output = dynamo_client
        .get_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(format!("WS_CONN#{connection_id}")))
        .key(SK, AttributeValue::S(format!("INFO")))
        .send()
        .await?;
    let user_id = get_output
        .item
        .as_ref()
        .and_then(|item| item.get("userId"))
        .and_then(|user_id| user_id.as_s().ok());

    let sub = if let Some(sub) = user_id {
        sub
    } else {
        "Empty"
    };

    let ws_topic_pk = format!("WS_TOPIC#{topic}");
    let ws_topic_sk = format!("WS_CONN#{connection_id}");
    let time = Local::now().to_rfc3339();
    let map = DynamoMapHelper::new()
        .insert_pk(ws_topic_pk)
        .insert_sk(ws_topic_sk)
        .insert_attr_s("createdAt", &time)
        .insert_attr_s("userId", sub)
        .insert_attr_s("connectionId", &connection_id)
        .build();

    let output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;

    let ws_conn_pk = format!("WS_CONN#{connection_id}");
    let ws_conn_sk = format!("WS_TOPIC#{topic}");
    let map = DynamoMapHelper::new()
        .insert_pk(ws_conn_pk)
        .insert_sk(ws_conn_sk)
        .insert_attr_s("createdAt", &time)
        .insert_attr_s("connectionId", &connection_id)
        .build();
    let output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;
    Ok(())
}
pub async fn topic_unsubscribe(connection_id: &str, topic: &str) -> anyhow::Result<()> {
    let dynamo_client = util::dynamo::get_dynamo_client();
    let ws_conn_pk = format!("WS_TOPIC#{topic}");
    let ws_conn_sk = format!("WS_CONN#{connection_id}");
    let _output = dynamo_client
        .delete_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(ws_conn_pk))
        .key(SK, AttributeValue::S(ws_conn_sk))
        .send()
        .await?;

    let delete_ws_conn_pk = format!("WS_CONN#{connection_id}");
    let delete_ws_conn_topic_sk = format!("WS_TOPIC#{topic}");
    let _delete_output = dynamo_client
        .delete_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(delete_ws_conn_pk))
        .key(SK, AttributeValue::S(delete_ws_conn_topic_sk))
        .send()
        .await?;
    Ok(())
}
async fn topic_post(topic_pk: &str, value: serde_json::Value) -> anyhow::Result<()> {
    let dynamo_client = util::dynamo::get_dynamo_client();
    let gw_ws_client = util::dynamo::get_gw_ws_client();

    let topic_sk = format!("WS_CONN#");
    let output = dynamo_client
        .query()
        .table_name(util::dynamo::get_table_name())
        .key_condition_expression("PK = :PK AND begins_with(SK, :SK)")
        .expression_attribute_values(":PK", AttributeValue::S(topic_pk.to_owned()))
        .expression_attribute_values(":SK", AttributeValue::S(topic_sk))
        .send()
        .await?;
    let items = output.items.unwrap_or_default();
    let futures = items
        .into_iter()
        .map(|item| {
            let connection_id = item
                .get("connectionId")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string();
            let value = value.clone();
            let gw_ws_client = gw_ws_client.clone();

            async move {
                let res = gw_ws_client
                    .post_to_connection()
                    .connection_id(&connection_id)
                    .data(util::dynamo::json_value_to_blob(value).unwrap())
                    .send()
                    .await;

                if let Err(err) = res {
                    tracing::error!("post to connection err: {err:?}");
                }
            }
        })
        .collect::<Vec<_>>();
    join_all(futures).await;
    Ok(())
}

pub async fn room_chat(connection_id: &str, msg: &str, room_id: &str) -> anyhow::Result<()> {
    let dynamo_client = util::dynamo::get_dynamo_client();

    let now = Utc::now();
    let time = now.to_rfc3339();
    let user_info = user_info_by_connection_id(dynamo_client, connection_id).await?;
    let topic_msg = if let Some(user_info) = user_info {
        let nick_name = user_info.get("nickName").unwrap().as_s().unwrap();
        json!({"msg": msg, "time": time, "nickName": nick_name})
    } else {
        json!({"msg": msg, "time": time})
    };

    // post msg
    let ws_topic_pk = format!("WS_TOPIC#ROOM#{room_id}");
    topic_post(&ws_topic_pk, topic_msg).await?;
    Ok(())
}

pub async fn user_info_by_connection_id(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: &str,
) -> anyhow::Result<Option<DynamoMap>> {
    let get_output = dynamo_client
        .get_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(format!("WS_CONN#{connection_id}")))
        .key(SK, AttributeValue::S(format!("INFO")))
        .send()
        .await?;

    let user_id = get_output
        .item
        .as_ref()
        .and_then(|item| item.get("userId"))
        .and_then(|user_id| user_id.as_s().ok());
    let Some(user_id) = user_id else {
        return Ok(None);
    };
    let get_output = dynamo_client
        .get_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(format!("USER#{user_id}")))
        .key(SK, AttributeValue::S(format!("INFO")))
        .send()
        .await?;
    Ok(get_output.item)
}
