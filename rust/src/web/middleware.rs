use std::collections::BTreeMap;

use crate::{web::error::{Error, Result}, ctx::Ctx, services::jwt_service};
use async_trait::async_trait;
use axum::{http::{Request, HeaderValue, request::Parts, HeaderMap}, middleware::Next, response::Response, extract::FromRequestParts, RequestPartsExt};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use tracing::debug;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

pub async fn require_auth<T>(ctx: Result<Ctx>, req: Request<T>, next: Next<T>)
    -> Result<Response> {
    debug!("{:<12} - require_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - ctx", "EXTRACTOR");
        
        let headers = parts.extract::<HeaderMap>().await?;
        let auth_header = match headers.get("Authorization") {
            Some(header) => header.to_str()?,
            None => return Err(Error::AccessTokenNotFound)
        };
        let token = match &auth_header.len() {
            7.. => &auth_header[7..],
            _ => return Err(Error::AccessTokenInvalid)
        };

        Ok(Ctx {
            user_id: jwt_service::extract_user_id(token)?
        })
    }
}
