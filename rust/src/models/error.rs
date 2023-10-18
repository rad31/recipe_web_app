pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DbSetupFailed,
}

impl From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        Error::DbSetupFailed
    }
}

impl From<crate::models::store::Error> for Error {
    fn from(_: crate::models::store::Error) -> Self {
        Error::DbSetupFailed
    }
}
