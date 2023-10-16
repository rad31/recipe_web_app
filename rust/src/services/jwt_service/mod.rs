use crate::services::jwt_service::error::{Error, Result};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use uuid::Uuid;

pub const JWT_KEY: &[u8; 6] = b"secret";

pub mod error;

type HmacSha256 = Hmac<Sha256>;

pub fn extract_user_id(token: &str) -> Result<Uuid> {
    let key = HmacSha256::new_from_slice(JWT_KEY)?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    let user_id = Uuid::parse_str(&claims["user_id"])?;

    Ok(user_id)
}

pub fn generate_jwt(user_id: &Uuid) -> Result<String> {
    let key = HmacSha256::new_from_slice(JWT_KEY)?;
    let mut claims = BTreeMap::new();
    claims.insert("user_id", user_id.simple().to_string());
    let token_str = claims.sign_with_key(&key)?;

    Ok(token_str)
}
