pub mod error;

use crate::config;
use crate::ctx::Ctx;
use crate::services::jwt_service::error::{Error, Result};
use jwt::{SignWithKey, VerifyWithKey, VerifyingAlgorithm};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use uuid::Uuid;

pub fn extract_ctx(token: &str) -> Result<Ctx> {
    let claims: BTreeMap<String, String> = token.verify_with_key(&config().signing_key)?;
    let now = chrono::Utc::now().timestamp();
    let exp = claims["exp"].parse::<i64>()?;

    if now > exp {
        return Err(Error::AccessTokenExpired);
    }

    let user_id = Uuid::parse_str(&claims["user_id"])?;

    Ok(Ctx { user_id })
}

pub fn generate_jwt(user_id: &Uuid) -> Result<String> {
    let iat = chrono::Utc::now().timestamp();
    let exp = iat + &config().jwt_lifespan_seconds;

    let mut claims = BTreeMap::new();
    claims.insert("user_id", user_id.simple().to_string());
    claims.insert("exp", exp.to_string());
    claims.insert("nbf", iat.to_string());
    claims.insert("iat", iat.to_string());

    let token_str = claims.sign_with_key(&config().signing_key)?;

    Ok(token_str)
}
