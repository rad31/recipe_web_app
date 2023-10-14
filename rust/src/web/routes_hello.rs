use crate::{error::Error, error::Result, models::requests::LoginRequest, web};
use axum::{Json, routing::post, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};
use tracing::debug;

pub fn routes() -> Router {
    Router::new().route("/api/hello", post(hello))
}

async fn hello(req: Json<LoginRequest>) -> Result<Json<Value>> {
    debug!("{:<12} - login", "HANDLER");

    let body = Json(json!({ "success": true }));
    
    Ok(body)
}