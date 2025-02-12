use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use lambda_http::tracing;
use serde_json::json;
use utoipa::OpenApi;

use crate::{
    config::app_state::ArcAppState,
    dto::{
        request::ws::{WsConnect, WsDisConnect},
        response::ApiResponse,
    },
    error::HandlerError,
    service,
};

pub fn ws_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        .route("/ws/initial", post(ws_initial))
        .route("/ws/disconnect", post(ws_disconnect))
        .route("/ws/conn_list", get(ws_conn_list))
}

#[derive(OpenApi)]
#[openapi(
    paths(ws_initial, ws_disconnect),
    tags(
        (name = "ws", description = "ws desc"),
    ),
)]
pub(super) struct WsApi;

#[utoipa::path(tag = "ws", post, path = "/initial")]
pub async fn ws_initial(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Json(j): Json<WsConnect>,
) -> Result<impl IntoResponse, HandlerError> {
    service::ws::ws_initial(&dynamo_client, j.connection_id, j.jwt).await?;
    let ret = ApiResponse::success(());
    Ok(Json(ret))
}

#[utoipa::path(tag = "ws", post, path = "/disconnect")]
pub async fn ws_disconnect(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Json(j): Json<WsDisConnect>,
) -> Result<impl IntoResponse, HandlerError> {
    service::ws::ws_disconnect(&dynamo_client, j.connection_id, j.jwt).await?;
    let ret = ApiResponse::success(());
    Ok(Json(ret))
}
#[utoipa::path(tag = "ws", get, path = "/conn_list")]
pub async fn ws_conn_list(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
) -> Result<impl IntoResponse, HandlerError> {
    let ret = ApiResponse::success(());
    Ok(Json(ret))
}
