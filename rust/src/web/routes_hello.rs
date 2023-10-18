use crate::{models::requests::LoginRequest, web::error::{Error, Result}};
use axum::{Json, routing::get, Router, response::{Response, IntoResponse}};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};
use tracing::debug;

pub fn routes() -> Router {
    Router::new().route("/hello", get(hello))
}

async fn hello() -> Result<Json<Value>> {
    debug!("{:<12} - hello", "HANDLER");
    let res = Json(json!({ "success": true }));
    Ok(res)
}
