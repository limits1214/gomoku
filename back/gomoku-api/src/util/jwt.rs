use time::{Duration, OffsetDateTime};

use crate::model::jwt_claim::{AccessClaims, RefreshClaims};

pub struct GenAccessTokenArgs {
    pub uid: String,
    pub nick_name: String,
    pub avatar_url: Option<String>,
    pub user_status: String,
    pub user_role: String,
}

pub fn generate_accesss_claim(args: GenAccessTokenArgs) -> AccessClaims {
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let acc_exp = *super::config::get_config_jwt_access_time();

    AccessClaims::new(
        args.uid,
        now + Duration::seconds(acc_exp),
        now,
        None,
        args.nick_name,
        args.avatar_url,
        args.user_status,
        args.user_role,
    )
}
pub fn generate_access_token(
    args: GenAccessTokenArgs,
) -> Result<String, jsonwebtoken::errors::Error> {
    let access_claims = generate_accesss_claim(args);
    let acc = super::config::get_config_jwt_access_keys();
    let access_token = acc.encode(&access_claims)?;
    Ok(access_token)
}

pub struct GenRefreshTokenArgs {
    pub id: String,
}

pub fn generate_refresh_token(
    args: GenRefreshTokenArgs,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let refr_exp = *super::config::get_config_jwt_refresh_time();
    let refresh_claims = RefreshClaims::new(args.id, now + Duration::seconds(refr_exp), now, None);
    let refr = super::config::get_config_jwt_refresh_keys();
    let refresh_token = refr.encode(&refresh_claims)?;
    Ok(refresh_token)
}
