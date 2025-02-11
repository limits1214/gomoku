use std::collections::HashMap;

use crate::{
    constant::{PK, SK},
    model::jwt_claim::RefreshClaims,
    util::{
        self,
        jwt::{GenAccessTokenArgs, GenRefreshTokenArgs},
    },
};
use anyhow::bail;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Local;
use nanoid::nanoid;

pub async fn signup_guest(
    dynamo_client: &aws_sdk_dynamodb::Client,
    nick_name: String,
) -> anyhow::Result<(String, String)> {
    let table_name = util::dynamo::get_table_name();
    let time = Local::now().to_rfc3339();
    let nanoid = nanoid!();
    let pk = format!("USER#{nanoid}");
    let status = "OK".to_string();
    let ty = "GUEST".to_string();
    let role = "USER".to_string();
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(PK.to_string(), AttributeValue::S(pk));
    item.insert(SK.to_string(), AttributeValue::S("INFO".to_string()));

    item.insert("type".to_string(), AttributeValue::S(ty));
    item.insert("createdAt".to_string(), AttributeValue::S(time));
    item.insert("nickName".to_string(), AttributeValue::S(nick_name.clone()));
    item.insert("role".to_string(), AttributeValue::S(role.clone()));
    item.insert("status".to_string(), AttributeValue::S(status.clone()));

    let _output = dynamo_client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await?;

    let access_token = util::jwt::generate_access_token(GenAccessTokenArgs {
        avatar_url: None,
        nick_name: nick_name,
        uid: nanoid.clone(),
        user_status: status,
        user_role: role,
    })?;

    let refresh_token =
        util::jwt::generate_refresh_token(GenRefreshTokenArgs { id: nanoid.clone() })?;
    let refresh_token_hash = util::hash::hash_sha_256(&refresh_token);

    let session_pk = format!("SESSION#{refresh_token_hash}");
    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert(PK.to_string(), AttributeValue::S(session_pk));
    item.insert(SK.to_string(), AttributeValue::S("INFO".to_string()));
    item.insert("jwt".to_string(), AttributeValue::S(refresh_token.clone()));
    item.insert("userId".to_string(), AttributeValue::S(nanoid.clone()));

    let _output = dynamo_client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await?;

    Ok((access_token, refresh_token_hash))
}

pub async fn access_token_refresh(
    dynamo_client: &aws_sdk_dynamodb::Client,
    refresh_token_hash: String,
) -> anyhow::Result<String> {
    let table_name = util::dynamo::get_table_name();
    let response = dynamo_client
        .get_item()
        .table_name(table_name)
        .key(
            PK,
            AttributeValue::S(format!("SESSION#{refresh_token_hash}")),
        )
        .key(SK, AttributeValue::S("INFO".to_string()))
        .send()
        .await?;

    let Some(output) = response.item else {
        bail!("RefreshTokenNotExists");
    };
    let Some(refresh_jwt) = output.get("jwt") else {
        bail!("RefreshTokenNotExists2");
    };
    let Ok(refresh_jwt) = refresh_jwt.as_s() else {
        bail!("RefreshTokenNotExists3");
    };
    let refresh_jwt =
        util::config::get_config_jwt_refresh_keys().decode::<RefreshClaims>(refresh_jwt)?;

    let user_id = refresh_jwt.claims.sub;

    let response = dynamo_client
        .get_item()
        .table_name(table_name)
        .key(PK, AttributeValue::S(format!("USER#{user_id}")))
        .key(SK, AttributeValue::S("INFO".to_string()))
        .send()
        .await?;
    let Some(output) = response.item else {
        bail!("UserNotExists");
    };

    let Some(nick_name) = output.get("nickName") else {
        bail!("NICK_NOT_EXISTS")
    };
    let Ok(nick_name) = nick_name.as_s() else {
        bail!("NICK_NOT_EXISTS")
    };

    let Some(role) = output.get("role") else {
        bail!("ROLE_NOT_EXISTS")
    };
    let Ok(role) = role.as_s() else {
        bail!("ROLE_NOT_EXISTS")
    };

    let Some(status) = output.get("status") else {
        bail!("STATUS_NOT_EXISTS")
    };
    let Ok(status) = status.as_s() else {
        bail!("STATUS_NOT_EXISTS")
    };

    let access_token = util::jwt::generate_access_token(GenAccessTokenArgs {
        avatar_url: None,
        nick_name: nick_name.clone(),
        uid: user_id.clone(),
        user_status: status.clone(),
        user_role: role.clone(),
    })?;

    Ok(access_token)
}
