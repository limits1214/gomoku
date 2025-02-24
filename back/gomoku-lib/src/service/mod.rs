use chrono::Local;

use crate::util::{self, dynamo::DynamoMapHelper};

pub async fn put_empty_conn(connection_id: &str) -> anyhow::Result<()> {
    let ws_conn_pk = format!("WS_CONN#{connection_id}");
    let ws_conn_sk = format!("INFO");
    let time = Local::now().to_rfc3339();
    let map = DynamoMapHelper::new()
        .insert_pk(ws_conn_pk)
        .insert_sk(ws_conn_sk)
        .insert_attr_s("createdAt", &time)
        .build();
    let dynamo_client = util::dynamo::get_dynamo_client();
    let _output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;
    Ok(())
}
pub async fn put_conn(connection_id: &str, user_id: &str) -> anyhow::Result<()> {
    let ws_conn_pk = format!("WS_CONN#{connection_id}");
    let ws_conn_sk = format!("INFO");
    let time = Local::now().to_rfc3339();
    let map = DynamoMapHelper::new()
        .insert_pk(ws_conn_pk)
        .insert_sk(ws_conn_sk)
        .insert_attr_s("createdAt", &time)
        .insert_attr_s("userId", &user_id)
        .build();
    let dynamo_client = util::dynamo::get_dynamo_client();
    let _output = dynamo_client
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

    let _output = dynamo_client
        .put_item()
        .table_name(util::dynamo::get_table_name())
        .set_item(Some(map))
        .send()
        .await?;
    Ok(())
}
