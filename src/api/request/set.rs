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
#[derive(Default)]
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
            && !matches!(fmt, DataFormat::Json) {
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
#[derive(Default)]
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
            && !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }

        Ok(SetFromIdRequest {
            id: self.id,
            format: self.format,
            pretty: self.pretty,
        })
    }
}
