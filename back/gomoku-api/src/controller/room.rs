use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use utoipa::OpenApi;

use crate::{
    config::app_state::ArcAppState,
    dto::{
        request::room::{CreateRoom, RoomList},
        response::ApiResponse,
    },
    error::HandlerError,
    service,
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
    Json(j): Json<CreateRoom>,
) -> Result<impl IntoResponse, HandlerError> {
    service::room::create_room(&dynamo_client, &j.room_name, &j.channel).await?;
    Ok(Json(ApiResponse::success(())))
}

#[utoipa::path(
    tag = "room",
    get,
    path = "/list",
    params(
        ("channel" = String, Query, example = 1),
        ("paginationKey" = Option<String>, Query),
    )
)]
async fn room_list(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Query(j): Query<RoomList>,
) -> Result<impl IntoResponse, HandlerError> {
    let room_list = service::room::room_list(&dynamo_client, &j.channel, j.pagination_key).await?;
    let data = json!({
        "list": room_list.0,
        "pagination_key": room_list.1
    });
    let ret = ApiResponse::success(data);
    Ok(Json(ret))
}
