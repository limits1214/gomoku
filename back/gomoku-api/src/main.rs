use axum::{routing::get, Json, Router};
use lambda_http::{run, tracing};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    std::env::set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(|| async { Json(json!({"msg":"hello3"})) }))
        .route("/hello", get(|| async { Json(json!({"msg":"hello"})) }));

    run(app).await?;

    Ok(())
}
