use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
    pub access_token: String,
}
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenWithIsFirst {
    pub access_token: String,
    pub is_first: bool,
}
