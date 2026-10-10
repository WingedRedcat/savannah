use thiserror::Error;

/// Errors returned by Savannah engine operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid engine configuration: {0}")]
    InvalidConfiguration(&'static str),
    #[error("unsupported URL scheme: {0}")]
    UnsupportedScheme(String),
    #[error("invalid URL: {0}")]
    InvalidUrl(String),
    #[error("navigation was cancelled")]
    Cancelled,
    #[error("response exceeded the configured size limit of {0} bytes")]
    ResponseTooLarge(usize),
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("navigation timed out after {0:?}")]
    Timeout(std::time::Duration),
}

pub type Result<T> = std::result::Result<T, Error>;
