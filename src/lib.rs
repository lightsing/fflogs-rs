//! # Rust API for the [fflogs](https://www.fflogs.com) API
mod error;
pub mod v1;

pub use error::{ApiError, Error, Result};
