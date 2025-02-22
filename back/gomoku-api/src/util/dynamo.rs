use std::collections::HashMap;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
use serde_json::Value;

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
    pub fn insert_attr_n<T: Into<String>>(mut self, key: T, val: T) -> Self {
        self.0.insert(key.into(), AttributeValue::N(val.into()));
        self
    }
    pub fn build(self) -> HashMap<String, AttributeValue> {
        self.0
    }
}

pub fn last_evaluated_key_to_str(key: Option<HashMap<String, AttributeValue>>) -> Option<String> {
    let Some(key) = key else { return None };
    let key = key
        .into_iter()
        .map(|m| (m.0, m.1.as_s().unwrap_or(&String::new()).to_owned()))
        .collect::<HashMap<_, _>>();
    let jsonstr = serde_json::to_string(&key).unwrap_or_default();
    let res = super::aes::aes128encrypt(&jsonstr).unwrap_or_default();
    Some(res)
}

pub fn str_to_last_evaluated_key(key: Option<&str>) -> Option<HashMap<String, AttributeValue>> {
    let Some(key) = key else { return None };
    let key = super::aes::aes128decrypt(&key).unwrap_or_default();
    let value = serde_json::from_str::<Value>(&key).unwrap_or_default();
    let Some(obj) = value.as_object() else {
        return None;
    };
    let res = obj
        .iter()
        .map(|(key, value)| {
            (
                key.clone(),
                AttributeValue::S(value.as_str().unwrap_or_default().to_owned()),
            )
        })
        .collect::<HashMap<_, _>>();
    Some(res)
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

#[tokio::test]
async fn eval_key_str_test() {
    use crate::config::app_config::AppConfig;
    AppConfig::init().await;
    let mut map = HashMap::new();
    map.insert("PK".to_string(), AttributeValue::S("hi1".to_string()));
    map.insert("SK".to_string(), AttributeValue::S("hi2".to_string()));
    let str = last_evaluated_key_to_str(Some(map.clone()));
    let res = str_to_last_evaluated_key(str.as_deref());
    assert_eq!(res, Some(map));
}
#[tokio::test]
async fn eval_key_str_test2() {
    use crate::config::app_config::AppConfig;
    AppConfig::init().await;
    let str = "Oaz6NJWpBmlDoILIugeOwyN5NETmG2+wNYdOl9a2RTUqe+jUqdqmwfLU/Ez2cZ5W1SYLxnPJE+3+ql+4";
    let res = str_to_last_evaluated_key(Some(str));
    println!("res: {res:?}");
}
