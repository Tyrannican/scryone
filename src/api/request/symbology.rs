//! Request types and builders for Mana Symbols and costs from Scryfall API
//!
//! More detailed information here: <https://scryfall.com/docs/api/card-symbols>
use super::{BASE_URL, ScryfallApiError, ScryfallRequest, add_query_pair};
use crate::{
    api::request::DataFormat,
    objects::{CardSymbol, List, ManaCost},
};

use url::Url;

/// Request type for the `/symbology` Scryfall endpoint
#[derive(Debug, Clone)]
pub struct SymbolListRequest {
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SymbolListRequest {
    /// Constructs a builder for a `SymbolListRequest`
    pub fn builder() -> SymbolListRequestBuilder {
        SymbolListRequestBuilder::default()
    }
}

impl ScryfallRequest for SymbolListRequest {
    type Response = List<CardSymbol>;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(&format!("{BASE_URL}/symbology"))?;
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

/// Builder for constructing a [`SymbolListRequest`]
#[derive(Default)]
pub struct SymbolListRequestBuilder {
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SymbolListRequestBuilder {
    /// Constructs a new `SymbolListRequestBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the [`DataFormat`] for the response
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

    /// Builds the [`SymbolListRequest`]
    pub fn build(self) -> Result<SymbolListRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }

        Ok(SymbolListRequest {
            format: self.format,
            pretty: self.pretty,
        })
    }
}

/// Request type for calling the `/symbology/parse-mana` endpoint
#[derive(Debug, Clone)]
pub struct ParseManaRequest {
    cost: String,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl ParseManaRequest {
    /// Constructs a builder for a `ParseManaRequest`
    pub fn builder() -> ParseManaRequestBuilder {
        ParseManaRequestBuilder::default()
    }
}

impl ScryfallRequest for ParseManaRequest {
    type Response = ManaCost;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let path = "/symbology/parse-mana".to_string();
        url = url.join(&path)?;

        url.query_pairs_mut().append_pair("cost", &self.cost);
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

/// Builder for constructing a [`ParseManaRequest`]
#[derive(Default)]
pub struct ParseManaRequestBuilder {
    cost: String,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl ParseManaRequestBuilder {
    /// Constructs a new `ParseManaRequestBuilder`
    pub fn new(cost: impl AsRef<str>) -> Self {
        Self {
            cost: cost.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Sets the `cost` field which is the mana value to parse
    pub fn cost(mut self, cost: impl AsRef<str>) -> Self {
        self.cost = cost.as_ref().to_string();
        self
    }

    /// Sets the [`DataFormat`] for the response
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

    /// Builds the [`ParseManaRequest`]
    pub fn build(self) -> Result<ParseManaRequest, ScryfallApiError> {
        if self.cost.is_empty() {
            return Err(ScryfallApiError::ExpectedField("cost".to_string()));
        }

        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }

        Ok(ParseManaRequest {
            cost: self.cost,
            format: self.format,
            pretty: self.pretty,
        })
    }
}
