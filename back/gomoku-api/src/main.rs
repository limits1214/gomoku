#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    gomoku_api::start_app().await?;
    Ok(())
}
