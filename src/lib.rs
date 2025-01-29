//! # Rust API for the [fflogs](https://www.fflogs.com) API
#[macro_use]
extern crate tracing;

mod error;
pub mod v1;

pub use error::{ApiError, Error, Result};

#[cfg(test)]
#[ctor::ctor]
fn init() {
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}
