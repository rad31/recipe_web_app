use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use tracing::debug;

use crate::models;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error  {
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    // Model(models::error::Error),
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter)
        -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}
