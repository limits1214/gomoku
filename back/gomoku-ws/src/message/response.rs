use aws_sdk_apigatewaymanagement::primitives::Blob;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "d")]
#[serde(rename_all = "camelCase")]
pub enum WsResponseMessage {
    #[serde(rename_all = "camelCase")]
    Echo { msg: String },
}

impl TryFrom<WsResponseMessage> for Blob {
    type Error = serde_json::error::Error;

    fn try_from(value: WsResponseMessage) -> Result<Self, Self::Error> {
        Ok(serde_json::to_value(value)?.to_string().into_bytes().into())
    }
}
