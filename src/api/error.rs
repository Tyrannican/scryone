use thiserror::Error;

use crate::api::request::DataFormat;

#[derive(Error, Debug)]
pub enum ScryfallApiError {
    #[error("invalid url provided")]
    UrlParse(#[from] url::ParseError),

    #[error("expected mandatory field: {0}")]
    ExpectedField(String),

    #[error("expected one of the following fields: {0:?}")]
    ExpectedFieldsOneOf(Vec<String>),

    #[error("invalid data format supplied: {0}")]
    InvalidDataFormat(DataFormat),

    #[error("invalid data supplied: {0}")]
    InvalidData(String),

    #[error("unexpected error occurred calling scryfall api")]
    ApiCall(#[from] reqwest::Error),

    #[error("invalid response")]
    InvalidResponse,

    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),
}
