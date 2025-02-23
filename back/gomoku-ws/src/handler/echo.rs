use std::time::Instant;

use lambda_http::tracing::info;
use serde_json::json;

use crate::{config::APP_CONFIG, message::response::WsResponseMessage};

pub async fn echo_handler(
    api_gw_client: &aws_sdk_apigatewaymanagement::Client,
    sqs_client: &aws_sdk_sqs::Client,
    connection_id: &str,
    msg: String,
) -> anyhow::Result<()> {
    let a = json!({"msg": msg,"a":"q test"});
    let s = serde_json::to_string(&a)?;
    let start = Instant::now();
    let q_url = APP_CONFIG.get().unwrap().settings.sqs.queue_url.as_str();
    let res = sqs_client
        .send_message()
        .queue_url(q_url)
        .message_body(s)
        .send()
        .await?;
    info!("q res: {res:?}");
    api_gw_client
        .post_to_connection()
        .connection_id(connection_id)
        .data(WsResponseMessage::Echo { msg }.try_into()?)
        .send()
        .await?;
    let duration = start.elapsed();
    info!("코드 실행 시간: {:.2?}", duration);
    Ok(())
}
