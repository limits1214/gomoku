use std::time::Instant;

use lambda_http::tracing::info;

use crate::message::response::WsResponseMessage;

pub async fn echo_handler(
    api_gw_client: aws_sdk_apigatewaymanagement::Client,
    connection_id: &str,
    msg: String,
) -> anyhow::Result<()> {
    let start = Instant::now();
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
