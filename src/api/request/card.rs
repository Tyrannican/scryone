use super::{
    BASE_URL, DataFormat, ImageVersion, ScryfallApiError, ScryfallRequest, SortDirection,
    SortOrder, UniqueMode, add_query_pair,
};
use std::str::FromStr;

use crate::{
    api::ScryfallPostRequest,
    objects::{
        card::{Card, CardIdentifier},
        catalog::Catalog,
        list::List,
        types::Language,
    },
};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CardSearchRequest {
    query: String,
    unique: Option<UniqueMode>,
    order: Option<SortOrder>,
    dir: Option<SortDirection>,
    include_extras: Option<bool>,
    include_multilingual: Option<bool>,
    include_variations: Option<bool>,
    page: Option<u32>,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl CardSearchRequest {
    pub fn builder() -> CardSearchRequestBuilder {
        CardSearchRequestBuilder::default()
    }
}

impl ScryfallRequest for CardSearchRequest {
    type Response = List<Vec<Card>>;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let path = format!("/cards/search?q={}", self.query);
        url = url.join(&path)?;

        add_query_pair!(url, self.unique, "unique");
        add_query_pair!(url, self.order, "order");
        add_query_pair!(url, self.dir, "dir");
        add_query_pair!(url, self.include_extras, "include_extras");
        add_query_pair!(url, self.include_multilingual, "include_multilingual");
        add_query_pair!(url, self.include_variations, "include_variations");
        add_query_pair!(url, self.page, "page");
        add_query_pair!(url, self.format, "format");
        add_query_pair!(url, self.pretty, "pretty");

        Ok(url)
    }
}

#[derive(Default)]
pub struct CardSearchRequestBuilder {
    query: String,
    unique: Option<UniqueMode>,
    order: Option<SortOrder>,
    dir: Option<SortDirection>,
    include_extras: Option<bool>,
    include_multilingual: Option<bool>,
    include_variations: Option<bool>,
    page: Option<u32>,
    format: Option<DataFormat>,
    pretty: Option<bool>,
}

impl CardSearchRequestBuilder {
    pub fn new(query: impl AsRef<str>) -> Self {
        Self {
            query: query.as_ref().to_string(),
            ..Default::default()
        }
    }

    pub fn query(mut self, query: impl AsRef<str>) -> Self {
        self.query = query.as_ref().to_string();
        self
    }

    pub fn unique_mode(mut self, unique_mode: UniqueMode) -> Self {
        self.unique = Some(unique_mode);
        self
    }

    pub fn sort_order(mut self, ordering: SortOrder) -> Self {
        self.order = Some(ordering);
        self
    }

    pub fn sort_direction(mut self, dir: SortDirection) -> Self {
        self.dir = Some(dir);
        self
    }

    pub fn include_extras(mut self, flag: bool) -> Self {
        self.include_extras = Some(flag);
        self
    }

    pub fn include_multilingual(mut self, flag: bool) -> Self {
        self.include_multilingual = Some(flag);
        self
    }

    pub fn include_variations(mut self, flag: bool) -> Self {
        self.include_variations = Some(flag);
        self
    }

    pub fn page(mut self, page_no: u32) -> Self {
        self.page = Some(page_no);
        self
    }

    pub fn data_format(mut self, data_format: DataFormat) -> Self {
        self.format = Some(data_format);
        self
    }

