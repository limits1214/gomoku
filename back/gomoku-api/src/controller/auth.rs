use crate::{
    config::app_state::ArcAppState,
    constant::REFRESH_TOKEN,
    dto::{
        self,
        request::auth::{AccessTokenRefresh, SignupGuest},
        response::ApiResponse,
    },
    error::HandlerError,
    service, util,
};
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use axum_extra::extract::CookieJar;
use serde_json::json;
use utoipa::OpenApi;
use validator::Validate;

pub fn auth_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        .route("/auth/signup/guest", post(signup_guest))
        .route("/auth/access/refresh", post(access_token_refresh))
}

#[derive(OpenApi)]
#[openapi(
    paths(signup_guest, access_token_refresh),
    tags(
        (name = "auth", description = "auth desc"),
    ),
)]
pub(super) struct AuthApi;

#[utoipa::path(tag = "auth", post, path = "/signup/guest")]
pub async fn signup_guest(
    jar: CookieJar,
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Json(j): Json<SignupGuest>,
) -> Result<impl IntoResponse, HandlerError> {
    j.validate()?;

    let (access_token, refresh_token_hash) =
        service::auth::signup_guest(&dynamo_client, j.nick_name).await?;

    let ref_token_cookie = util::cookie::generate_refresh_token_cookie(refresh_token_hash);
    let jar: CookieJar = jar.add(ref_token_cookie);
    let ret = ApiResponse::success(dto::response::auth::AccessToken { access_token });
    Ok((jar, Json(ret)))
}

#[utoipa::path(tag = "auth", post, path = "/access/refresh")]
pub async fn access_token_refresh(
    jar: CookieJar,
    dynamo_client: State<aws_sdk_dynamodb::Client>,
    Json(j): Json<AccessTokenRefresh>,
) -> Result<impl IntoResponse, HandlerError> {
    let refresh_token = match jar.get(REFRESH_TOKEN) {
        Some(rt) => rt.value().to_string(),
        None => {
            let Some(rt) = j.refresh_token else {
                return Ok((
                    jar,
                    Json(ApiResponse::success(json!({
                        "msg": "RefreshTokenEmpty"
                    }))),
                )
                    .into_response());
            };
            rt
        }
    };
    let access_token =
        service::auth::access_token_refresh(&dynamo_client, refresh_token.to_string()).await?;
    let ret = ApiResponse::success(dto::response::auth::AccessToken { access_token });
    Ok((jar, Json(ret)).into_response())
}
