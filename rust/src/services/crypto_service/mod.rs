pub mod error;

use crate::services::crypto_service::error::{Error, Result};
use getrandom::getrandom;
use hmac::digest::core_api::{CoreWrapper, CtVariableCoreWrapper};
use hmac::digest::typenum::{UInt, UTerm, B1, B0};
use hmac::{Hmac, Mac, HmacCore};
use sha2::{OidSha256, Sha256, Sha256VarCore};


type HmacSha256 = Hmac<Sha256>;
pub type Key = CoreWrapper<HmacCore<CoreWrapper<CtVariableCoreWrapper<Sha256VarCore, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, OidSha256>>>>;

pub fn generate_signing_key(secret: String) -> Result<Key> {
    let config = secret.as_bytes();
    let key = Hmac::<Sha256>::new_from_slice(config)?;

    Ok(key)
}

// pub fn hash_password(password: &str, hash_cost: u32) -> Result<(String, String)> {
//     let res = bcrypt::hash_with_result(password, hash_cost)?;
//     let salt = res.get_salt();
//     let hash = res.format_for_version(bcrypt::Version::TwoA);
//     Ok((hash, salt))
// }

pub fn hash_password(password: &str, hash_cost: u32) -> Result<String> {
    let hash = bcrypt::hash(password, hash_cost)?;
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let result = bcrypt::verify(password, hash)?;
    Ok(result)
}