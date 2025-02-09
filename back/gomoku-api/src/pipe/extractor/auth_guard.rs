use crate::{
    config::app_state::ArcAppState, error::HandlerError, model::jwt_claim::AccessClaims, util,
};
use anyhow::anyhow;
use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

#[derive(Debug)]
pub struct ApiAuthGuard(pub AccessClaims);

impl<S> FromRequestParts<S> for ApiAuthGuard
where
    ArcAppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = HandlerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let bearer_token =
            <TypedHeader<Authorization<Bearer>> as FromRequestParts<S>>::from_request_parts(
                parts, state,
            )
            .await
            .map_err(|err| anyhow!(err))?;
        let token = util::config::get_config_jwt_access_keys()
            .decode::<AccessClaims>(bearer_token.token())
            .map_err(|err| anyhow!(err))?;

        Ok(Self(token.claims))
    }
}

impl<S> OptionalFromRequestParts<S> for ApiAuthGuard
where
    ArcAppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = HandlerError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let bearer_token =
            <TypedHeader<Authorization<Bearer>> as OptionalFromRequestParts<S>>::from_request_parts(
                parts, state,
            )
            .await
            .map_err(|err| anyhow!(err))?;
        let bearer_token = match bearer_token {
            Some(bearer_token) => bearer_token,
            None => return Ok(None),
        };
        let token =
            util::config::get_config_jwt_access_keys().decode::<AccessClaims>(bearer_token.token());
        match token {
            Ok(token) => Ok(Some(Self(token.claims))),
            Err(_) => Ok(None),
        }
    }
}
