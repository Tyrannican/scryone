//! Request types and builders for Rulings from Scryfall API
//!
//! More detailed information here: <https://scryfall.com/docs/api/rulings>
use std::str::FromStr;

use url::Url;
use uuid::Uuid;

use super::add_query_pair;
use crate::{
    api::{
        error::ScryfallApiError,
        request::{BASE_URL, DataFormat, ScryfallRequest},
    },
    objects::{list::List, ruling::Ruling},
};

/// The ID of the Ruling to request
#[derive(Debug, Clone, PartialEq)]
pub enum RulingId {
    /// Use the `/cards/multiverse/:id/rulings` endpoint
    Multiverse(u32),
    /// Use the `/cards/mtgo/:id/rulings` endpoint
    Mtgo(u32),
    /// Use the `/cards/arena/:id/rulings` endpoint
    Arena(u32),
    /// Use the `/cards/:id/rulings` endpoint
    Card(Uuid),
    /// Use the `/cards/:code/:number/rulings` endpoint
    SetAndCollector((String, String)),
}

impl RulingId {
    pub(crate) fn subpaths(&self) -> (String, String) {
        match self {
            Self::Multiverse(id) => ("multiverse".to_string(), id.to_string()),
            Self::Mtgo(id) => ("mtgo".to_string(), id.to_string()),
            Self::Arena(id) => ("arena".to_string(), id.to_string()),
            Self::Card(id) => (String::new(), id.to_string()),
            Self::SetAndCollector((code, number)) => (code.clone(), number.clone()),
        }
    }
}

impl Default for RulingId {
    fn default() -> Self {
        Self::Card(
            Uuid::from_str("f2b9983e-20d4-4d12-9e2c-ec6d9a345787")
                .expect("a valid uuid for a card from scryfall"),
        )
    }
}

/// Request type for calling the `/cards/:ruling_id/:id/rulings` Scryfall endpoint
#[derive(Debug, Clone)]
pub struct RulingRequest {
    id: RulingId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl RulingRequest {
    /// Construct a builder for `RulingRequest`
    pub fn builder() -> RulingRequestBuilder {
        RulingRequestBuilder::default()
    }
}

impl ScryfallRequest for RulingRequest {
    type Response = List<Ruling>;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let (first_path, second_path) = self.id.subpaths();
        let path = if first_path.is_empty() {
            format!("/cards/{second_path}/rulings")
        } else {
            format!("/cards/{first_path}/{second_path}/rulings")
        };
        url = url.join(&path)?;

        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

/// Builder for constructing a [`RulingRequest`]
#[derive(Default)]
pub struct RulingRequestBuilder {
    id: RulingId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl RulingRequestBuilder {
    /// Construct a new `RulingRequestBuilder`
    pub fn new(id: RulingId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Set the [`RulingId`] for the request which is used to construct the endpoint URI
    pub fn id(mut self, id: RulingId) -> Self {
        self.id = id;
        self
    }

    /// Set the [`DataFormat`] for the request
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

    /// Builds the [`RulingRequest`]
    pub fn build(self) -> Result<RulingRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(RulingRequest {
            id: self.id,
            format: self.format,
            pretty: self.pretty,
        })
    }
}
