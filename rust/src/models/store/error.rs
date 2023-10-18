use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	FailedToCreatePool(String),
	MigrationFailed
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
        Error::FailedToCreatePool(e.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Error::FailedToCreatePool(e.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(_: sqlx::migrate::MigrateError) -> Self {
        Error::MigrationFailed
    }
}