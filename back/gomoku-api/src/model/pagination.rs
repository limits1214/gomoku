use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationReq {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

impl PaginationReq {
    pub fn new(page: i32, limit: i32) -> Self {
        Self {
            page: Some(page),
            limit: Some(limit),
        }
    }
}

const DEFAULT_PAGE: i32 = 1;
const DEFAULT_LIMIT: i32 = 10;
pub struct PaginationDb {
    pub page: i64,
    pub limit: i64,
    pub offset: i64,
}

impl From<&PaginationReq> for PaginationDb {
    fn from(value: &PaginationReq) -> Self {
        let page: i64 = value.page.unwrap_or(DEFAULT_PAGE).into();
        let limit: i64 = value.limit.unwrap_or(DEFAULT_LIMIT).into();
        let offset: i64 = (page - 1) * limit;
        Self {
            page,
            limit,
            offset,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationRes {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

impl PaginationRes {
    pub fn new(page: i64, limit: i64, total: i64) -> Self {
        Self { page, limit, total }
    }
}
