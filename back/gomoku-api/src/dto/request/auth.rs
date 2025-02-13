use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignupGuest {
    #[validate(length(
        min = 3,
        max = 10,
        message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."
    ))]
    pub nick_name: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenRefresh {
    pub refresh_token: Option<String>,
}
