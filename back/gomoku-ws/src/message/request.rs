use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "d")]
#[serde(rename_all = "camelCase")]
pub enum WsRequestMessage {
    #[serde(rename_all = "camelCase")]
    Echo { msg: String },
    #[serde(rename_all = "camelCase")]
    TopicSubscribe { topic: String },
    #[serde(rename_all = "camelCase")]
    TopicUnSubscribe { topic: String },
    #[serde(rename_all = "camelCase")]
    RoomChat { msg: String, room_id: String },
}

#[test]
fn t() {
    let expected_json = r#"
    {
        "t": "echo",
        "d": {
            "msg": "()"
        }
    }"#;
    let value = WsRequestMessage::Echo {
        msg: "()".to_string(),
    };
    let expected_json_value = serde_json::from_str::<WsRequestMessage>(&expected_json).unwrap();

    assert_eq!(expected_json_value, value);
}
#[test]
fn t2() {
    let expected_json = r#"
    {
        "t": "wsInitial",
        "d": {
            "jwt": "()"
        }
    }"#;
    let value = WsRequestMessage::Echo {
        msg: "()".to_string(),
    };
    let expected_json_value = serde_json::from_str::<WsRequestMessage>(&expected_json).unwrap();
    println!("{expected_json_value:?}")
    // assert_eq!(expected_json_value, value);
}
