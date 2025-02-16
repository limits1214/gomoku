use std::collections::HashMap;

use aws_sdk_dynamodb::types::{AttributeValue, KeysAndAttributes};
use chrono::Local;

use lambda_http::tracing;
use nanoid::nanoid;

use crate::{
    constant::{PK, SK},
    dto::response::room::RoomInfo,
    util::{self, dynamo::DynamoMapHelper},
};

pub async fn create_room(
    dynamo_client: &aws_sdk_dynamodb::Client,
    room_name: &str,
    channel: &str,
) -> anyhow::Result<()> {
    //
    let channel_pk = format!("CHANNEL#{channel}");
    let output = dynamo_client
        .query()
        .table_name(util::dynamo::get_table_name())
        .key_condition_expression("PK = :PK AND begins_with(SK, :SK)")
        .expression_attribute_values(":PK", AttributeValue::S(channel_pk.clone()))
        .expression_attribute_values(":SK", AttributeValue::S("ROOM#".to_string()))
        .send()
        .await?;
    // tracing::info!("channel output {output:?}");
    let mut room_num = 1;

    let rn = output
        .items
        .as_ref()
        .and_then(|items| items.last())
        .and_then(|last| last.get("roomNum"))
        .and_then(|rn| rn.as_n().ok())
        .and_then(|rn| rn.parse::<u32>().ok());

    if let Some(rn) = rn {
        room_num = rn + 1;
    }

    // add ROOM
    let room_id = nanoid!();
    let room_pk = format!("ROOM#{room_id}");
    let time = Local::now().to_rfc3339();

    let map = DynamoMapHelper::new()
        .insert_pk(room_pk)
        .insert_sk("INFO")
        .insert_attr_s("createdAt", &time)
        .insert_attr_n("roomNum", &room_num.to_string())
        .insert_attr_s("channel", &channel)
        .insert_attr_s("roomName", room_name)
        .insert_attr_s("roomId", &room_id)
        .build();
    let output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;

    // add CHANNEL ROOM
    let channel_room_sk = format!("ROOM#{room_num}");
    let map = DynamoMapHelper::new()
        .insert_pk(channel_pk)
        .insert_sk(channel_room_sk)
        .insert_attr_s("roomId", &room_id)
        .insert_attr_n("roomNum", &room_num.to_string())
        .build();
    let output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;

    Ok(())
}

pub async fn room_list(
    dynamo_client: &aws_sdk_dynamodb::Client,
    channel: &str,
    pagination_key: Option<String>,
) -> anyhow::Result<(Vec<RoomInfo>, Option<String>)> {
    let start_key = util::dynamo::str_to_last_evaluated_key(pagination_key.as_deref());

    let channel_pk = format!("CHANNEL#{channel}");
    let output = dynamo_client
        .query()
        .table_name(util::dynamo::get_table_name())
        .key_condition_expression("PK = :PK AND begins_with(SK, :SK)")
        .expression_attribute_values(":PK", AttributeValue::S(channel_pk.clone()))
        .expression_attribute_values(":SK", AttributeValue::S("ROOM#".to_string()))
        .set_exclusive_start_key(start_key)
        .limit(20)
        .scan_index_forward(false)
        .send()
        .await?;

    let last_evaluated_key = output.last_evaluated_key;
    let pagination_key_str = util::dynamo::last_evaluated_key_to_str(last_evaluated_key);

    let items = output.items.unwrap_or_default();
    if items.is_empty() {
        // batch get item 에 파라미터 안넣으면 패닉
        return Ok((vec![], None));
    }

    let room_ids = items
        .iter()
        .map(|m| {
            let room_id = m.get("roomId").unwrap().as_s().unwrap();
            let room_num = m.get("roomNum").unwrap().as_n().unwrap();
            (room_id, room_num)
        })
        .collect::<HashMap<_, _>>();

    let room_batch_key = room_ids
        .iter()
        .map(|(room_id, _)| {
            DynamoMapHelper::new()
                .insert_pk(format!("ROOM#{room_id}"))
                .insert_sk("INFO")
                .build()
        })
        .collect::<Vec<_>>();

    let mut request_items = HashMap::new();
    request_items.insert(
        util::dynamo::get_table_name().to_string(),
        KeysAndAttributes::builder()
            .set_keys(Some(room_batch_key))
            .build()
            .unwrap(),
    );

    let batch_get_item_output = dynamo_client
        .batch_get_item()
        .set_request_items(Some(request_items))
        .send()
        .await?;

    let room = batch_get_item_output
        .responses
        .and_then(|mut r| r.remove(util::dynamo::get_table_name()))
        .and_then(|items| Some(items))
        .unwrap_or_default();

    let room_infos = room
        .iter()
        .map(|m| {
            tracing::info!("m {m:?}");
            let channel = m.get("channel").unwrap().as_s().unwrap();
            let room_name = m.get("roomName").unwrap().as_s().unwrap();
            let room_id = m.get("roomId").unwrap().as_s().unwrap();
            let room_num = m.get("roomNum").unwrap().as_n().unwrap();
            RoomInfo {
                channel: channel.to_string(),
                room_id: room_id.to_string(),
                room_name: room_name.to_string(),
                room_num: room_num.to_string(),
            }
        })
        .collect::<Vec<_>>();

    Ok((room_infos, pagination_key_str))
}

pub async fn channel_room_info(
    dynamo_client: &aws_sdk_dynamodb::Client,
    channel_id: &str,
    room_num: &str,
) -> anyhow::Result<Option<RoomInfo>> {
    let channel_room_pk = format!("CHANNEL#{channel_id}");
    let channel_room_sk = format!("ROOM#{room_num}");
    let output = dynamo_client
        .get_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(channel_room_pk))
        .key(SK, AttributeValue::S(channel_room_sk))
        .send()
        .await?;
    let room_id = output.item.and_then(|item| {
        let room_id = item.get("roomId").unwrap().as_s().unwrap();
        Some(room_id.to_owned())
    });
    if let Some(room_id) = room_id {
        let rinfo = room_info(dynamo_client, &room_id).await?;
        Ok(rinfo)
    } else {
        Ok(None)
    }
}

pub async fn room_info(
    dynamo_client: &aws_sdk_dynamodb::Client,
    room_id: &str,
) -> anyhow::Result<Option<RoomInfo>> {
    let room_pk = format!("ROOM#{room_id}");
    let room_sk = "INFO".to_string();
    let output = dynamo_client
        .get_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(room_pk))
        .key(SK, AttributeValue::S(room_sk))
        .send()
        .await?;

    let room_info = output.item.and_then(|item| {
        let room_name = item.get("roomName").unwrap().as_s().unwrap();
        let room_id = item.get("roomId").unwrap().as_s().unwrap();
        let room_num = item.get("roomNum").unwrap().as_n().unwrap();
        let channel = item.get("channel").unwrap().as_s().unwrap();
        let room_info = RoomInfo {
            channel: channel.to_string(),
            room_id: room_id.to_string(),
            room_name: room_name.to_string(),
            room_num: room_num.to_string(),
        };
        Some(room_info)
    });

    Ok(room_info)
}
