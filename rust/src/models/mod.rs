// pub mod food_name;
pub mod db;
pub mod error;
pub mod requests;
pub mod store;

pub use self::error::{Error, Result};
use self::store::{Db, new_db_pool};

#[derive(Clone)]
pub struct ModelManager {
    db: Db
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db: Db = new_db_pool().await?;
        Ok(ModelManager { db })
    }

    // TODO restrict access to db
    pub fn db(&self) -> &Db {
        &self.db
    }
}