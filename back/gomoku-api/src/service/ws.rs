use crate::{
    constant::{PK, SK},
    util::{self, dynamo::DynamoMapHelper},
};
use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
use chrono::Local;
use lambda_http::tracing;
use serde_json::json;

pub async fn ws_token_verify(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: &str,
    sub: &str,
) -> anyhow::Result<()> {
    if sub == "Empty" {
        let ws_conn_pk = format!("WS_CONN#{connection_id}");
        let ws_conn_sk = format!("INFO");
        let time = Local::now().to_rfc3339();
        let map = DynamoMapHelper::new()
            .insert_pk(ws_conn_pk)
            .insert_sk(ws_conn_sk)
            .insert_attr_s("createdAt", &time)
            .build();

        let output = dynamo_client
            .put_item()
            .table_name(util::dynamo::get_table_name())
            .set_item(Some(map))
            .send()
            .await?;
    } else {
        let ws_token = util::jwt::generate_ws_token(sub, connection_id)?;
        // tracing::info!("get ws token: {ws_token}");
        let user_id = sub;
        let ws_conn_pk = format!("WS_CONN#{connection_id}");
        let ws_conn_sk = format!("INFO");
        let time = Local::now().to_rfc3339();
        let map = DynamoMapHelper::new()
            .insert_pk(ws_conn_pk)
            .insert_sk(ws_conn_sk)
            .insert_attr_s("wsToken", &ws_token)
            .insert_attr_s("createdAt", &time)
            .insert_attr_s("userId", &user_id)
            .build();

        let output = dynamo_client
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

        let output = dynamo_client
            .put_item()
            .table_name(util::dynamo::get_table_name())
            .set_item(Some(map))
            .send()
            .await?;
    }

    Ok(())
}

// ws 끊어짐
pub async fn ws_disconnect(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: String,
) -> anyhow::Result<()> {
    // 1. USER 제거 // jwt 존재시
    // 2. WS_CONN 제거
    // 3. WS_TOPIC 제거(with broadcast?)

    // 혹시 Guest 가 존재할수 있으니 지워준다.
    let delete_ws_conn_pk = format!("WS_CONN#{connection_id}");
    let delete_ws_conn_sk = format!("INFO");
    let delete_ws_conn_topic_sk = format!("WS_TOPIC#");

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

        let _delete_output = dynamo_client
            .delete_item()
            .table_name(util::dynamo::get_table_name())
            .key(PK, AttributeValue::S(item_pk))
            .key(SK, AttributeValue::S(item_sk.clone()))
            .send()
            .await?;

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

    let _delete_output = dynamo_client
        .delete_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(delete_ws_conn_pk))
        .key(SK, AttributeValue::S(delete_ws_conn_sk))
        .send()
        .await?;

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

// jwt 업데이트
pub fn ws_jwt_set(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: String,
) -> anyhow::Result<()> {
    Ok(())
}

pub async fn ws_get_token(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: String,
) -> anyhow::Result<Option<String>> {
    let ws_conn_pk = format!("WS_CONN#{connection_id}");
    let ws_conn_sk = format!("INFO");
    let get_output = dynamo_client
        .get_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(ws_conn_pk))
        .key(SK, AttributeValue::S(ws_conn_sk))
        .send()
        .await?;

    let user_id = get_output
        .item
        .as_ref()
        .and_then(
            |output: &std::collections::HashMap<String, AttributeValue>| output.get("wsToken"),
        )
        .and_then(|ws_token| match ws_token {
            AttributeValue::S(ws_token) => Some(ws_token.to_owned()),
            _ => None,
        });
    Ok(user_id)
}

// ws topic 세팅
pub async fn ws_subscribe_topic(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: &str,
    sub: &str,
    topic: &str,
) -> anyhow::Result<()> {
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

// ws topic 제거
pub async fn ws_unsubscribe_topic(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: &str,
    sub: &str,
    topic: &str,
) -> anyhow::Result<()> {
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
    // let output = dynamo_client
    //     .query()
    //     .table_name(util::dynamo::get_table_name())
    //     .key_condition_expression("PK = :PK AND begins_with(SK, :SK)")
    //     .expression_attribute_values(":PK", AttributeValue::S(delete_ws_conn_pk.clone()))
    //     .expression_attribute_values(":SK", AttributeValue::S(delete_ws_conn_topic_sk))
    //     .send()
    //     .await?;

    // for item in output.items.unwrap_or_default() {
    //     let item_pk = item.get(PK).unwrap().as_s().unwrap().to_owned();
    //     let item_sk = item.get(SK).unwrap().as_s().unwrap().to_owned();
    //     let _delete_output = dynamo_client
    //         .delete_item()
    //         .table_name(util::dynamo::get_table_name())
    //         .key(PK, AttributeValue::S(item_pk))
    //         .key(SK, AttributeValue::S(item_sk.clone()))
    //         .send()
    //         .await?;

    //     // let _delete_output = dynamo_client
    //     //     .delete_item()
    //     //     .table_name(util::dynamo::get_table_name())
    //     //     .key(PK, AttributeValue::S(item_sk))
    //     //     .key(SK, AttributeValue::S(format!("WS_CONN#{connection_id}")))
    //     //     .send()
    //     //     .await?;
    // }
    Ok(())
}

pub async fn ws_room_chat(
    dynamo_client: &aws_sdk_dynamodb::Client,
    gw_ws_client: &aws_sdk_apigatewaymanagement::Client,
    connection_id: &str,
    sub: &str,
    msg: &str,
    room_id: &str,
) -> anyhow::Result<()> {
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

    for item in items {
        let connection_id = item.get("connectionId").unwrap().as_s().unwrap();
        let res = gw_ws_client
            .post_to_connection()
            .connection_id(connection_id)
            .data(util::dynamo::json_value_to_blob(json!({"msg": msg}))?)
            .send()
            .await;
        if let Err(err) = res {
            tracing::error!("post to connection err: {err:?}");
        }
    }
    Ok(())
}
