#![doc = include_str!("../README.md")]

mod api_error;
mod client;
pub(crate) mod constants;
mod create_postcard;

pub use api_error::ApiError;
pub(crate) use api_error::WrapperApiError;
pub use client::LobClient;
pub use create_postcard::{CreatePostcardError, CreatePostcardResponse, MailType, Size, UseType};

#[cfg(test)]
mod test;
