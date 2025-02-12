use crate::message::response::WsResponseMessage;

pub async fn echo_handler(
    api_gw_client: aws_sdk_apigatewaymanagement::Client,
    connection_id: &str,
    msg: String,
) -> anyhow::Result<()> {
    api_gw_client
        .post_to_connection()
        .connection_id(connection_id)
        .data(WsResponseMessage::Echo { msg }.try_into()?)
        .send()
        .await?;
    Ok(())
}
