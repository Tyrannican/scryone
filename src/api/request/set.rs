//! Request types and builders for Sets from Scryfall API
//!
//! More detailed information here: <https://scryfall.com/docs/api/sets>
use crate::objects::{list::List, set::Set};

use super::{BASE_URL, DataFormat, ScryfallApiError, ScryfallRequest, add_query_pair};
use url::Url;
use uuid::Uuid;

/// Reqeust type for the `/sets` Scryfall endpoint
#[derive(Debug, Clone)]
pub struct SetsRequest {
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SetsRequest {
    /// Construct a builder for a `SetsRequest`
    pub fn builder() -> SetsRequestBuilder {
        SetsRequestBuilder::default()
    }
}

impl ScryfallRequest for SetsRequest {
    type Response = List<Set>;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?.join("/sets")?;
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

/// Builder for constructing a [`SetsRequest`]
#[derive(Debug, Default)]
pub struct SetsRequestBuilder {
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SetsRequestBuilder {
    /// Construct a new `SetsRequestBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the [`DataFormat`] for the response
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    /// Sets the flag for prettifying the JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Builds the [`SetsRequest`]
    pub fn build(self) -> Result<SetsRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
        }

        Ok(SetsRequest {
            format: self.format,
            pretty: self.pretty,
        })
    }
}

/// The ID of the Set to request
#[derive(Debug, Clone, PartialEq)]
pub enum SetId {
    /// Use the `/sets/:code` endpoint (3-6 letter set code)
    Code(String),

    /// Use the `/sets/tcgplayer/:id` endpoint
    TcgPlayer(u32),

    /// Use the `/sets/:id` endpoint
    Id(Uuid),
}

impl SetId {
    pub(crate) fn subpaths(&self) -> (String, String) {
        match self {
            Self::Code(code) => (String::new(), code.clone()),
            Self::TcgPlayer(id) => ("tcgplayer".to_string(), id.to_string()),
            Self::Id(id) => (String::new(), id.to_string()),
        }
    }
}

impl Default for SetId {
    fn default() -> Self {
        Self::Code("mh2".to_string())
    }
}

/// Request type for calling the `/sets/(tcgplayer)/(:id/:code)` Scryfall endpoints
#[derive(Debug, Clone)]
pub struct SetFromIdRequest {
    id: SetId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SetFromIdRequest {
    /// Construct a builder for a `SetFromIdRequest`
    pub fn builder() -> SetFromIdRequestBuilder {
        SetFromIdRequestBuilder::default()
    }
}

impl ScryfallRequest for SetFromIdRequest {
    type Response = Set;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let (subpath, id) = self.id.subpaths();

        let path = if subpath.is_empty() {
            format!("/sets/{id}")
        } else {
            format!("/sets/{subpath}/{id}")
        };

        url = url.join(&path)?;

        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

/// Builder for constructing [`SetFromIdRequest`]
#[derive(Debug, Default)]
pub struct SetFromIdRequestBuilder {
    id: SetId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SetFromIdRequestBuilder {
    /// Construct a new `SetFromIdRequestBuilder`
    pub fn new(id: SetId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Set the [`SetId`] for the request
    ///
    /// This is used to construct the appropriate URI
    pub fn id(mut self, id: SetId) -> Self {
        self.id = id;
        self
    }

    /// Set the [`DataFormat`] for the response
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    pub fn format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    /// Sets the flag for prettifying the JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Builds the [`SetFromIdRequest`]
    pub fn build(self) -> Result<SetFromIdRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
        }

        Ok(SetFromIdRequest {
            id: self.id,
            format: self.format,
            pretty: self.pretty,
        })
    }
}

#[cfg(test)]
mod url_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn sets_request_default() {
        let req = SetsRequest::builder().build().expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/sets");
    }

    #[test]
    fn sets_request_pretty_only() {
        let req = SetsRequest::builder()
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/sets?pretty=true");
    }

    #[test]
    fn sets_request_format_only() {
        let req = SetsRequest::builder()
            .data_format(DataFormat::Json)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/sets?format=json");
    }

    #[test]
    fn sets_request_format_and_pretty() {
        let req = SetsRequest::builder()
            .data_format(DataFormat::Json)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/sets?format=json&pretty=true"
        );
    }

    #[test]
    fn sets_request_rejects_csv() {
        let err = SetsRequest::builder()
            .data_format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /sets");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Csv)));
    }

    #[test]
    fn sets_request_rejects_text() {
        let err = SetsRequest::builder()
            .data_format(DataFormat::Text)
            .build()
            .expect_err("Text is not a valid format for /sets");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Text)));
    }

    #[test]
    fn sets_request_rejects_image() {
        let err = SetsRequest::builder()
            .data_format(DataFormat::Image)
            .build()
            .expect_err("Image is not a valid format for /sets");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Image)));
    }

    #[test]
    fn sets_request_rejects_file() {
        let err = SetsRequest::builder()
            .data_format(DataFormat::File)
            .build()
            .expect_err("File is not a valid format for /sets");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::File)));
    }

    #[test]
    fn set_default_id_is_mh2_code() {
        let req = SetFromIdRequest::builder()
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/sets/mh2");
    }

    #[test]
    fn set_from_id_code() {
        let req = SetFromIdRequest::builder()
            .id(SetId::Code("mh2".to_string()))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/sets/mh2");
    }

    #[test]
    fn set_from_id_code_neo() {
        let req = SetFromIdRequest::builder()
            .id(SetId::Code("neo".to_string()))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/sets/neo");
    }

    #[test]
    fn set_from_id_code_khm() {
        let req = SetFromIdRequest::builder()
            .id(SetId::Code("khm".to_string()))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/sets/khm");
    }

    #[test]
    fn set_from_id_tcgplayer() {
        let req = SetFromIdRequest::builder()
            .id(SetId::TcgPlayer(1909))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/sets/tcgplayer/1909");
    }

    #[test]
    fn set_from_id_uuid() {
        let id = Uuid::from_str("2ec77b94-6d47-4891-a480-5d0b4e5c9372")
            .expect("valid Scryfall set uuid");
        let req = SetFromIdRequest::builder()
            .id(SetId::Id(id))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/sets/2ec77b94-6d47-4891-a480-5d0b4e5c9372"
        );
    }

    #[test]
    fn set_from_id_with_format_and_pretty() {
        let req = SetFromIdRequest::builder()
            .id(SetId::Code("mh2".to_string()))
            .format(DataFormat::Json)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/sets/mh2?format=json&pretty=true"
        );
    }

    #[test]
    fn set_from_id_rejects_csv() {
        let err = SetFromIdRequest::builder()
            .id(SetId::Code("mh2".to_string()))
            .format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /sets/:id");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Csv)));
    }

    #[test]
    fn set_from_id_rejects_text() {
        let err = SetFromIdRequest::builder()
            .id(SetId::Code("mh2".to_string()))
            .format(DataFormat::Text)
            .build()
            .expect_err("Text is not a valid format for /sets/:id");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Text)));
    }

    #[test]
    fn set_from_id_rejects_image() {
        let err = SetFromIdRequest::builder()
            .id(SetId::Code("mh2".to_string()))
            .format(DataFormat::Image)
            .build()
            .expect_err("Image is not a valid format for /sets/:id");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Image)));
    }

    #[test]
    fn set_from_id_rejects_file() {
        let err = SetFromIdRequest::builder()
            .id(SetId::Code("mh2".to_string()))
            .format(DataFormat::File)
            .build()
            .expect_err("File is not a valid format for /sets/:id");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::File)));
    }
}
