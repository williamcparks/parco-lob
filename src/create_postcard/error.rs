use thiserror::Error;

use crate::ApiError;

/// Errors that can occur while performing a Lob "create postcard" request.
#[derive(Debug, Error)]
pub enum CreatePostcardError {
    /// HTTP or network error from [`reqwest`].
    #[error("Reqwest: {0}")]
    Reqwest(
        #[from]
        #[source]
        reqwest::Error,
    ),

    /// Failed to serialize the json request
    #[error("Json: {0}")]
    Serialize(
        #[from]
        #[source]
        serde_json::Error,
    ),

    /// An Api Error From Lob
    #[error("Lob: {0}")]
    Api(
        #[from]
        #[source]
        ApiError,
    ),

    /// Failed to parse the JSON response.
    ///
    /// Contains the parse error and the raw response body.
    #[error("Json: {0} - {1}")]
    Json(#[source] serde_json::Error, String),
}
