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
        request::ws::{WsDisConnect, WsGetToken, WsTokenVerify, WsTopic},
        response::ApiResponse,
    },
    error::HandlerError,
    model::jwt_claim::AccessClaims,
    pipe::extractor::auth_guard::{AuthGuard, WsGuard},
    service, util,
};

pub fn ws_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        // .route("/ws/initial", post(ws_initial))
        .route("/ws/disconnect", post(ws_disconnect))
        // .route("/ws/conn_list", get(ws_conn_list))
        .route("/ws/temptoken/issue", get(ws_temp_token_issue))
        .route("/ws/temptoken/verify", post(ws_temp_token_verify))
        //
        .route("/ws/token", post(ws_get_token_by_connection_id))
        //
        .route("/ws/topic/subscribe", post(ws_subscribe_topic))
        .route("/ws/topic/unsubscribe", post(ws_unsubscribe_topic))
    //
}

#[derive(OpenApi)]
#[openapi(
    paths( ws_disconnect, ws_temp_token_issue, ws_temp_token_verify, ws_get_token_by_connection_id, ws_subscribe_topic, ws_unsubscribe_topic),
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
        util::jwt::generate_ws_temp_token("Empty")?
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

#[utoipa::path(tag = "ws", post, path = "/token")]
pub async fn ws_get_token_by_connection_id(
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Json(j): Json<WsGetToken>,
) -> Result<impl IntoResponse, HandlerError> {
    let ws_token = service::ws::ws_get_token(&dynamo_client, j.connection_id).await?;
    let data = json!({
        "token": ws_token
    });
    let ret = ApiResponse::success(data);
    Ok(Json(ret))
}

#[utoipa::path(tag = "ws", post, path = "/topic/subscribe")]
pub async fn ws_subscribe_topic(
    WsGuard(w): WsGuard,
    Json(j): Json<WsTopic>,
) -> Result<impl IntoResponse, HandlerError> {
    tracing::info!("ws_subscribe_topic, {j:?}");
    let ret = ApiResponse::success(());
    Ok(Json(ret))
}

#[utoipa::path(tag = "ws", post, path = "/topic/unsubscribe")]
pub async fn ws_unsubscribe_topic(
    WsGuard(w): WsGuard,
    Json(j): Json<WsTopic>,
) -> Result<impl IntoResponse, HandlerError> {
    tracing::info!("ws_unsubscribe_topic, {j:?}");
    let ret = ApiResponse::success(());
    Ok(Json(ret))
}
