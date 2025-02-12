use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::constant::{PK, SK};

pub fn get_table_name() -> &'static str {
    super::config::get_dynamo_settings().table_name.as_str()
}

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
    pub fn build(self) -> HashMap<String, AttributeValue> {
        self.0
    }
}

#[test]
fn test() {
    let _map = DynamoMapHelper::new()
        .insert_pk("pk")
        .insert_sk("pk")
        .build();
}
