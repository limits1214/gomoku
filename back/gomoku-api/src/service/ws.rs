use crate::{
    constant::{PK, SK},
    util::{self, dynamo::DynamoMapHelper},
};
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Local;

// ws 최초 연결, connection_id, jwt 등록
// case1: jwt 존재, case2: jwt 미존재
// TODO: connection_id 에대한 검증
pub async fn ws_initial(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: String,
    jwt: Option<String>,
) -> anyhow::Result<()> {
    if let Some(jwt) = jwt {
        let claims = util::jwt::decode_access(&jwt)?;
        let user_id = claims.sub;
        let ws_conn_pk = format!("WS_CONN#{connection_id}");
        let ws_conn_sk = format!("USER#{user_id}");
        let time = Local::now().to_rfc3339();
        let map = DynamoMapHelper::new()
            .insert_pk(ws_conn_pk)
            .insert_sk(ws_conn_sk)
            .insert_attr_s("jwt", &jwt)
            .insert_attr_s("createdAt", &time)
            .build();

        let output = dynamo_client
            .put_item()
            .table_name(util::dynamo::get_table_name())
            .set_item(Some(map))
            .send()
            .await?;

        let user_pk = format!("USER#{user_id}");
        let user_sk = format!("WS_CONN#{connection_id}");
        let map = DynamoMapHelper::new()
            .insert_pk(user_pk)
            .insert_sk(user_sk)
            .insert_attr_s("createdAt", &time)
            .build();

        let output = dynamo_client
            .put_item()
            .table_name(util::dynamo::get_table_name())
            .set_item(Some(map))
            .send()
            .await?;
    } else {
        let ws_conn_pk = format!("WS_CONN#{connection_id}");
        let ws_conn_sk = format!("USER#_");
        let time = Local::now().to_rfc3339();
        let map = DynamoMapHelper::new()
            .insert_pk(ws_conn_pk)
            .insert_sk(ws_conn_sk)
            .insert_attr_s("createdAt", &time)
            .build();

        let output = dynamo_client
            .put_item()
            .table_name(util::dynamo::get_table_name())
            .set_item(Some(map))
            .send()
            .await?;
    };

    Ok(())
}

// ws 끊어짐
pub async fn ws_disconnect(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: String,
    jwt: Option<String>,
) -> anyhow::Result<()> {
    // 1. USER 제거 // jwt 존재시
    // 2. WS_CONN 제거
    // 3. WS_TOPIC 제거(with broadcast?)

    if let Some(jwt) = jwt {
        let claims = util::jwt::decode_access(&jwt)?;
        let user_id = claims.sub;

        // WS_CONN 의 USER 제거
        let delete_ws_conn_pk = format!("WS_CONN#{connection_id}");
        let delete_ws_conn_sk = format!("USER#{user_id}");
        let output = dynamo_client
            .delete_item()
            .table_name(util::dynamo::get_table_name())
            .key(PK, AttributeValue::S(delete_ws_conn_pk))
            .key(SK, AttributeValue::S(delete_ws_conn_sk))
            .send()
            .await?;

        // USER의 WS_CONN 제거
        let delete_user_pk = format!("USER#{user_id}");
        let delete_user_sk = format!("WS_CONN#{connection_id}");
        let output = dynamo_client
            .delete_item()
            .table_name(util::dynamo::get_table_name())
            .key(PK, AttributeValue::S(delete_user_pk))
            .key(SK, AttributeValue::S(delete_user_sk))
            .send()
            .await?;
    }

    // 혹시 USER_ID#_ 가 존재할수 있으니 지워준다.
    let delete_ws_conn_pk = format!("WS_CONN#{connection_id}");
    let delete_ws_conn_sk = format!("USER#_");
    let output = dynamo_client
        .delete_item()
        .table_name(util::dynamo::get_table_name())
        .key(PK, AttributeValue::S(delete_ws_conn_pk))
        .key(SK, AttributeValue::S(delete_ws_conn_sk))
        .send()
        .await?;

    Ok(())
}

// jwt 업데이트
pub fn ws_jwt_set(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: String,
) -> anyhow::Result<()> {
    Ok(())
}

// ws topic 세팅
pub fn ws_topic_set(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: String,
) -> anyhow::Result<()> {
    Ok(())
}

// ws topic 제거
pub fn ws_topic_unset(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: String,
) -> anyhow::Result<()> {
    Ok(())
}
