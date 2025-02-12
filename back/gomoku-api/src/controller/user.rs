use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use utoipa::OpenApi;

use crate::{
    config::app_state::ArcAppState, dto::response::ApiResponse, error::HandlerError,
    pipe::extractor::auth_guard::AuthGuard,
};

pub fn user_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new().route("/user/info", get(user_info))
}

#[derive(OpenApi)]
#[openapi(
    paths(user_info),
    tags(
        (name = "user", description = "user desc"),
    ),
)]
pub(super) struct UserApi;

#[utoipa::path(tag = "user", get, path = "/info")]
pub async fn user_info(AuthGuard(a): AuthGuard) -> Result<impl IntoResponse, HandlerError> {
    let ret = ApiResponse::success(json!({
        "userId": a.sub
    }));
    Ok(Json(ret))
}
