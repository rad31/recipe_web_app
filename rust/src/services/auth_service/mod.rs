use crate::{models::{ModelManager, requests::LoginRequest, db::User}, config};
use crate::services::auth_service::error::{Error, Result};
use crate::services::crypto_service::{hash_password, verify_password};
use tracing::debug;
use uuid::Uuid;

pub mod error;

pub async fn sign_up(mm: &ModelManager, req: LoginRequest) -> Result<Uuid> {
    let db = mm.db();
    let password_hash = hash_password(&req.password, config().password_hash_cost.clone())?;
    let user_id = Uuid::new_v4();
    
    let query = "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(user_id)
        .bind(req.username)
        .bind(password_hash)
        .execute(db)
        .await?;

    Ok(user_id)
}

pub async fn login(mm: &ModelManager, req: LoginRequest) -> Result<Uuid> {
    let db = mm.db();
    
    let query = "SELECT id, username, password_hash FROM users WHERE username = $1";
    let user = sqlx::query_as::<_, User>(query)
        .bind(req.username)
        .fetch_one(db)
        .await?;

    let authorized = verify_password(&req.password, &user.password_hash)?;

    match authorized {
        true => Ok(user.id),
        false => Err(Error::PasswordInvalid),
    }
}
