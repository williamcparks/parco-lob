use crate::{LobClient, constants::DEFAULT_BASE_URL};

/// Builder for constructing a [`LobClient`].
///
/// Enforces providing a [`reqwest::Client`] and API key before use.
pub struct LobClientBuilder;

impl LobClientBuilder {
    /// Sets the underlying [`reqwest::Client`].
    pub fn client(self, client: reqwest::Client) -> LobClientBuilderWithClient {
        LobClientBuilderWithClient { client }
    }
}

/// Intermediate builder state after a [`reqwest::Client`] is provided.
pub struct LobClientBuilderWithClient {
    client: reqwest::Client,
}

impl LobClientBuilderWithClient {
    /// Sets the Lob API key.
    pub fn api_key<'a>(self, api_key: &'a str) -> LobClientBuilderWithApiKey<'a> {
        LobClientBuilderWithApiKey {
            client: self.client,
            api_key,
        }
    }
}

/// Final builder state with an API key set.
pub struct LobClientBuilderWithApiKey<'a> {
    client: reqwest::Client,
    api_key: &'a str,
}

impl<'a> LobClientBuilderWithApiKey<'a> {
    /// Builds a client using the default Lob base URL.
    pub fn build(self) -> LobClient<'a, 'static> {
        LobClient {
            client: self.client,
            api_key: self.api_key,
            base_url: DEFAULT_BASE_URL,
        }
    }

    /// Builds a client using a custom base URL.
    pub fn build_with_base_url<'b>(self, base_url: &'b str) -> LobClient<'a, 'b> {
        LobClient {
            client: self.client,
            api_key: self.api_key,
            base_url,
        }
    }
}

impl<'a, 'b> LobClient<'a, 'b> {
    /// Create a client builder
    pub fn builder() -> LobClientBuilder {
        LobClientBuilder
    }
}