    pub fn prettify_json(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<CardSearchRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json | DataFormat::Csv) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(CardSearchRequest {
            query: self.query,
            unique: self.unique,
            order: self.order,
            dir: self.dir,
            include_extras: self.include_extras,
            include_multilingual: self.include_multilingual,
            include_variations: self.include_variations,
            page: self.page,
            format: self.format,
            pretty: self.pretty,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NamedCardRequest {
    exact: Option<String>,
    fuzzy: Option<String>,
    set: Option<String>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl NamedCardRequest {
    pub fn builder() -> NamedCardRequestBuilder {
        NamedCardRequestBuilder::default()
    }
}

impl ScryfallRequest for NamedCardRequest {
    type Response = Card;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?.join("/cards/named")?;
        if let Some(ref exact) = self.exact {
            url.query_pairs_mut().append_pair("exact", &exact);
        }

        if let Some(ref fuzzy) = self.fuzzy {
            url.query_pairs_mut().append_pair("fuzzy", &fuzzy);
        }

        add_query_pair!(url, &self.set, "set");
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.face, "face");
        add_query_pair!(url, &self.version, "version");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

#[derive(Default)]
pub struct NamedCardRequestBuilder {
    exact: Option<String>,
    fuzzy: Option<String>,
    set: Option<String>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl NamedCardRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exact_name(mut self, exact: impl AsRef<str>) -> Self {
        self.exact = Some(exact.as_ref().to_string());
        self
    }

    pub fn fuzzy_search(mut self, input: impl AsRef<str>) -> Self {
        self.fuzzy = Some(input.as_ref().to_string());
        self
    }

    pub fn set_code(mut self, code: impl AsRef<str>) -> Self {
        self.set = Some(code.as_ref().to_string());
        self
    }

    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    pub fn face(mut self, face: impl AsRef<str>) -> Self {
        self.face = Some(face.as_ref().to_string());
        self
    }

    pub fn image_version(mut self, img_version: ImageVersion) -> Self {
        self.version = Some(img_version);
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<NamedCardRequest, ScryfallApiError> {
        if self.exact.is_none() && self.fuzzy.is_none() {
            return Err(ScryfallApiError::ExpectedFieldsOneOf(vec![
                "exact".to_string(),
                "fuzzy".to_string(),
            ]));
        }

        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json | DataFormat::Text | DataFormat::Image) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(NamedCardRequest {
            exact: self.exact,
            fuzzy: self.fuzzy,
            set: self.set,
            format: self.format,
            face: self.face,
            version: self.version,
            pretty: self.pretty,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CardAutoCompleteRequest {
    query: String,
    format: Option<DataFormat>,
    pretty: Option<bool>,
    include_extras: Option<bool>,
}

impl CardAutoCompleteRequest {
    pub fn builder() -> CardAutoCompleteRequestBuilder {
        CardAutoCompleteRequestBuilder::default()
    }
}

impl ScryfallRequest for CardAutoCompleteRequest {
    type Response = Catalog;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?.join("/cards/autocomplete")?;
        url.query_pairs_mut().append_pair("q", &self.query);
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");
        add_query_pair!(url, &self.include_extras, "include_extras");

        Ok(url)
    }
}

#[derive(Default)]
pub struct CardAutoCompleteRequestBuilder {
    query: String,
    format: Option<DataFormat>,
    pretty: Option<bool>,
    include_extras: Option<bool>,
}

impl CardAutoCompleteRequestBuilder {
    pub fn new(query: impl AsRef<str>) -> Self {
        Self {
            query: query.as_ref().to_string(),
            ..Default::default()
        }
    }

    pub fn query(mut self, query: impl AsRef<str>) -> Self {
        self.query = query.as_ref().to_string();
        self
    }

    pub fn format(mut self, data_format: DataFormat) -> Self {
        self.format = Some(data_format);
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn include_extras(mut self, flag: bool) -> Self {
        self.include_extras = Some(flag);
        self
    }

    pub fn build(self) -> Result<CardAutoCompleteRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(CardAutoCompleteRequest {
            query: self.query,
            format: self.format,
            pretty: self.pretty,
            include_extras: self.include_extras,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RandomCardRequest {
    query: Option<String>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl RandomCardRequest {
    pub fn builder() -> RandomCardRequestBuilder {
        RandomCardRequestBuilder::default()
    }
}

impl ScryfallRequest for RandomCardRequest {
    type Response = Card;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?.join("/cards/random")?;
        add_query_pair!(url, &self.query, "q");
        add_query_pair!(url, &self.face, "face");
        add_query_pair!(url, &self.version, "version");
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

#[derive(Default)]
pub struct RandomCardRequestBuilder {
    query: Option<String>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl RandomCardRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(mut self, query: impl AsRef<str>) -> Self {
        self.query = Some(query.as_ref().to_string());
        self
    }

    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    pub fn face(mut self, face: impl AsRef<str>) -> Self {
        self.face = Some(face.as_ref().to_string());
        self
    }

    pub fn image_version(mut self, version: ImageVersion) -> Self {
        self.version = Some(version);
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<RandomCardRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json | DataFormat::Text | DataFormat::Image) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(RandomCardRequest {
            query: self.query,
            format: self.format,
            face: self.face,
            version: self.version,
            pretty: self.pretty,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CardCollectionRequest {
    identifiers: Vec<CardIdentifier>,
    pretty: Option<bool>,
}

impl ScryfallRequest for CardCollectionRequest {
    type Response = List<Card>;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?
            .join("/cards/collection")
            .map_err(|e| ScryfallApiError::UrlParse(e))?;

        add_query_pair!(url, &self.pretty, "pretty");
        Ok(url)
    }
}

impl ScryfallPostRequest for CardCollectionRequest {
    type Body = Vec<CardIdentifier>;

    fn body(&self) -> &Self::Body {
        &self.identifiers
    }
}

#[derive(Default)]
pub struct CardCollectionRequestBuilder {
    identifiers: Vec<CardIdentifier>,
    pretty: Option<bool>,
}

impl CardCollectionRequestBuilder {
    pub fn new(identifiers: Vec<CardIdentifier>) -> Self {
        Self {
            identifiers,
            ..Default::default()
        }
    }

    pub fn identifiers(mut self, identifiers: Vec<CardIdentifier>) -> Self {
        self.identifiers = identifiers;
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<CardCollectionRequest, ScryfallApiError> {
        if self.identifiers.is_empty() {
            return Err(ScryfallApiError::InvalidData(
                "supplied identifiers list cannot be empty".to_string(),
            ));
        }

        Ok(CardCollectionRequest {
            identifiers: self.identifiers,
            pretty: self.pretty,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SingleCardRequest {
    set_code: String,
    collector_number: String,
    language: Option<Language>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl SingleCardRequest {
    pub fn builder() -> SingleCardRequestBuilder {
        SingleCardRequestBuilder::default()
    }
}

impl ScryfallRequest for SingleCardRequest {
    type Response = Card;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let path = if let Some(lang) = self.language {
            format!("/cards/{}/{}/{lang}", self.set_code, self.collector_number)
        } else {
            format!("/cards/{}/{}", self.set_code, self.collector_number)
        };
        url = url.join(&path)?;

        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.face, "face");
        add_query_pair!(url, &self.version, "version");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

#[derive(Default)]
pub struct SingleCardRequestBuilder {
    set_code: String,
    collector_number: String,
    language: Option<Language>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl SingleCardRequestBuilder {
    pub fn new(set_code: impl AsRef<str>, collector_number: impl AsRef<str>) -> Self {
        Self {
            set_code: set_code.as_ref().to_string(),
            collector_number: collector_number.as_ref().to_string(),
            ..Default::default()
        }
    }

    pub fn set_code(mut self, set_code: impl AsRef<str>) -> Self {
        self.set_code = set_code.as_ref().to_string();
        self
    }

    pub fn collector_number(mut self, collector_number: impl AsRef<str>) -> Self {
        self.collector_number = collector_number.as_ref().to_string();
        self
    }

    pub fn language(mut self, lang: Language) -> Self {
        self.language = Some(lang);
        self
    }

    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    pub fn face(mut self, face: impl AsRef<str>) -> Self {
        self.face = Some(face.as_ref().to_string());
        self
    }

    pub fn image_version(mut self, img_version: ImageVersion) -> Self {
        self.version = Some(img_version);
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<SingleCardRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json | DataFormat::Text | DataFormat::Image) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(SingleCardRequest {
            set_code: self.set_code,
            collector_number: self.collector_number,
            language: self.language,
            format: self.format,
            face: self.face,
            version: self.version,
            pretty: self.pretty,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CardId {
    Card(Uuid),
    Multiverse(u32),
    Mtgo(u32),
    Arena(u32),
    TcgPlayer(u32),
    CardMarket(u32),
}

impl CardId {
    pub(crate) fn subpaths(&self) -> (String, String) {
        match self {
            Self::Card(id) => (String::new(), id.to_string()),
            Self::Multiverse(id) => ("multiverse".to_string(), id.to_string()),
            Self::Mtgo(id) => ("mtgo".to_string(), id.to_string()),
            Self::Arena(id) => ("arena".to_string(), id.to_string()),
            Self::TcgPlayer(id) => ("tcgplayer".to_string(), id.to_string()),
            Self::CardMarket(id) => ("cardmarket".to_string(), id.to_string()),
        }
    }
}

impl Default for CardId {
    fn default() -> Self {
        Self::Card(
            Uuid::from_str("56ebc372-aabd-4174-a943-c7bf59e5028d")
                .expect("a valid UUID format from Scryfall"),
        )
    }
}

#[derive(Debug, Clone)]
pub struct CardFromIdRequest {
    id: CardId,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl CardFromIdRequest {
    pub fn builder() -> CardFromIdRequestBuilder {
        CardFromIdRequestBuilder::default()
    }
}

impl ScryfallRequest for CardFromIdRequest {
    type Response = Card;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let (subpath, id) = self.id.subpaths();
        let path = if subpath.is_empty() {
            format!("/cards/{id}")
        } else {
            format!("/cards/{subpath}/{id}")
        };
        url = url.join(&path)?;

        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.face, "face");
        add_query_pair!(url, &self.version, "version");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

#[derive(Default)]
pub struct CardFromIdRequestBuilder {
    id: CardId,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl CardFromIdRequestBuilder {
    pub fn new(id: CardId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn id(mut self, id: CardId) -> Self {
        self.id = id;
        self
    }

    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    pub fn face(mut self, face: impl AsRef<str>) -> Self {
        self.face = Some(face.as_ref().to_string());
        self
    }

    pub fn version(mut self, img_version: ImageVersion) -> Self {
        self.version = Some(img_version);
        self
    }

    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    pub fn build(self) -> Result<CardFromIdRequest, ScryfallApiError> {
        if let Some(fmt) = self.format {
            if !matches!(fmt, DataFormat::Json | DataFormat::Text | DataFormat::Image) {
                return Err(ScryfallApiError::InvalidDataFormat(fmt));
            }
        }

        Ok(CardFromIdRequest {
            id: self.id,
            format: self.format,
            face: self.face,
            version: self.version,
            pretty: self.pretty,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_url(flag: &str, opt: impl ScryfallRequest) {
        let url = opt.to_url();
        assert!(url.is_ok());
        let url = url.unwrap();
        eprintln!("{flag}: {url}");
    }

    #[test]
    fn it_parses_search_request() {
        let opt = CardSearchRequestBuilder::new("austere command")
            .unique_mode(UniqueMode::Prints)
            .sort_direction(SortDirection::Desc)
            .page(2)
            .build()
            .expect("a valid request");

        check_url("SEARCH", opt);
    }

    #[test]
    fn it_parses_named_card() {
        let opt = NamedCardRequestBuilder::new()
            .exact_name("austere command")
            .image_version(ImageVersion::ArtCrop)
            .data_format(DataFormat::Json)
            .set_code("aes")
            .build()
            .expect("a valid request");
        check_url("NAMED", opt);
    }

    #[test]
    fn card_id() {
        let ids = [
            CardId::Card(
                Uuid::from_str("56ebc372-aabd-4174-a943-c7bf59e5028d")
                    .expect("a valid UUID format from Scryfall"),
            ),
            CardId::Mtgo(123),
            CardId::CardMarket(456),
            CardId::Arena(789),
            CardId::TcgPlayer(012),
        ];

        for id in ids {
            let opt = CardFromIdRequest::builder()
                .id(id)
                .data_format(DataFormat::Json)
                .build()
                .expect("a valid request");
            check_url("CARD ID", opt);
        }
    }
}
