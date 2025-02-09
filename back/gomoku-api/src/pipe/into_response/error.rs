use crate::{dto::response::ApiErrorResponse, error::HandlerError};
use axum::{response::IntoResponse, Json};
use axum_extra::typed_header::TypedHeaderRejection;
use hyper::StatusCode;
use lambda_http::tracing;
use serde_json::json;

impl IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        const TAG: &str = "[HandlerError]";
        let (stt, value) = match &self {
            Self::Validation(err) => {
                tracing::error!("{TAG} vali err: {:?}", err);
                let mut value = json!({
                    "msg": "validate fail"
                });
                err.field_errors().into_iter().for_each(|(k, v)| {
                    let vv = v
                        .iter()
                        .map(|ve| match (&ve.message, &ve.code) {
                            (Some(message), code) => {
                                format!("{};{}", code.to_string(), message.to_string())
                            }
                            _ => String::new(),
                        })
                        .filter(|f| !f.is_empty())
                        .collect::<Vec<_>>();
                    value[k] = json!(vv);
                });

                (StatusCode::BAD_REQUEST, value)
            }
            Self::Anyhow(anyhow_err) => {
                anyhow_err.chain().enumerate().for_each(|(i, cause)| {
                    tracing::error!(
                        "{TAG} anyhow-{i}, err: {:?}, cause: {:?}",
                        anyhow_err,
                        cause
                    );
                });

                let stt = match anyhow_err.root_cause() {
                    e if e.is::<TypedHeaderRejection>() => StatusCode::BAD_REQUEST,
                    e if e.is::<jsonwebtoken::errors::Error>() => StatusCode::UNAUTHORIZED,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (
                    stt,
                    json!({
                        "msg": anyhow_err.to_string()
                    }),
                )
            }
            HandlerError::Dynamo(error) => {
                tracing::error!("{TAG} vali err: {:?}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({
                        "msg": error.to_string()
                    }),
                )
            }
        };

        (stt, Json(ApiErrorResponse::new(value))).into_response()
    }
}
