use serde::Serialize;
use utoipa::ToSchema;

pub mod auth;
pub mod user;

#[derive(Serialize, ToSchema, Debug)]
pub struct ApiResponse<T = ()> {
    pub message: String,
    pub code: ApiResponseCode,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub const DEFAULT_SUCCESS_MSG: &str = "성공";
    pub fn success(data: T) -> Self {
        Self {
            code: ApiResponseCode::Success,
            message: Self::DEFAULT_SUCCESS_MSG.to_string(),
            data,
        }
    }
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ApiResponseWithMeta<T = (), M = ()> {
    pub message: String,
    pub code: ApiResponseCode,
    pub data: T,
    pub meta: M,
}

impl<T, M> ApiResponseWithMeta<T, M> {
    pub const DEFAULT_SUCCESS_MSG: &str = "성공";
    pub fn success(data: T, meta: M) -> Self {
        Self {
            code: ApiResponseCode::Success,
            message: Self::DEFAULT_SUCCESS_MSG.to_string(),
            data,
            meta,
        }
    }
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ApiErrorResponse {
    pub message: String,
    pub code: ApiResponseCode,
    pub reason: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ApiErrorResponse {
    pub const DEFAULT_FAIL_MSG: &str = "실패";
    pub fn new(reason: serde_json::Value) -> Self {
        Self {
            code: ApiResponseCode::Fail,
            message: Self::DEFAULT_FAIL_MSG.to_string(),
            reason,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Serialize, ToSchema, Debug)]
pub enum ApiResponseCode {
    #[serde(rename = "S")]
    Success,
    #[serde(rename = "F")]
    Fail,
}

// #[derive(Debug, Serialize, ToSchema)]
// pub struct MetaPagination {
//     pagination: PaginationRes,
// }

// impl MetaPagination {
//     pub fn new(page: i64, limit: i64, total: i64) -> Self {
//         Self {
//             pagination: PaginationRes::new(page, limit, total),
//         }
//     }
// }

// impl From<PaginationRes> for MetaPagination {
//     fn from(value: PaginationRes) -> Self {
//         Self { pagination: value }
//     }
// }
