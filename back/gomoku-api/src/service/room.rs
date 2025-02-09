use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Local;
use lambda_http::tracing;
use nanoid::nanoid;
use std::collections::HashMap;

use crate::{
    constant::{GSI1INDEX, GSI1PK, GSI1SK, PK, SK},
    util,
};

pub async fn create_room(dynamo_client: &aws_sdk_dynamodb::Client) -> anyhow::Result<()> {
    let nanoid = nanoid!();
    let pk = format!("ROOM#{nanoid}");

    let time = Local::now().to_rfc3339();
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(PK.to_string(), AttributeValue::S(pk.clone()));
    item.insert(SK.to_string(), AttributeValue::S("HAHA".to_string()));
    item.insert(GSI1PK.to_string(), AttributeValue::S("ROOMS".to_string()));
    item.insert(
        GSI1SK.to_string(),
        AttributeValue::S(format!("CREATED_AT#{time}")),
    );
    item.insert(
        "RoomName".to_string(),
        AttributeValue::S(format!("hahaha {pk}")),
    );

    let table_name = util::dynamo::get_table_name();

    let output = dynamo_client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .condition_expression("attribute_not_exists(PK) AND attribute_not_exists(SK)")
        .send()
        .await?;

    tracing::info!("output: {output:?}");

    Ok(())
}

pub async fn room_list(dynamo_client: &aws_sdk_dynamodb::Client) -> anyhow::Result<()> {
    //
    let table_name = util::dynamo::get_table_name();
    let time = Local::now().to_rfc3339();
    let query = dynamo_client
        .query()
        .table_name(table_name)
        .index_name(GSI1INDEX)
        .key_condition_expression(format!("{GSI1PK} = :PK AND {GSI1SK} <= :TIME"))
        .expression_attribute_values(":PK", AttributeValue::S("ROOMS".to_string()))
        .expression_attribute_values(":TIME", AttributeValue::S(format!("CREATED_AT#{time}")))
        .limit(10);

    // query.set_exclusive_start_key(input)

    let response = query.send().await?;

    tracing::info!("response: {:?}", response);
    if let Some(items) = response.items {
        for item in items {
            tracing::info!("item: {item:?}");
        }
    }

    Ok(())
}
