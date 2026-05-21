//! Wrapped error types that can occur with the Scryone Library
use reqwest::StatusCode;
use thiserror::Error;

use crate::api::request::DataFormat;

/// Errors that can occur when communicating with the Scryfall API
#[derive(Error, Debug)]
pub enum ScryfallApiError {
    /// Error parsing the provided URL
    #[error("invalid url provided")]
    UrlParse(#[from] url::ParseError),

    /// Expected a mandatory field to be present
    #[error("expected mandatory field: {0}")]
    ExpectedField(String),

    /// Expected one of the given fields to be present
    #[error("expected one of the following fields: {0:?}")]
    ExpectedFieldsOneOf(Vec<String>),

    /// Endpoint doesn't support his format
    #[error("invalid data format supplied: {0}")]
    InvalidDataFormat(DataFormat),

    /// Data provided is invalid
    #[error("invalid data supplied: {0}")]
    InvalidData(String),

    /// Error occurred communicating with the Scryfall API
    #[error("unexpected error occurred calling scryfall api")]
    ApiCall(#[from] reqwest::Error),

    /// Response does not match what was expected
    #[error("invalid response")]
    InvalidResponse(StatusCode),

    /// HTTP 404 Error
    #[error("not found: {0}")]
    NotFound(String),

    /// HTTP 400 Error
    #[error("bad request: {0}")]
    BadRequest(String),

    /// HTTP 429 Error
    #[error("too many requests")]
    TooManyRequests,
}
