use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use utoipa::OpenApi;

use crate::{
    config::app_state::ArcAppState,
    dto::{
        request::ws::{WsConnect, WsDisConnect, WsTokenVerify},
        response::ApiResponse,
    },
    error::HandlerError,
    model::jwt_claim::AccessClaims,
    pipe::extractor::auth_guard::AuthGuard,
    service, util,
};

pub fn ws_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        // .route("/ws/initial", post(ws_initial))
        .route("/ws/disconnect", post(ws_disconnect))
        .route("/ws/conn_list", get(ws_conn_list))
        .route("/ws/temptoken/issue", get(ws_temp_token_issue))
        .route("/ws/temptoken/verify", post(ws_temp_token_verify))
}

#[derive(OpenApi)]
#[openapi(
    paths(ws_initial, ws_disconnect, ws_temp_token_issue, ws_temp_token_verify),
    tags(
        (name = "ws", description = "ws desc"),
    ),
)]
pub(super) struct WsApi;

#[utoipa::path(tag = "ws", get, path = "/temptoken/issue")]
pub async fn ws_temp_token_issue(
    auth_guard: Option<AuthGuard>,
) -> Result<impl IntoResponse, HandlerError> {
    let token = if let Some(AuthGuard(AccessClaims { sub, .. })) = auth_guard {
        util::jwt::generate_ws_temp_token(&sub)?
    } else {
        util::jwt::generate_ws_temp_token("Guest")?
    };
    let data = json!({
        "token": token
    });
    let ret = ApiResponse::success(data);
    Ok(Json(ret))
}

#[utoipa::path(tag = "ws", post, path = "/temptoken/verify")]
pub async fn ws_temp_token_verify(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Json(j): Json<WsTokenVerify>,
) -> Result<impl IntoResponse, HandlerError> {
    // todo: connection_id verify from apigatemanagerment
    let ws = util::jwt::decode_ws_temp(&j.token)?;
    service::ws::ws_token_verify(&dynamo_client, &j.connection_id, &ws.sub).await?;
    let ret = ApiResponse::success(());
    Ok(Json(ret))
}

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
    service::ws::ws_disconnect(&dynamo_client, j.connection_id).await?;

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
