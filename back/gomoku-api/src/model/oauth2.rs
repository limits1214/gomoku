use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GoogleOauth2UserInfo {
    pub sub: String,
    #[validate(length(
        min = 3,
        max = 10,
        message = "닉네임은 3글자 이상 10글자 미만 이어야 합니다."
    ))]
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
}
