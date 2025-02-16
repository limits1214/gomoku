use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use utoipa::OpenApi;

use crate::{
    config::app_state::ArcAppState,
    dto::{
        request::room::{ChannelRoomInfo, CreateRoom, RoomList},
        response::ApiResponse,
    },
    error::HandlerError,
    service,
};

pub fn room_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        .route("/room/create", post(create_room))
        .route("/room/list", get(room_list))
        .route("/room/info/{roomId}", get(room_info))
        .route("/room/info/channelroom", get(channel_room_info))
}

#[derive(OpenApi)]
#[openapi(
    paths(create_room, room_list, room_info, channel_room_info),
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
        "paginationKey": room_list.1
    });
    let ret = ApiResponse::success(data);
    Ok(Json(ret))
}
#[utoipa::path(
    tag = "room",
    get,
    path = "/info/{roomId}",
    params(
        ("roomId" = String, Path),
    )
)]
async fn room_info(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Path(p): Path<String>,
) -> Result<impl IntoResponse, HandlerError> {
    let room_info = service::room::room_info(&dynamo_client, &p).await?;
    let ret = ApiResponse::success(room_info);
    Ok(Json(ret))
}
#[utoipa::path(
    tag = "room",
    get,
    path = "/info/channelroom",
    params(
        ("channel" = String, Query, example = 1),
        ("roomNum" = String, Query, example = 1),
    )
)]
async fn channel_room_info(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Query(q): Query<ChannelRoomInfo>,
) -> Result<impl IntoResponse, HandlerError> {
    let room_info =
        service::room::channel_room_info(&dynamo_client, &q.channel, &q.room_num).await?;
    let ret = ApiResponse::success(room_info);
    Ok(Json(ret))
}
