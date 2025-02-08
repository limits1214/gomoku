use axum::{
    http::{header, Method},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, limit::RequestBodyLimitLayer};

pub async fn create_app() -> Router {
    Router::new()
        .route("/", get(|| async { Json(json!({"msg":"hello4"})) }))
        .route("/hello", get(|| async { Json(json!({"msg":"hellozz"})) }))
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(1024 * 1024))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:5174".parse().unwrap(),
                    "http://localhost:4173".parse().unwrap(),
                    "https://lsy969999.github.io".parse().unwrap(),
                ])
                .allow_credentials(true)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::OPTIONS,
                    Method::PUT,
                    Method::DELETE,
                    Method::PATCH,
                ])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
        )
}
