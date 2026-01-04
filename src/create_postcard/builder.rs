use std::borrow::Cow;

use serde::Serialize;

use crate::{MailType, Size, UseType};

use super::request::{
    CreatePostcardRequest, CreatePostcardRequestNoMerge, JsonRequest, JsonRequestNoMerge, To,
};

/// Builder for creating a create postcard request.
///
/// Returned from [`crate::LobClient`](LobClient).
pub struct CreatePostcardBuilder<'a, 'b> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
}

impl<'a, 'b> CreatePostcardBuilder<'a, 'b> {
    pub(crate) fn new(client: reqwest::Client, api_key: &'a str, url: Cow<'b, str>) -> Self {
        Self {
            client,
            api_key,
            url,
        }
    }

    /// sets the idempotency key for the request. Idempotent requests are requests that can be called many times without producing different outcomes.
    ///
    /// The Idempotency Key is a feature that allows you to pass a unique key along with each request and guarantee that only one mailer is created and sent to prevent any duplicate mailings from being created. You can safely retry the same request with the same Idempotency Key and be assured that no duplicates are created even if the API is called multiple times within 24 hours.
    pub fn idempotency_key<'c>(
        self,
        idempotency_key: &'c str,
    ) -> CreatePostcardBuilderWithIdempotencyKey<'a, 'b, 'c> {
        CreatePostcardBuilderWithIdempotencyKey {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key,
        }
    }
}

/// Builder for a create postcard request with an idempotency key set.
pub struct CreatePostcardBuilderWithIdempotencyKey<'a, 'b, 'c> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
}

impl<'a, 'b, 'c> CreatePostcardBuilderWithIdempotencyKey<'a, 'b, 'c> {
    /// sets the name for the address to which the postcard is being sent
    pub fn name<'d>(self, name: &'d str) -> CreatePostcardBuilderWithName<'a, 'b, 'c, 'd> {
        CreatePostcardBuilderWithName {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name,
        }
    }
}

/// Builder for a create postcard request with a name set.
pub struct CreatePostcardBuilderWithName<'a, 'b, 'c, 'd> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
}

impl<'a, 'b, 'c, 'd> CreatePostcardBuilderWithName<'a, 'b, 'c, 'd> {
    /// sets the first address line for the address to which the postcard is being sent
    pub fn address_line_1<'e, 'f>(
        self,
        address_line_1: &'e str,
    ) -> CreatePostcardBuilderWithAddress<'a, 'b, 'c, 'd, 'e, 'f> {
        CreatePostcardBuilderWithAddress {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1,
            address_line_2: None,
        }
    }
}

/// Builder for a create postcard request with addresses set.
pub struct CreatePostcardBuilderWithAddress<'a, 'b, 'c, 'd, 'e, 'f> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f> CreatePostcardBuilderWithAddress<'a, 'b, 'c, 'd, 'e, 'f> {
    /// sets the second address line for the address to which the postcard is being sent
    pub fn address_line_2(self, address_line_2: &'f str) -> Self {
        Self {
            address_line_2: Some(address_line_2),
            ..self
        }
    }

    /// sets the city for the address to which the postcard is being sent
    pub fn city<'g>(
        self,
        city: &'g str,
    ) -> CreatePostcardBuilderWithCity<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
        CreatePostcardBuilderWithCity {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city,
        }
    }
}

/// Builder for a create postcard request with a city set.
pub struct CreatePostcardBuilderWithCity<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> CreatePostcardBuilderWithCity<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    /// sets the state for the address to which the postcard is being sent
    pub fn state<'h>(
        self,
        state: &'h str,
    ) -> CreatePostcardBuilderWithState<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
        CreatePostcardBuilderWithState {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city: self.city,
            state,
        }
    }
}

/// Builder for a create postcard request with a state set.
pub struct CreatePostcardBuilderWithState<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
    state: &'h str,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
    CreatePostcardBuilderWithState<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
{
    /// sets the zip code for the address to which the postcard is being sent
    pub fn zip_code<'i>(
        self,
        zip_code: &'i str,
    ) -> CreatePostcardBuilderWithZipCode<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
        CreatePostcardBuilderWithZipCode {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city: self.city,
            state: self.state,
            zip_code,
        }
    }
}

/// Builder for a create postcard request with a zip code set.
pub struct CreatePostcardBuilderWithZipCode<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
    state: &'h str,
    zip_code: &'i str,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i>
    CreatePostcardBuilderWithZipCode<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i>
{
    /// sets the front template of the postcard being sent.
    ///
    /// The artwork to use as the front of your postcard.
    ///
    /// ## Notes:
    ///
    /// HTML merge variables should not include delimiting whitespace.
    /// PDF, PNG, and JPGs must be sized at 4.25"x6.25", 6.25"x9.25", or 6.25"x11.25" at 300 DPI, while supplied HTML will be rendered to the specified size.
    pub fn front<'j>(
        self,
        front: &'j str,
    ) -> CreatePostcardBuilderWithFront<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
        CreatePostcardBuilderWithFront {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            front,
        }
    }
}

