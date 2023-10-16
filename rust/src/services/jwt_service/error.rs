use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    AccessTokenInvalid,
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter)
        -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl From<jwt::Error> for Error {
    fn from(_: jwt::Error) -> Self {
        Error::AccessTokenInvalid
    }
}

impl From<std::fmt::Error> for Error {
    fn from(_: std::fmt::Error) -> Self {
        Error::AccessTokenInvalid
    }
}

impl From<uuid::Error> for Error {
    fn from(_: uuid::Error) -> Self {
        Error::AccessTokenInvalid
    }
}

impl From<hmac::digest::InvalidLength> for Error {
    fn from(_: hmac::digest::InvalidLength) -> Self {
        Error::AccessTokenInvalid
    }
}
