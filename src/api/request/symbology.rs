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
#[derive(Debug, Default)]
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
            && !matches!(fmt, DataFormat::Json)
        {
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
#[derive(Debug, Default)]
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
            && !matches!(fmt, DataFormat::Json)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
        }

        Ok(ParseManaRequest {
            cost: self.cost,
            format: self.format,
            pretty: self.pretty,
        })
    }
}

#[cfg(test)]
mod url_tests {
    use super::*;

    #[test]
    fn symbology_default_builder_produces_bare_endpoint() {
        let req = SymbolListRequest::builder()
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/symbology");
    }

    #[test]
    fn symbology_pretty_only() {
        let req = SymbolListRequest::builder()
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/symbology?pretty=true"
        );
    }

    #[test]
    fn symbology_format_json_only() {
        let req = SymbolListRequest::builder()
            .format(DataFormat::Json)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/symbology?format=json"
        );
    }

    #[test]
    fn symbology_format_and_pretty_combined() {
        let req = SymbolListRequest::builder()
            .format(DataFormat::Json)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/symbology?format=json&pretty=true"
        );
    }

    #[test]
    fn symbology_builder_rejects_csv() {
        let err = SymbolListRequest::builder()
            .format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /symbology");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Csv)));
    }

    #[test]
    fn symbology_builder_rejects_text() {
        let err = SymbolListRequest::builder()
            .format(DataFormat::Text)
            .build()
            .expect_err("Text is not a valid format for /symbology");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Text)));
    }

    #[test]
    fn symbology_builder_rejects_image() {
        let err = SymbolListRequest::builder()
            .format(DataFormat::Image)
            .build()
            .expect_err("Image is not a valid format for /symbology");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Image)));
    }

    #[test]
    fn symbology_builder_rejects_file() {
        let err = SymbolListRequest::builder()
            .format(DataFormat::File)
            .build()
            .expect_err("File is not a valid format for /symbology");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::File)));
    }

    #[test]
    fn parse_mana_simple_cost() {
        let req = ParseManaRequest::builder()
            .cost("3WR")
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/symbology/parse-mana?cost=3WR"
        );
    }

    #[test]
    fn parse_mana_complex_cost_is_percent_encoded() {
        let req = ParseManaRequest::builder()
            .cost("{X}{R/G}{2/U}")
            .format(DataFormat::Json)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/symbology/parse-mana?cost=%7BX%7D%7BR%2FG%7D%7B2%2FU%7D&format=json&pretty=true"
        );
    }

    #[test]
    fn parse_mana_cost_setter_overrides_empty_default() {
        let req = ParseManaRequestBuilder::default()
            .cost("UU")
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/symbology/parse-mana?cost=UU"
        );
    }

    #[test]
    fn parse_mana_missing_cost_errors() {
        let err = ParseManaRequestBuilder::default()
            .build()
            .expect_err("cost is required for /symbology/parse-mana");
        assert!(matches!(err, ScryfallApiError::ExpectedField(ref field) if field == "cost"));
    }

    #[test]
    fn parse_mana_rejects_csv_format() {
        let err = ParseManaRequest::builder()
            .cost("3R")
            .format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /symbology/parse-mana");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Csv)));
    }

    #[test]
    fn parse_mana_rejects_text_format() {
        let err = ParseManaRequest::builder()
            .cost("3R")
            .format(DataFormat::Text)
            .build()
            .expect_err("Text is not a valid format for /symbology/parse-mana");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Text)));
    }

    #[test]
    fn parse_mana_rejects_image_format() {
        let err = ParseManaRequest::builder()
            .cost("3R")
            .format(DataFormat::Image)
            .build()
            .expect_err("Image is not a valid format for /symbology/parse-mana");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Image)));
    }

    #[test]
    fn parse_mana_rejects_file_format() {
        let err = ParseManaRequest::builder()
            .cost("3R")
            .format(DataFormat::File)
            .build()
            .expect_err("File is not a valid format for /symbology/parse-mana");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::File)));
    }
}
