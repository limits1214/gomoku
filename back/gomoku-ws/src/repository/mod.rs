use aws_sdk_dynamodb::types::AttributeValue;

use crate::{
    constant::{PK, SK},
    util::{self, dynamo::DynamoMap},
};

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
