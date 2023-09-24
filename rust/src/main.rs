#![allow(unused)]
mod models;
mod services;
mod error;
mod web;

use std::net::SocketAddr;
use axum::extract::{Query, Path};
// use self::services::db_service;
use axum::{Router, Server, middleware};
use axum::routing::{get, get_service};
use axum::response::{Html, IntoResponse, Response};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use web::route_login;
use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    // let path = "../nutrient_db/nutrients.db";
    // let connection = rusqlite::Connection::open(path).unwrap();
    // let foods = db_service::get_foods_by_desc("onion", &connection);

    let routes_all = Router::new()
        .merge(route_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->  LISTENING ON {addr}\n");
    Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-> {:<12} main_response_mapper", "RES_MAPPER");
    println!();
    res
} 

fn routes_static() -> Router { 
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}