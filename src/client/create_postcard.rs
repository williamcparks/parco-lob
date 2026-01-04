use std::borrow::Cow;

use crate::{
    LobClient,
    constants::{DEFAULT_BASE_URL, DEFAULT_POSTCARDS_URL},
    create_postcard::builder::CreatePostcardBuilder,
};

impl<'a, 'b> LobClient<'a, 'b> {
    /// create a postcard request builder
    pub fn create_postcard(&self) -> CreatePostcardBuilder<'a, 'b> {
        use super::build_url::build_url;

        let url = match self.base_url == DEFAULT_BASE_URL {
            true => Cow::Borrowed(DEFAULT_POSTCARDS_URL),
            false => Cow::Owned(build_url(self.base_url, "postcards")),
        };

        CreatePostcardBuilder::new(self.client.clone(), self.api_key, url)
    }
}
