// use std::ops::FromResidual;

use axum::{response::{IntoResponse, Response}, http::{StatusCode, header::ToStrError}};
use thiserror::Error;
use tracing::debug;

use crate::services::{jwt_service, auth_service};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    AccessTokenExpired,
    AccessTokenInvalid,
    AccessTokenNotFound,
    UserAlreadyExists,
    UserCreationFailed,
    LoginFailed,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - {self:?}", "INTO_RES");
        let res = match self {
            Error::AccessTokenExpired => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            Error::AccessTokenInvalid => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            Error::AccessTokenNotFound => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            Error::LoginFailed => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            Error::UserCreationFailed => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
            Error::UserAlreadyExists => (StatusCode::CONFLICT, "CONFLICT"),
        };
        res.into_response()
    }
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter)
        -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl From<jwt_service::error::Error> for Error {
    fn from(_: jwt_service::error::Error) -> Self {
        Error::AccessTokenInvalid
    }
}

impl From<ToStrError> for Error {
    fn from(_: ToStrError) -> Self {
        Error::AccessTokenInvalid
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        Error::AccessTokenNotFound
    }
}

impl From<auth_service::error::Error> for Error {
    fn from(e: auth_service::error::Error) -> Self {
        match e {
            auth_service::error::Error::PasswordInvalid => Error::LoginFailed,
            auth_service::error::Error::UserAlreadyExists => Error::UserAlreadyExists,
            auth_service::error::Error::UserCreationFailed(_) => Error::UserCreationFailed,
            auth_service::error::Error::UserConversionFailed => Error::UserCreationFailed,
            auth_service::error::Error::UserNotFound => Error::LoginFailed,
        }
    }
}