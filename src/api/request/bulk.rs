//! Request types and builders for Bulk Data Files from Scryfall API
//!
//! More detailed information here: <https://scryfall.com/docs/api/bulk-data>
use super::add_query_pair;
use crate::api::{
    error::ScryfallApiError,
    request::{BASE_URL, DataFormat, ScryfallRequest},
};
use crate::objects::bulk::BulkData;
use crate::objects::list::List;
use crate::objects::types::BulkDataType;

use url::Url;
use uuid::Uuid;

/// Request type for calling the `/bulk-data` Scryfall endpoint
#[derive(Debug, Clone)]
pub struct BulkDataRequest {
    pretty: Option<bool>,
}

impl BulkDataRequest {
    /// Construct a builder for a `BulkDataRequest`
    pub fn builder() -> BulkDataRequestBuilder {
        BulkDataRequestBuilder::default()
    }
}

impl ScryfallRequest for BulkDataRequest {
    type Response = List<BulkData>;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?.join("/bulk-data")?;
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

/// Builder for constructing a [`BulkDataRequest`]
#[derive(Debug, Default)]
pub struct BulkDataRequestBuilder {
    pretty: Option<bool>,
}

impl BulkDataRequestBuilder {
    /// Construct a new `BulkDataRequestBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the `pretty` flag for prettifying JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Build the [`BulkDataRequest`]
    pub fn build(self) -> Result<BulkDataRequest, ScryfallApiError> {
        Ok(BulkDataRequest {
            pretty: self.pretty,
        })
    }
}

/// Request type for calling the `/bulk-data/:id` and `/bulk-data/:type` Scryfall endpoints
#[derive(Debug, Clone)]
pub struct BulkDataFromIdRequest {
    data_type: BulkDataId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl BulkDataFromIdRequest {
    /// Construct a builder for a `BulkDataFromIdRequest`
    pub fn builder() -> BulkDataFromIdRequestBuilder {
        BulkDataFromIdRequestBuilder::default()
    }
}

impl ScryfallRequest for BulkDataFromIdRequest {
    type Response = BulkData;

    fn to_url(&self) -> Result<url::Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let path = match self.data_type {
            BulkDataId::Id(id) => format!("/bulk-data/{}", id),
            BulkDataId::Type(ty) => {
                format!("/bulk-data/{}", ty.to_string().replace("_", "-"))
            }
        };

        url = url.join(&path)?;
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

/// Builder for constructing a [`BulkDataFromIdRequest`]
#[derive(Debug, Default)]
pub struct BulkDataFromIdRequestBuilder {
    data_type: BulkDataId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl BulkDataFromIdRequestBuilder {
    /// Construct a new `BulkDataFromIdRequestBuilder`
    pub fn new(data_type: BulkDataId) -> Self {
        Self {
            data_type,
            ..Default::default()
        }
    }

    /// Set the ID or Type for the Bulk Data requested
    pub fn data_type(mut self, data_type: BulkDataId) -> Self {
        self.data_type = data_type;
        self
    }

    /// Set the [`DataFormat`] for this request
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    /// * [`DataFormat::File`]
    pub fn format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    /// Set the `pretty` flag for prettifying JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Build the [`BulkDataFromIdRequest`]
    pub fn build(self) -> Result<BulkDataFromIdRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json | DataFormat::File)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
        }

        Ok(BulkDataFromIdRequest {
            data_type: self.data_type,
            format: self.format,
            pretty: self.pretty,
        })
    }
}

/// Type of Bulk Data type to request when calling `/bulk-data/:id` or `/bulk-data/:type` Scryfall
/// Endpoints
#[derive(Debug, Copy, Clone)]
pub enum BulkDataId {
    /// Request a [`BulkDataType`]
    Type(BulkDataType),

    /// Request Bulk Data from a UUID
    Id(Uuid),
}

impl Default for BulkDataId {
    fn default() -> Self {
        Self::Type(BulkDataType::OracleCards)
    }
}

#[cfg(test)]
mod url_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn bulk_data_default() {
        let req = BulkDataRequest::builder().build().expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/bulk-data");
    }

    #[test]
    fn bulk_data_pretty_true() {
        let req = BulkDataRequest::builder()
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data?pretty=true"
        );
    }

    #[test]
    fn bulk_data_pretty_false() {
        let req = BulkDataRequest::builder()
            .pretty(false)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data?pretty=false"
        );
    }

    #[test]
    fn bulk_data_from_id_type_oracle_cards() {
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::OracleCards))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data/oracle-cards"
        );
    }

    #[test]
    fn bulk_data_from_id_type_unique_artwork() {
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::UniqueArtwork))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data/unique-artwork"
        );
    }

    #[test]
    fn bulk_data_from_id_type_default_cards() {
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::DefaultCards))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data/default-cards"
        );
    }

    #[test]
    fn bulk_data_from_id_type_all_cards() {
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::AllCards))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/bulk-data/all-cards");
    }

    #[test]
    fn bulk_data_from_id_type_rulings() {
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::Rulings))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/bulk-data/rulings");
    }

    #[test]
    fn bulk_data_from_id_uuid() {
        let id = Uuid::from_str("27bf3214-1271-490a-bcb2-5da9c5066ef1")
            .expect("valid Scryfall bulk data UUID");
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Id(id))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data/27bf3214-1271-490a-bcb2-5da9c5066ef1"
        );
    }

    #[test]
    fn bulk_data_from_id_format_json_pretty() {
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::UniqueArtwork))
            .format(DataFormat::Json)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data/unique-artwork?format=json&pretty=true"
        );
    }

    #[test]
    fn bulk_data_from_id_format_file() {
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::OracleCards))
            .format(DataFormat::File)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data/oracle-cards?format=file"
        );
    }

    #[test]
    fn bulk_data_from_id_uuid_with_options() {
        let id = Uuid::from_str("27bf3214-1271-490a-bcb2-5da9c5066ef1")
            .expect("valid Scryfall bulk data UUID");
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Id(id))
            .format(DataFormat::Json)
            .pretty(false)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/bulk-data/27bf3214-1271-490a-bcb2-5da9c5066ef1?format=json&pretty=false"
        );
    }

    #[test]
    fn bulk_data_from_id_rejects_csv() {
        let err = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::OracleCards))
            .format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /bulk-data");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Csv)));
    }

    #[test]
    fn bulk_data_from_id_rejects_text() {
        let err = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::OracleCards))
            .format(DataFormat::Text)
            .build()
            .expect_err("Text is not a valid format for /bulk-data");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Text)));
    }

    #[test]
    fn bulk_data_from_id_rejects_image() {
        let err = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::OracleCards))
            .format(DataFormat::Image)
            .build()
            .expect_err("Image is not a valid format for /bulk-data");
        assert!(matches!(err, ScryfallApiError::InvalidDataFormat(DataFormat::Image)));
    }
}