/// Builder for a create postcard request with front template set.
pub struct CreatePostcardBuilderWithFront<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
    state: &'h str,
    zip_code: &'i str,
    front: &'j str,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
    CreatePostcardBuilderWithFront<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
{
    /// sets the back template of the postcard being sent.
    ///
    /// The artwork to use as the back of your postcard.
    ///
    /// ## Notes:
    ///
    /// HTML merge variables should not include delimiting whitespace.
    /// PDF, PNG, and JPGs must be sized at 4.25"x6.25", 6.25"x9.25", or 6.25"x11.25" at 300 DPI, while supplied HTML will be rendered to the specified size.
    pub fn back<'k>(
        self,
        back: &'k str,
    ) -> CreatePostcardBuilderWithBack<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
        CreatePostcardBuilderWithBack {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            front: self.front,
            back,
        }
    }
}

/// Builder for a create postcard request with back template set.
pub struct CreatePostcardBuilderWithBack<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
    state: &'h str,
    zip_code: &'i str,
    front: &'j str,
    back: &'k str,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
    CreatePostcardBuilderWithBack<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
{
    /// sets the "usage type" for the postcard being sent.
    ///
    /// The use type for each mailpiece. Can be one of marketing, operational. For more information on use_type, see lob's Help Center article.
    pub fn use_type(
        self,
        use_type: UseType,
    ) -> CreatePostcardBuilderWithUseType<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
        CreatePostcardBuilderWithUseType {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            front: self.front,
            back: self.back,
            use_type,
        }
    }
}

/// Builder for a create postcard request with a use type set.
pub struct CreatePostcardBuilderWithUseType<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
    state: &'h str,
    zip_code: &'i str,
    front: &'j str,
    back: &'k str,
    use_type: UseType,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
    CreatePostcardBuilderWithUseType<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
{
    /// sets the "size" for the postcard being sent.
    pub fn size(
        self,
        size: Size,
    ) -> CreatePostcardBuilderWithSize<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
        CreatePostcardBuilderWithSize {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            front: self.front,
            back: self.back,
            use_type: self.use_type,
            size,
        }
    }
}

/// Builder for a create postcard request with a postcard size set.
pub struct CreatePostcardBuilderWithSize<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
    state: &'h str,
    zip_code: &'i str,
    front: &'j str,
    back: &'k str,
    use_type: UseType,
    size: Size,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
    CreatePostcardBuilderWithSize<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
{
    /// sets the "mail type" for the postcard being sent.
    pub fn mail_type(
        self,
        mail_type: MailType,
    ) -> CreatePostcardBuilderWithMailType<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
        CreatePostcardBuilderWithMailType {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            front: self.front,
            back: self.back,
            use_type: self.use_type,
            size: self.size,
            mail_type,
        }
    }
}

/// Builder for a create postcard request with a postcard size set.
pub struct CreatePostcardBuilderWithMailType<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
    state: &'h str,
    zip_code: &'i str,
    front: &'j str,
    back: &'k str,
    use_type: UseType,
    size: Size,
    mail_type: MailType,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
    CreatePostcardBuilderWithMailType<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
{
    /// sets the "mail type" for the postcard being sent.
    pub fn description<'l>(
        self,
        description: &'l str,
    ) -> CreatePostcardBuilderWithDescription<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l> {
        CreatePostcardBuilderWithDescription {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            name: self.name,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            front: self.front,
            back: self.back,
            use_type: self.use_type,
            size: self.size,
            mail_type: self.mail_type,
            description,
        }
    }
}

/// Builder for a create postcard request with a description set.
pub struct CreatePostcardBuilderWithDescription<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l> {
    client: reqwest::Client,
    api_key: &'a str,
    url: Cow<'b, str>,
    idempotency_key: &'c str,
    name: &'d str,
    address_line_1: &'e str,
    address_line_2: Option<&'f str>,
    city: &'g str,
    state: &'h str,
    zip_code: &'i str,
    front: &'j str,
    back: &'k str,
    use_type: UseType,
    size: Size,
    mail_type: MailType,
    description: &'l str,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l>
    CreatePostcardBuilderWithDescription<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l>
{
    pub fn build(
        self,
    ) -> CreatePostcardRequestNoMerge<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l> {
        CreatePostcardRequestNoMerge {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            json_request: JsonRequestNoMerge {
                to: To {
                    name: self.name,
                    address_line_1: self.address_line_1,
                    address_line_2: self.address_line_2,
                    address_city: self.city,
                    address_state: self.state,
                    address_zip: self.zip_code,
                },
                front: self.front,
                back: self.back,
                use_type: self.use_type,
                size: self.size,
                mail_type: self.mail_type,
                description: self.description,
            },
        }
    }

    pub fn merge<Merge: Serialize>(
        self,
        merge_variables: Merge,
    ) -> CreatePostcardRequest<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, Merge> {
        CreatePostcardRequest {
            client: self.client,
            api_key: self.api_key,
            url: self.url,
            idempotency_key: self.idempotency_key,
            json_request: JsonRequest {
                to: To {
                    name: self.name,
                    address_line_1: self.address_line_1,
                    address_line_2: self.address_line_2,
                    address_city: self.city,
                    address_state: self.state,
                    address_zip: self.zip_code,
                },
                front: self.front,
                back: self.back,
                use_type: self.use_type,
                size: self.size,
                mail_type: self.mail_type,
                description: self.description,
                merge_variables,
            },
        }
    }
}
