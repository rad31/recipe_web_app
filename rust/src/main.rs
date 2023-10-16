#![allow(unused)]
mod config;
mod ctx;
mod error;
mod models;
mod services;
mod web;

pub use self::error::{Error, Result};
pub use config::config;

use crate::models::db::Measure;
use crate::services::sqlx_service;
use std::net::SocketAddr;
use axum::extract::{Query, Path};
use axum::{Router, Server, middleware};
use axum::routing::{get, get_service};
use axum::response::{Html, IntoResponse, Response};
use serde::Deserialize;
use sqlx::{Connection, Row};
use tower_http::services::ServeDir;
use tracing::{info, debug};
use tracing_subscriber::EnvFilter;
use web::routes_auth;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // let nutrients_connection_string = "postgres://ronny:password@localhost:5432/nutrients";
    // let pool = sqlx::postgres::PgPool::connect(nutrients_connection_string).await?;
    // sqlx::migrate!("./migrations").run(&pool).await?;

    // let data_present = sqlx_service::is_data_present(&pool).await?;
    // if (!data_present) {
    //     sqlx_service::insert_nutrient_data(nutrients_connection_string).await?;
    // }

    let routes_hello = web::routes_hello::routes();
        // .route_layer(middleware::from_fn(web::middleware::require_auth));

    let routes_all = Router::new()
        .merge(routes_auth::routes())
        .nest("/api", routes_hello)
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("{:<12} - {addr}\n", "LISTENING");
    Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    debug!("{:<12} main_response_mapper\n", "RES_MAPPER");
    res
}

fn routes_static() -> Router { 
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
