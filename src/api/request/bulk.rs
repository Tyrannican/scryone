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

#[derive(Debug, Clone)]
pub struct BulkDataRequest {
    pretty: Option<bool>,
}

impl BulkDataRequest {
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

#[derive(Default)]
pub struct BulkDataRequestBuilder {
    pretty: Option<bool>,
}

impl BulkDataRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<BulkDataRequest, ScryfallApiError> {
        Ok(BulkDataRequest {
            pretty: self.pretty,
        })
    }
}

#[derive(Debug, Clone)]
pub struct BulkDataFromIdRequest {
    data_type: BulkDataId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl BulkDataFromIdRequest {
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

#[derive(Default)]
pub struct BulkDataFromIdRequestBuilder {
    data_type: BulkDataId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl BulkDataFromIdRequestBuilder {
    pub fn new(data_type: BulkDataId) -> Self {
        Self {
            data_type,
            ..Default::default()
        }
    }

    pub fn data_type(mut self, data_type: BulkDataId) -> Self {
        self.data_type = data_type;
        self
    }

    pub fn format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

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

#[derive(Debug, Copy, Clone)]
pub enum BulkDataId {
    Type(BulkDataType),
    Id(Uuid),
}

impl Default for BulkDataId {
    fn default() -> Self {
        Self::Type(BulkDataType::OracleCards)
    }
}
