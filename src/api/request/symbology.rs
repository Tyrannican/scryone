use super::{BASE_URL, ScryfallApiError, ScryfallRequest, add_query_pair};
use crate::{
    api::request::DataFormat,
    objects::{CardSymbol, List, ManaCost},
};

use url::Url;

#[derive(Debug, Clone)]
pub struct SymbolListRequest {
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SymbolListRequest {
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

#[derive(Default)]
pub struct SymbolListRequestBuilder {
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl SymbolListRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<SymbolListRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(SymbolListRequest {
            format: self.format,
            pretty: self.pretty,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParseManaRequest {
    cost: String,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl ParseManaRequest {
    pub fn builder() -> ParseManaRequestBuilder {
        ParseManaRequestBuilder::default()
    }
}

impl ScryfallRequest for ParseManaRequest {
    type Response = ManaCost;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let path = format!("/symbology/parse-mana");
        url = url.join(&path)?;

        url.query_pairs_mut().append_pair("cost", &self.cost);
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

#[derive(Default)]
pub struct ParseManaRequestBuilder {
    cost: String,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl ParseManaRequestBuilder {
    pub fn new(cost: impl AsRef<str>) -> Self {
        Self {
            cost: cost.as_ref().to_string(),
            ..Default::default()
        }
    }

    pub fn cost(mut self, cost: impl AsRef<str>) -> Self {
        self.cost = cost.as_ref().to_string();
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

    pub fn build(self) -> Result<ParseManaRequest, ScryfallApiError> {
        if self.cost.is_empty() {
            return Err(ScryfallApiError::ExpectedField("cost".to_string()));
        }

        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(ParseManaRequest {
            cost: self.cost,
            format: self.format,
            pretty: self.pretty,
        })
    }
}
