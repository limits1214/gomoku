use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Local;
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
    if let Some(items) = output.items {
        if let Some(a) = items.last() {
            let rn = a.get("roomNum");
            if let Some(rn) = rn {
                if let Ok(rn) = rn.as_n() {
                    if let Ok(rn) = rn.parse::<u32>() {
                        room_num = rn + 1;
                    }
                }
            }
        }
    }

    // add ROOM
    let room_id = nanoid!();
    let room_pk = format!("ROOM#{room_id}");
    let time = Local::now().to_rfc3339();

    let map = DynamoMapHelper::new()
        .insert_pk(room_pk)
        .insert_sk("INFO")
        .insert_attr_s("createdAt", &time)
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
        .limit(2)
        .scan_index_forward(false)
        .send()
        .await?;

    let last_evaluated_key = output.last_evaluated_key;
    let pagination_key_str = util::dynamo::last_evaluated_key_to_str(last_evaluated_key);

    let items = output.items.unwrap_or_default();

    let room_ids = items
        .iter()
        .map(|m: &std::collections::HashMap<String, AttributeValue>| {
            let room_num = m.get("roomNum").unwrap().as_n().unwrap();
            let room_id = m.get("roomId").unwrap().as_s().unwrap();
            (room_num, room_id)
        })
        .collect::<Vec<_>>();

    let mut room: Vec<HashMap<String, AttributeValue>> = Vec::new();
    for (room_num, room_id) in room_ids {
        let response = dynamo_client
            .get_item()
            .table_name(util::dynamo::get_table_name())
            .key(PK, AttributeValue::S(format!("ROOM#{room_id}")))
            .key(SK, AttributeValue::S("INFO".to_string()))
            .send()
            .await
            .unwrap();
        let mut a = response.item.unwrap();
        a.insert(
            "roomNum".to_string(),
            AttributeValue::N(room_num.to_string()),
        );
        room.push(a);
    }

    let room_infos = room
        .iter()
        .map(|m| {
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
