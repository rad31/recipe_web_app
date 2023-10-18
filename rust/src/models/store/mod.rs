mod error;

pub use self::error::{Error, Result};

use crate::config;
use crate::services::sqlx_service;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    let pool = PgPoolOptions::new()
        .max_connections(config().db_max_connections.clone())
        .connect(&config().db_url)
        .await
        .map_err(|e| Error::FailedToCreatePool((e.to_string())))?;

    
    // TODO: Define sqlx_service somewhere else
    sqlx::migrate!("./migrations").run(&pool).await?;
    let data_present = sqlx_service::is_data_present(&pool).await?;
    if (!data_present) {
        sqlx_service::insert_nutrient_data(&config().db_url).await?;
    }

    Ok(pool)
}
