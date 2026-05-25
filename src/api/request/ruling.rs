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
#[derive(Debug, Default)]
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
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
        }

        Ok(RulingRequest {
            id: self.id,
            format: self.format,
            pretty: self.pretty,
        })
    }
}

#[cfg(test)]
mod url_tests {
    use super::*;

    #[test]
    fn ruling_default_builder_uses_known_card_uuid() {
        let req = RulingRequestBuilder::default()
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/f2b9983e-20d4-4d12-9e2c-ec6d9a345787/rulings"
        );
    }

    #[test]
    fn ruling_card_uuid() {
        let id = Uuid::from_str("f2b9983e-20d4-4d12-9e2c-ec6d9a345787")
            .expect("valid Scryfall card uuid");
        let req = RulingRequest::builder()
            .id(RulingId::Card(id))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/f2b9983e-20d4-4d12-9e2c-ec6d9a345787/rulings"
        );
    }

    #[test]
    fn ruling_multiverse_id() {
        let req = RulingRequest::builder()
            .id(RulingId::Multiverse(100))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/multiverse/100/rulings"
        );
    }

    #[test]
    fn ruling_mtgo_id() {
        let req = RulingRequest::builder()
            .id(RulingId::Mtgo(200))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/mtgo/200/rulings"
        );
    }

    #[test]
    fn ruling_arena_id() {
        let req = RulingRequest::builder()
            .id(RulingId::Arena(300))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/arena/300/rulings"
        );
    }

    #[test]
    fn ruling_set_and_collector() {
        let req = RulingRequest::builder()
            .id(RulingId::SetAndCollector((
                "mh2".to_string(),
                "42".to_string(),
            )))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/mh2/42/rulings"
        );
    }

    #[test]
    fn ruling_set_and_collector_with_format_and_pretty() {
        let req = RulingRequest::builder()
            .id(RulingId::SetAndCollector((
                "mh2".to_string(),
                "42".to_string(),
            )))
            .data_format(DataFormat::Json)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/mh2/42/rulings?format=json&pretty=true"
        );
    }

    #[test]
    fn ruling_pretty_only() {
        let req = RulingRequest::builder()
            .id(RulingId::Multiverse(100))
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/multiverse/100/rulings?pretty=true"
        );
    }

    #[test]
    fn ruling_format_json_only() {
        let req = RulingRequest::builder()
            .id(RulingId::Mtgo(200))
            .data_format(DataFormat::Json)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/mtgo/200/rulings?format=json"
        );
    }

    #[test]
    fn ruling_builder_rejects_csv() {
        let err = RulingRequest::builder()
            .id(RulingId::Multiverse(1))
            .data_format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for rulings");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Csv)));
    }

    #[test]
    fn ruling_builder_rejects_text() {
        let err = RulingRequest::builder()
            .id(RulingId::Multiverse(1))
            .data_format(DataFormat::Text)
            .build()
            .expect_err("Text is not a valid format for rulings");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Text)));
    }

    #[test]
    fn ruling_builder_rejects_image() {
        let err = RulingRequest::builder()
            .id(RulingId::Multiverse(1))
            .data_format(DataFormat::Image)
            .build()
            .expect_err("Image is not a valid format for rulings");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Image)));
    }

    #[test]
    fn ruling_builder_rejects_file() {
        let err = RulingRequest::builder()
            .id(RulingId::Multiverse(1))
            .data_format(DataFormat::File)
            .build()
            .expect_err("File is not a valid format for rulings");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::File)));
    }
}
