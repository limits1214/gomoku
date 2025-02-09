use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use utoipa::OpenApi;

use crate::{
    config::app_state::ArcAppState, dto::response::ApiResponse, error::HandlerError, service,
};

pub fn room_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        .route("/room/create", post(create_room))
        .route("/room/list", get(room_list))
}

#[derive(OpenApi)]
#[openapi(
    paths(create_room, room_list),
    tags(
        (name = "room", description = "room desc"),
    ),
)]
pub(super) struct RoomApi;

#[utoipa::path(tag = "room", post, path = "/create")]
async fn create_room(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
) -> Result<impl IntoResponse, HandlerError> {
    service::room::create_room(&dynamo_client).await?;
    Ok(Json(ApiResponse::success(())))
}

#[utoipa::path(tag = "room", get, path = "/list")]
async fn room_list(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
) -> Result<impl IntoResponse, HandlerError> {
    service::room::room_list(&dynamo_client).await?;
    Ok(())
}
