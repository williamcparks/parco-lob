use std::borrow::Cow;

use reqwest::header::CONTENT_TYPE;
use serde::Serialize;

use crate::{
    CreatePostcardError, CreatePostcardResponse, WrapperApiError,
    constants::{APPLICATION_JSON, IDEMPOTENCY_KEY},
};

pub struct CreatePostcardRequest<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, Merge> {
    pub(crate) client: reqwest::Client,
    pub(crate) api_key: &'a str,
    pub(crate) url: Cow<'b, str>,
    pub(crate) idempotency_key: &'c str,
    pub(crate) json_request: JsonRequest<'d, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, Merge>,
}

pub struct CreatePostcardRequestNoMerge<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l> {
    pub(crate) client: reqwest::Client,
    pub(crate) api_key: &'a str,
    pub(crate) url: Cow<'b, str>,
    pub(crate) idempotency_key: &'c str,
    pub(crate) json_request: JsonRequestNoMerge<'d, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l>,
}

#[derive(Serialize)]
pub struct JsonRequest<'d, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, Merge> {
    pub(crate) to: To<'d, 'e, 'f, 'g, 'h, 'i>,
    pub(crate) front: &'j str,
    pub(crate) back: &'k str,
    pub(crate) size: Size,
    pub(crate) mail_type: MailType,
    pub(crate) merge_variables: Merge,
    pub(crate) description: &'l str,
    pub(crate) use_type: UseType,
}

#[derive(Serialize)]
pub struct JsonRequestNoMerge<'d, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l> {
    pub(crate) to: To<'d, 'e, 'f, 'g, 'h, 'i>,
    pub(crate) front: &'j str,
    pub(crate) back: &'k str,
    pub(crate) size: Size,
    pub(crate) mail_type: MailType,
    pub(crate) description: &'l str,
    pub(crate) use_type: UseType,
}

#[derive(Serialize)]
pub struct To<'d, 'e, 'f, 'g, 'h, 'i> {
    pub(crate) name: &'d str,
    #[serde(rename = "address_line1")]
    pub(crate) address_line_1: &'e str,
    #[serde(rename = "address_line2")]
    pub(crate) address_line_2: Option<&'f str>,
    pub(crate) address_city: &'g str,
    pub(crate) address_state: &'h str,
    pub(crate) address_zip: &'i str,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
/// Identifying your mail use type helps Lob populate the right mail settings and postage options to ensure your mail is produced and delivered in an optimal way. Lob requires that you identify—or tag—your mail with one of the following use type options:
pub enum UseType {
    /// Marketing mail: Any mailers that are sent for marketing, advertising, and promotional purposes
    Marketing,

    /// Operational mail: All other mail, typically transactional or functional in nature, such as invoices, adverse action notices, statements, and other confidential mail that include sensitive PII/PHI data
    Operational,
}

#[derive(Serialize)]
/// Specifies the size of the postcard. Only 4x6 postcards can be sent to international destinations.
pub enum Size {
    #[serde(rename = "4x6")]
    FourBySix,
    #[serde(rename = "6x9")]
    SixByNine,
    #[serde(rename = "6x11")]
    SixByEleven,
}

/// An enum designating the mail postage type
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MailType {
    /// usps_first_class - (default)
    UspsFirstClass,
    /// usps_standard - a cheaper option which is less predictable and takes longer to deliver. usps_standard cannot be used with 4x6 postcards or for any postcards sent outside of the United States.
    UspsStandard,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, Merge: Serialize>
    CreatePostcardRequest<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, Merge>
{
    pub async fn send(self) -> Result<CreatePostcardResponse, CreatePostcardError> {
        let request = serde_json::to_string(&self.json_request)?;

        let src = self
            .client
            .post(self.url.as_ref())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(IDEMPOTENCY_KEY, self.idempotency_key)
            .basic_auth::<_, &str>(self.api_key, None)
            .body(request)
            .send()
            .await?
            .text()
            .await?;

        if let Ok(wrapper_api_error) = serde_json::from_str::<WrapperApiError>(src.as_str()) {
            return Err(CreatePostcardError::Api(wrapper_api_error.error));
        }

        match serde_json::from_str(src.as_str()) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(CreatePostcardError::Json(err, src)),
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l>
    CreatePostcardRequestNoMerge<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l>
{
    pub async fn send(self) -> Result<CreatePostcardResponse, CreatePostcardError> {
        let request = serde_json::to_string(&self.json_request)?;

        let src = self
            .client
            .post(self.url.as_ref())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(IDEMPOTENCY_KEY, self.idempotency_key)
            .basic_auth::<&str, &str>(self.api_key, None)
            .body(request)
            .send()
            .await?
            .text()
            .await?;

        if let Ok(wrapper_api_error) = serde_json::from_str::<WrapperApiError>(src.as_str()) {
            return Err(CreatePostcardError::Api(wrapper_api_error.error));
        }

        match serde_json::from_str(src.as_str()) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(CreatePostcardError::Json(err, src)),
        }
    }
}
