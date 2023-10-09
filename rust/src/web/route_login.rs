use crate::{error::Error, error::Result, web};
use axum::{Json, routing::post, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(login))
}

async fn login(cookies: Cookies, req: Json<LoginRequest>) -> Result<Json<Value>> {
    println!("-> {:<12} - login", "HANDLER");

    // TODO: Implement real db auth logic
    if  (req.username != "ronny" || req.password != "password") {
        return Err(Error::LoginFailed);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user.exp.sign"));

    let body = Json(json!({ "success": true }));
    
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginRequest  {
    username: String,
    password: String,
}