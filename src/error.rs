//! Error types for the API client.
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt;

/// A type alias for `Result<T, Error>`.
pub type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

/// An error that can occur while interacting with the API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occurred while building the client.
    #[error("An error occurred while building the client: {0}")]
    Builder(BoxError),
    /// An error occurred while sending the request.
    #[error("An error occurred while sending the request: {0}")]
    Request(BoxError),
    /// An error occurred while deserializing the response.
    #[error("An error occurred while deserializing the response: {0}")]
    Deserialize(BoxError),
    /// An error returned from the API.
    #[error("An error returned from server: {0}")]
    Api(#[from] ApiError),
}

/// An error returned from the API.
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// The HTTP status code of the response.
    pub status: i32,
    /// The error message returned from the API.
    pub error: String,
}

impl Error {
    pub(crate) fn builder<E>(error: E) -> Self
    where
        E: Into<BoxError>,
    {
        let error = Self::Builder(error.into());
        error!("{error}");
        error
    }

    pub(crate) fn request<E>(error: E) -> Self
    where
        E: Into<BoxError>,
    {
        let error = Self::Request(error.into());
        error!("{error}");
        error
    }

    pub(crate) fn deserialize<E>(error: E) -> Self
    where
        E: Into<BoxError>,
    {
        let error = Self::Deserialize(error.into());
        error!("{error}");
        error
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]{}", self.status, self.error)
    }
}

impl StdError for ApiError {}
