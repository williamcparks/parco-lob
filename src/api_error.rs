use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
pub(crate) struct WrapperApiError {
    pub(crate) error: ApiError,
}

/// An Api Error Message From Lob
#[derive(Debug, Deserialize, Error)]
#[error("Lob: {message} : {code} : {status_code}")]
pub struct ApiError {
    pub message: Box<str>,
    pub code: Box<str>,
    pub status_code: u16,
}
