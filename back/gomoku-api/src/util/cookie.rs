use axum_extra::extract::cookie::{Cookie, SameSite};
use time::Duration;

use crate::constant::REFRESH_TOKEN;

pub fn generate_refresh_token_cookie(token_str: String) -> Cookie<'static> {
    let refr_time = super::config::get_config_jwt_refresh_time();
    let secure = super::config::get_cookie_secure();
    Cookie::build((REFRESH_TOKEN, token_str))
        .path("/")
        // .http_only(true)
        // .same_site(SameSite::None)
        .secure(*secure)
        .max_age(Duration::seconds(*refr_time))
        .build()
}

pub fn generate_refresh_token_remove_cookie() -> Cookie<'static> {
    let secure = super::config::get_cookie_secure();
    Cookie::build((REFRESH_TOKEN, ""))
        .path("/")
        // .http_only(true)
        // .same_site(SameSite::None)
        .secure(*secure)
        .max_age(Duration::seconds(0))
        .build()
}
