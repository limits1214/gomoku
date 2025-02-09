use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use axum::{extract::State, response::IntoResponse, Router};
use lambda_http::tracing;

use crate::config::app_state::ArcAppState;

pub fn room_router(state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
}

async fn test(dynamo_client: State<aws_sdk_dynamodb::Client>) -> impl IntoResponse {
    let mut item = HashMap::new();
    item.insert("PK".to_string(), AttributeValue::S("USER#123".to_string()));
    item.insert("SK".to_string(), AttributeValue::S("HAHA".to_string()));

    let a = dynamo_client
        .put_item()
        .table_name("Test")
        .set_item(Some(item))
        .condition_expression("attribute_not_exists(PK) AND attribute_not_exists(SK)")
        .send()
        .await
        .unwrap();

    tracing::info!("a: {a:?}");
}

async fn create_room(dynamo_client: State<aws_sdk_dynamodb::Client>) -> impl IntoResponse {
    //
}
