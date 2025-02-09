use axum::Router;
use utoipa::OpenApi;

use crate::config::app_state::ArcAppState;

pub fn user_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
}

#[derive(OpenApi)]
#[openapi(
    paths(),
    tags(
        (name = "user", description = "user desc"),
    ),
)]
pub(super) struct UserApi;

pub async fn user_info() {
    //
}
