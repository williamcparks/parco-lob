use serde::Deserialize;

/// The response from lob's api for creating a postcard
#[derive(Clone, Debug, Deserialize)]
pub struct CreatePostcardResponse {
    /// the id of the postcard
    pub id: Box<str>,
    /// the send date
    pub send_date: Box<str>,
}
