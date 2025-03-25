use axum::{
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use lambda_http::tracing;
use serde_json::json;
use time::Duration;
use utoipa::OpenApi;

use crate::config::app_state::ArcAppState;

pub fn test_router(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new()
        .route("/test/greet", get(greet))
        .route("/test/cookie", post(cookie))
        .route("/test/cookie", get(cookie2))
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

#[utoipa::path(tag = "test", get, path = "/cookie")]
pub async fn cookie(jar: CookieJar) -> impl IntoResponse {
    let test_cookie = jar.get("TEST_COOKIE");

    let cnt = if let Some(cookie) = test_cookie {
        let v = cookie.value();
        tracing::info!("cookie!! {}", v);
        let mut i = v.parse::<i32>().unwrap();
        i += 1;
        let istr = i.to_string();
        istr
    } else {
        "1".to_string()
    };

    let a = Cookie::build(("TEST_COOKIE", cnt.clone()))
        .path("/")
        .http_only(true)
        // .domain("https://lsy969999.github.io")
        .same_site(SameSite::None)
        .secure(true)
        .max_age(Duration::seconds(60))
        .build();

    (
        jar.add(a),
        Json(json!({
            "msg": format!("TEST_COOKIE: {cnt}")
        })),
    )
}

#[utoipa::path(tag = "test", get, path = "/cookie2")]
pub async fn cookie2(jar: CookieJar) -> impl IntoResponse {
    let test_cookie = jar.get("TEST_COOKIE");

    let cnt = if let Some(cookie) = test_cookie {
        let v = cookie.value();
        tracing::info!("cookie!! {}", v);
        let mut i = v.parse::<i32>().unwrap();
        i += 1;
        let istr = i.to_string();
        istr
    } else {
        "1".to_string()
    };

    let a = Cookie::build(("TEST_COOKIE", cnt.clone()))
        .path("/")
        .http_only(true)
        // .domain("https://lsy969999.github.io")
        .max_age(Duration::seconds(60))
        .same_site(SameSite::None)
        .secure(true)
        .build();

    (jar.add(a), Redirect::to("https://gogomoku.vercel.app"))
}
