use aws_sdk_dynamodb::types::AttributeValue;

use crate::{
    constant::{PK, SK},
    util,
};

pub async fn ws_disconnect(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: &str,
) -> anyhow::Result<()> {
    gomoku_lib::service::delete_conn(connection_id).await?;
    // 지워야하는 목록
    // 1. PK:WS_CONN#{}, SK:INFO
    // 2. PK:WS_CONN#{}, SK:WS_TOPIC#{}
    // 3. PK:WS_TOPIC#{}, WS_CONN#{}
    // 4. PK:USER#{}, SK:WS_CONN#{}

    // let delete_ws_conn_pk = format!("WS_CONN#{connection_id}");
    // let delete_ws_conn_sk = format!("INFO");
    // let delete_ws_conn_topic_sk = format!("WS_TOPIC#");

    // // ws_conn에 달려 있는 모든 토픽 쿼리
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

    //     // ws_conn에 달려 있는 토픽 제거
    //     let _delete_output = dynamo_client
    //         .delete_item()
    //         .table_name(util::dynamo::get_table_name())
    //         .key(PK, AttributeValue::S(item_pk))
    //         .key(SK, AttributeValue::S(item_sk.clone()))
    //         .send()
    //         .await?;

    //     // ws_topic 제거
    //     let _delete_output = dynamo_client
    //         .delete_item()
    //         .table_name(util::dynamo::get_table_name())
    //         .key(PK, AttributeValue::S(item_sk))
    //         .key(SK, AttributeValue::S(format!("WS_CONN#{connection_id}")))
    //         .send()
    //         .await?;
    // }

    // let get_output = dynamo_client
    //     .get_item()
    //     .table_name(util::dynamo::get_table_name())
    //     .key(PK, AttributeValue::S(delete_ws_conn_pk.clone()))
    //     .key(SK, AttributeValue::S(delete_ws_conn_sk.clone()))
    //     .send()
    //     .await?;

    // // ws_conn, info 제거
    // let _delete_output = dynamo_client
    //     .delete_item()
    //     .table_name(util::dynamo::get_table_name())
    //     .key(PK, AttributeValue::S(delete_ws_conn_pk))
    //     .key(SK, AttributeValue::S(delete_ws_conn_sk))
    //     .send()
    //     .await?;

    // // ws_conn에 user_id가 있다면 user_id의 ws_conn 제거
    // let user_id = get_output
    //     .item
    //     .as_ref()
    //     .and_then(|output| output.get("userId"))
    //     .and_then(|user_id| match user_id {
    //         AttributeValue::S(user_id) => Some(user_id.as_str()),
    //         _ => None,
    //     });

    // if let Some(user_id) = user_id {
    //     // USER의 WS_CONN 제거
    //     let delete_user_pk = format!("USER#{user_id}");
    //     let delete_user_sk = format!("WS_CONN#{connection_id}");
    //     let _output = dynamo_client
    //         .delete_item()
    //         .table_name(util::dynamo::get_table_name())
    //         .key(PK, AttributeValue::S(delete_user_pk))
    //         .key(SK, AttributeValue::S(delete_user_sk))
    //         .send()
    //         .await?;
    // }

    Ok(())
}
