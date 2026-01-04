use reqwest::header::{HeaderName, HeaderValue};

pub const DEFAULT_BASE_URL: &str = "https://api.lob.com/v1/";
pub const DEFAULT_POSTCARDS_URL: &str = "https://api.lob.com/v1/postcards";

pub const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
pub const IDEMPOTENCY_KEY: HeaderName = HeaderName::from_static("idempotency-key");
