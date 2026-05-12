use crate::objects::{list::List, set::Set};

use super::{BASE_URL, DataFormat, ScryfallApiError, ScryfallRequest, add_query_pair};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SetsRequest {
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SetsRequest {
    pub fn builder() -> SetRequestBuilder {
        SetRequestBuilder::default()
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

#[derive(Default)]
pub struct SetRequestBuilder {
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SetRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<SetsRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(SetsRequest {
            format: self.format,
            pretty: self.pretty,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetId {
    Code(String),
    TcgPlayer(u32),
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

#[derive(Debug, Clone)]
pub struct SetFromIdRequest {
    id: SetId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SetFromIdRequest {
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

#[derive(Default)]
pub struct SetFromIdRequestBuilder {
    id: SetId,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SetFromIdRequestBuilder {
    pub fn new(id: SetId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn id(mut self, id: SetId) -> Self {
        self.id = id;
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

    pub fn build(self) -> Result<SetFromIdRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(SetFromIdRequest {
            id: self.id,
            format: self.format,
            pretty: self.pretty,
        })
    }
}
