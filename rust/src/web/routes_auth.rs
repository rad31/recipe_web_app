use crate::{models::{requests::LoginRequest, ModelManager}, web::error::{Error, Result}, services::{auth_service, jwt_service}};
use axum::{Json, routing::post, Router, extract::State, response::Response};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};
use tracing::debug;
use uuid::Uuid;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/auth/signup", post(sign_up))
        .route("/api/auth/login", post(login))
        .with_state(mm)
}

async fn sign_up(
    State(mm): State<ModelManager>,
    Json(req): Json<LoginRequest>
) -> Result<Json<Value>> {
    debug!("{:<12} - sign up", "HANDLER");

    let user_id = auth_service::sign_up(&mm, req).await?;
    let access_token = jwt_service::generate_jwt(&user_id)?;
    let res = Json(json!({ "access_token": access_token }));
    
    Ok(res)
}

async fn login(
    State(mm): State<ModelManager>,
    Json(req): Json<LoginRequest>
) -> Result<Json<Value>> {
    debug!("{:<12} - login", "HANDLER");

    let user_id = auth_service::login(&mm, req).await?;
    let access_token = jwt_service::generate_jwt(&user_id)?;
    let res = Json(json!({ "access_token": access_token }));
    
    Ok(res)
}
