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
use sqlx::{Connection, Row};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use web::route_login;
use std::error::Error;

use crate::models::measure::Measure;
use crate::services::sqlx_service;
// use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let nutrients_connection_string = "postgres://ronny:password@localhost:5432/nutrients";

    let pool = sqlx::postgres::PgPool::connect(nutrients_connection_string).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let data_present = sqlx_service::is_data_present(&pool).await?;
    if (!data_present) {
        sqlx_service::insert_nutrient_data(nutrients_connection_string).await?;
    }    

    let row = sqlx::query_as::<_, Measure>("SELECT * FROM measure")
        .fetch_one(&pool)
        .await?;

    println!("{:?}", row);
    
    // let routes_all = Router::new()
    //     .merge(route_login::routes())
    //     .layer(middleware::map_response(main_response_mapper))
    //     .layer(CookieManagerLayer::new())
    //     .fallback_service(routes_static());

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // println!("->  LISTENING ON {addr}\n");
    // Server::bind(&addr)
    //     .serve(routes_all.into_make_service())
    //     .await
    //     .unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("-> {:<12} main_response_mapper", "RES_MAPPER");
    println!();
    res
}



fn routes_static() -> Router { 
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}