use crate::{
    config::APP_CONFIG,
    constant::{PK, SK},
};
use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
use std::collections::HashMap;

pub fn get_table_name() -> &'static str {
    APP_CONFIG
        .get()
        .unwrap()
        .settings
        .dynamo
        .table_name
        .as_str()
}

pub type DynamoMap = HashMap<String, AttributeValue>;

pub struct DynamoMapHelper(HashMap<String, AttributeValue>);

impl DynamoMapHelper {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn insert_pk<T: Into<String>>(mut self, pk: T) -> Self {
        self.0.insert(PK.to_string(), AttributeValue::S(pk.into()));
        self
    }
    pub fn insert_sk<T: Into<String>>(mut self, pk: T) -> Self {
        self.0.insert(SK.to_string(), AttributeValue::S(pk.into()));
        self
    }
    pub fn insert_attr_s<T: Into<String>>(mut self, key: T, val: T) -> Self {
        self.0.insert(key.into(), AttributeValue::S(val.into()));
        self
    }
    pub fn insert_attr_n<T: Into<String>>(mut self, key: T, val: T) -> Self {
        self.0.insert(key.into(), AttributeValue::N(val.into()));
        self
    }
    pub fn build(self) -> HashMap<String, AttributeValue> {
        self.0
    }
}

pub fn json_value_to_blob(value: serde_json::Value) -> anyhow::Result<Blob> {
    Ok(serde_json::to_value(value)?.to_string().into_bytes().into())
}

#[test]
fn test() {
    let _map = DynamoMapHelper::new()
        .insert_pk("pk")
        .insert_sk("pk")
        .build();
}
