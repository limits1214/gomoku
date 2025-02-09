use axum::{response::IntoResponse, routing::get, Json, Router};
use lambda_http::tracing;
use serde_json::json;
use utoipa::OpenApi;

use crate::config::app_state::ArcAppState;

pub fn test_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new().route("/test/greet", get(greet))
}

#[derive(OpenApi)]
#[openapi(
    paths(greet),
    tags(
        (name = "test", description = "greet desc"),
    ),
)]
pub(super) struct TestApi;

#[utoipa::path(tag = "test", get, path = "/greet")]
pub async fn greet() -> impl IntoResponse {
    tracing::info!("greet!!");
    Json(json!({
        "msg": "greet!"
    }))
}
