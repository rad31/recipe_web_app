use sqlx::error::DatabaseError;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    PasswordInvalid,
    UserAlreadyExists,
    UserConversionFailed,
    UserCreationFailed(sqlx::Error),
    UserNotFound,
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}


impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        debug!("{}", e);
        let dbe =  match e {
            sqlx::Error::Database(ref dbe) => dbe,
            sqlx::Error::RowNotFound => return Error::UserNotFound,
            _ => return Error::UserCreationFailed(e),
        };
        match dbe.is_unique_violation() {
            true => Error::UserAlreadyExists,
            false => Error::UserCreationFailed(e),
        }
    }
}

impl From<crate::services::crypto_service::error::Error> for Error {
    fn from(_: crate::services::crypto_service::error::Error) -> Self {
        Error::UserConversionFailed
    }
}
