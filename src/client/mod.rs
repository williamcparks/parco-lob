/// A reusable client for interacting with the Lob API.
///
/// Created via the [`LobClient::builder`] Method
#[derive(Clone, Debug)]
pub struct LobClient<'a, 'b> {
    client: reqwest::Client,
    api_key: &'a str,
    base_url: &'b str,
}

mod build_url;
mod builder;
mod create_postcard;
