use crate::{Error, Result};
use crate::services::crypto_service::{generate_signing_key, Key};
use std::{env, sync::OnceLock, str::FromStr};


pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|e| {
            panic!("Error while loading config: {e:?}")
        })
    })
}

pub struct Config {
    pub jwt_lifespan_seconds: i64,
    pub password_hash_cost: u32,
    pub db_max_connections: u32,
    pub db_url: String,
    pub web_folder: String,
    pub signing_key: Key,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        let jwt_secret = get_string("JWT_SECRET")?;
        let signing_key = generate_signing_key(jwt_secret)
            .map_err(|_| Error::ConfigWrongFormat("Failed to generate signing key"))?;
        
        Ok(Config {
            jwt_lifespan_seconds: get_int::<i64>("JWT_LIFESPAN_SECONDS")?,
            password_hash_cost: get_int::<u32>("PASSWORD_HASH_COST")?,
            db_max_connections: get_int::<u32>("DB_MAX_CONNECTIONS")?,
            db_url: get_string("DB_URL")?,
            web_folder: get_string("WEB_FOLDER")?,
            signing_key,
        })
    }
}

fn get_string(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_int<T>(name: &'static str) -> Result<T> where T: FromStr {
    get_string(name)?
        .parse::<T>()
        .map_err(|_| Error::ConfigWrongFormat((name)))
}
