//! Request types and builders for Bulk Data Files from Scryfall
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
#[derive(Default)]
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
            BulkDataId::Id(id) => format!("/bulk-data/{}", id.to_string()),
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
#[derive(Default)]
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
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json | DataFormat::File) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
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
