use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error occurred while building the client: {0}")]
    Builder(BoxError),
    #[error("An error occurred while sending the request: {0}")]
    Request(BoxError),
    #[error("An error occurred while deserializing the response: {0}")]
    Deserialize(BoxError),

    #[error("An error returned from server: {0}")]
    Api(#[from] ApiError),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status: i32,
    pub error: String,
}

impl Error {
    pub(crate) fn builder<E>(error: E) -> Self
    where
        E: Into<BoxError>,
    {
        Self::Builder(error.into())
    }

    pub(crate) fn request<E>(error: E) -> Self
    where
        E: Into<BoxError>,
    {
        Self::Request(error.into())
    }

    pub(crate) fn deserialize<E>(error: E) -> Self
    where
        E: Into<BoxError>,
    {
        Self::Deserialize(error.into())
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]{}", self.status, self.error)
    }
}

impl StdError for ApiError {}
