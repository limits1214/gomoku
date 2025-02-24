use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Local;
use futures::future::join_all;
use lambda_http::tracing;

use crate::{
    constant::{PK, SK},
    util::{self, dynamo::DynamoMapHelper},
};

pub async fn topic_subscribe(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: &str,
    topic: &str,
) -> anyhow::Result<()> {
    gomoku_lib::service::topic_subscribe(connection_id, topic).await?;
    // let get_output = dynamo_client
    //     .get_item()
    //     .table_name(util::dynamo::get_table_name())
    //     .key(PK, AttributeValue::S(format!("WS_CONN#{connection_id}")))
    //     .key(SK, AttributeValue::S(format!("INFO")))
    //     .send()
    //     .await?;
    // let user_id = get_output
    //     .item
    //     .as_ref()
    //     .and_then(|item| item.get("userId"))
    //     .and_then(|user_id| user_id.as_s().ok());

    // let sub = if let Some(sub) = user_id {
    //     sub
    // } else {
    //     "Empty"
    // };

    // let ws_topic_pk = format!("WS_TOPIC#{topic}");
    // let ws_topic_sk = format!("WS_CONN#{connection_id}");
    // let time = Local::now().to_rfc3339();
    // let map = DynamoMapHelper::new()
    //     .insert_pk(ws_topic_pk)
    //     .insert_sk(ws_topic_sk)
    //     .insert_attr_s("createdAt", &time)
    //     .insert_attr_s("userId", sub)
    //     .insert_attr_s("connectionId", &connection_id)
    //     .build();

    // let output = dynamo_client
    //     .put_item()
    //     .table_name(util::dynamo::get_table_name())
    //     .set_item(Some(map))
    //     .send()
    //     .await?;

    // let ws_conn_pk = format!("WS_CONN#{connection_id}");
    // let ws_conn_sk = format!("WS_TOPIC#{topic}");
    // let map = DynamoMapHelper::new()
    //     .insert_pk(ws_conn_pk)
    //     .insert_sk(ws_conn_sk)
    //     .insert_attr_s("createdAt", &time)
    //     .insert_attr_s("connectionId", &connection_id)
    //     .build();
    // let output = dynamo_client
    //     .put_item()
    //     .table_name(util::dynamo::get_table_name())
    //     .set_item(Some(map))
    //     .send()
    //     .await?;
    Ok(())
}

pub async fn topic_unsubscribe(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: &str,
    topic: &str,
) -> anyhow::Result<()> {
    gomoku_lib::service::topic_unsubscribe(connection_id, topic).await?;
    // let ws_conn_pk = format!("WS_TOPIC#{topic}");
    // let ws_conn_sk = format!("WS_CONN#{connection_id}");
    // let _output = dynamo_client
    //     .delete_item()
    //     .table_name(util::dynamo::get_table_name())
    //     .key(PK, AttributeValue::S(ws_conn_pk))
    //     .key(SK, AttributeValue::S(ws_conn_sk))
    //     .send()
    //     .await?;

    // let delete_ws_conn_pk = format!("WS_CONN#{connection_id}");
    // let delete_ws_conn_topic_sk = format!("WS_TOPIC#{topic}");
    // let _delete_output = dynamo_client
    //     .delete_item()
    //     .table_name(util::dynamo::get_table_name())
    //     .key(PK, AttributeValue::S(delete_ws_conn_pk))
    //     .key(SK, AttributeValue::S(delete_ws_conn_topic_sk))
    //     .send()
    //     .await?;

    Ok(())
}

pub async fn topic_post(
    dynamo_client: &aws_sdk_dynamodb::Client,
    gw_ws_client: &aws_sdk_apigatewaymanagement::Client,
    topic_pk: &str,
    value: serde_json::Value,
) -> anyhow::Result<()> {
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
