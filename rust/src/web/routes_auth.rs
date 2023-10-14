use crate::{error::Error, error::Result, models::requests::LoginRequest, web};
use axum::{Json, routing::post, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};
use tracing::debug;
use uuid::Uuid;

pub fn routes() -> Router {
    Router::new()
        .route("/api/signup", post(sign_up))
        .route("/api/login", post(login))
}

async fn sign_up(req: Json<LoginRequest>) -> Result<Json<Value>> {
    debug!("{:<12} - login", "HANDLER");

    let user_id = Uuid::new_v4();
    let body = Json(json!({ "success": true }));
    
    Ok(body)
}

async fn login(req: Json<LoginRequest>) -> Result<Json<Value>> {
    println!("-> {:<12} - login", "HANDLER");

    let body = Json(json!({ "success": true }));
    
    Ok(body)
}
