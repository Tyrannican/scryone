//! Request types and builders for Cards from Scryfall API
//!
//! More detailed information here: <https://scryfall.com/docs/api/cards>
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

/// Request type for calling the `/cards/search` Scryfall endpoint
///
/// Returns a [`List`] object containing Cards found using a fulltext search string
/// Response is paginated, returning 175 cards per page (as per Scryfall's documentation)
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
    /// Construct a builder for a `CardSearchRequest`
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

/// Builder for constructing a [`CardSearchRequest`]
#[derive(Debug, Default)]
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
    /// Construct a new `CardSearchRequestBuilder`
    pub fn new(query: impl AsRef<str>) -> Self {
        Self {
            query: query.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Set the search query for the request
    pub fn query(mut self, query: impl AsRef<str>) -> Self {
        self.query = query.as_ref().to_string();
        self
    }

    /// Set the type of unique cards returned
    pub fn unique_mode(mut self, unique_mode: UniqueMode) -> Self {
        self.unique = Some(unique_mode);
        self
    }

    /// Set the sort order of returned results
    pub fn sort_order(mut self, ordering: SortOrder) -> Self {
        self.order = Some(ordering);
        self
    }

    /// Set the sorting direction of the returned results
    pub fn sort_direction(mut self, dir: SortDirection) -> Self {
        self.dir = Some(dir);
        self
    }

    /// Sets the flag for including extra cards in the response
    pub fn include_extras(mut self, flag: bool) -> Self {
        self.include_extras = Some(flag);
        self
    }

    /// Sets the flag for including every language supported by Scryfall
    pub fn include_multilingual(mut self, flag: bool) -> Self {
        self.include_multilingual = Some(flag);
        self
    }

    /// Sets the flag for including rare card variants in the response
    pub fn include_variations(mut self, flag: bool) -> Self {
        self.include_variations = Some(flag);
        self
    }

    /// Set the page number to return
    pub fn page(mut self, page_no: u32) -> Self {
        self.page = Some(page_no);
        self
    }

    /// Set the [`DataFormat`] to return
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    /// * [`DataFormat::Csv`]
    pub fn data_format(mut self, data_format: DataFormat) -> Self {
        self.format = Some(data_format);
        self
    }

    /// Sets the flag for prettifying the JSON response
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Build the [`CardSearchRequest`]
    pub fn build(self) -> Result<CardSearchRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json | DataFormat::Csv)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
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

/// Request type for calling the `/cards/named` Scryfall endpoint
///
/// Returns a Card based on a name search string
///
/// `exact` matches card names exactly, otherwise a 404 is returned
/// `fuzzy` will perform a fuzzy search for the card and return the closest match to the input
/// string, otherwise a 404 is returned
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
    /// Construct a builder for a `NamedCardRequest`
    pub fn builder() -> NamedCardRequestBuilder {
        NamedCardRequestBuilder::default()
    }
}

impl ScryfallRequest for NamedCardRequest {
    type Response = Card;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?.join("/cards/named")?;
        if let Some(ref exact) = self.exact {
            url.query_pairs_mut().append_pair("exact", exact);
        }

        if let Some(ref fuzzy) = self.fuzzy {
            url.query_pairs_mut().append_pair("fuzzy", fuzzy);
        }

        add_query_pair!(url, &self.set, "set");
        add_query_pair!(url, &self.format, "format");
        add_query_pair!(url, &self.face, "face");
        add_query_pair!(url, &self.version, "version");
        add_query_pair!(url, &self.pretty, "pretty");

        Ok(url)
    }
}

/// Builder for constructing a [`NamedCardRequest`]
#[derive(Debug, Default)]
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
    /// Construct a new `NamedCardRequestBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text for an exact card name search, case insensitive
    pub fn exact_name(mut self, exact: impl AsRef<str>) -> Self {
        self.exact = Some(exact.as_ref().to_string());
        self
    }

    /// Sets the text for a fuzzy card name search
    pub fn fuzzy_search(mut self, input: impl AsRef<str>) -> Self {
        self.fuzzy = Some(input.as_ref().to_string());
        self
    }

    /// Sets the set code to limit the search to one set
    pub fn set_code(mut self, code: impl AsRef<str>) -> Self {
        self.set = Some(code.as_ref().to_string());
        self
    }

    /// Sets the [`DataFormat`] to use for the response
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    /// * [`DataFormat::Text`]
    /// * [`DataFormat::Image`]
    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    /// Sets the card face to return when using the `image` data format
    pub fn face(mut self, face: impl AsRef<str>) -> Self {
        self.face = Some(face.as_ref().to_string());
        self
    }

    /// Sets the image version to return when using the `image` data format
    pub fn image_version(mut self, img_version: ImageVersion) -> Self {
        self.version = Some(img_version);
        self
    }

    /// Sets the flag for prettifying the JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Builds the [`NamedCardRequest`]
    pub fn build(self) -> Result<NamedCardRequest, ScryfallApiError> {
        if self.exact.is_none() && self.fuzzy.is_none() {
            return Err(ScryfallApiError::ExpectedFieldsOneOf(vec![
                "exact".to_string(),
                "fuzzy".to_string(),
            ]));
        }

        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json | DataFormat::Text | DataFormat::Image)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
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

/// Request type for calling the `/cards/autocomplete` Scryfall endpoint
///
/// Returns a [`Catalog`] object containing up to 20 full English card names that could be
/// autocompletions of the given string parameter
///
/// Names are sorted with the nearest match first
///
/// If the `query` parameter is less than 2 characters long, or no names match, the [`Catalog`]
/// will contain 0 items (instead of an error as per Scryfall's documentation)
#[derive(Debug, Clone)]
pub struct CardAutoCompleteRequest {
    query: String,
    format: Option<DataFormat>,
    pretty: Option<bool>,
    include_extras: Option<bool>,
}

impl CardAutoCompleteRequest {
    /// Construct a builder for a `CardAutoCompleteRequest`
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

/// Builder for constructing a [`CardAutoCompleteRequest`]
#[derive(Debug, Default)]
pub struct CardAutoCompleteRequestBuilder {
    query: String,
    format: Option<DataFormat>,
    pretty: Option<bool>,
    include_extras: Option<bool>,
}

impl CardAutoCompleteRequestBuilder {
    /// Construct a new `CardAutoCompleteRequestBuilder`
    pub fn new(query: impl AsRef<str>) -> Self {
        Self {
            query: query.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Sets the query string for the autocomplete text
    pub fn query(mut self, query: impl AsRef<str>) -> Self {
        self.query = query.as_ref().to_string();
        self
    }

    /// Sets the [`DataFormat`] for the response
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    pub fn format(mut self, data_format: DataFormat) -> Self {
        self.format = Some(data_format);
        self
    }

    /// Sets the flag for prettifying JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Sets the flag for including extra cards in the response
    pub fn include_extras(mut self, flag: bool) -> Self {
        self.include_extras = Some(flag);
        self
    }

    /// Builds the [`CardAutoCompleteRequest`]
    pub fn build(self) -> Result<CardAutoCompleteRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
        }

        Ok(CardAutoCompleteRequest {
            query: self.query,
            format: self.format,
            pretty: self.pretty,
            include_extras: self.include_extras,
        })
    }
}

/// Request type for calling the `/cards/random` Scryfall endpoint
///
/// Returns a single random [`Card`] object
///
/// Setting the `query` parameter supports the same fulltext search and will filter the pool of
/// cards before returning a random entry
#[derive(Debug, Clone)]
pub struct RandomCardRequest {
    query: Option<String>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl RandomCardRequest {
    /// Constructs a builder for a `RandomCardRequest`
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

/// Builder for constructing a [`RandomCardRequest`]
#[derive(Debug, Default)]
pub struct RandomCardRequestBuilder {
    query: Option<String>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl RandomCardRequestBuilder {
    /// Constructs a new `RandomCardRequestBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the query parameter to filter card results before returning a random one
    pub fn query(mut self, query: impl AsRef<str>) -> Self {
        self.query = Some(query.as_ref().to_string());
        self
    }

    /// Sets the [`DataFormat`] for the response
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    /// * [`DataFormat::Text`]
    /// * [`DataFormat::Image`]
    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    /// Sets the card face to return when using the `image` data format
    pub fn face(mut self, face: impl AsRef<str>) -> Self {
        self.face = Some(face.as_ref().to_string());
        self
    }

    /// Sets the image version to return when using the `image` data format
    pub fn image_version(mut self, version: ImageVersion) -> Self {
        self.version = Some(version);
        self
    }

    /// Sets the flag for prettifying the JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Builds the [`RandomCardRequest`]
    pub fn build(self) -> Result<RandomCardRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json | DataFormat::Text | DataFormat::Image)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
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

/// Request type for calling the `/cards/collection` Scryfall endpoint
///
/// This is a POST request that accepts an array of [`CardIdentifier`] and returns a [`List`] of 75
/// card references
#[derive(Debug, Clone)]
pub struct CardCollectionRequest {
    identifiers: Vec<CardIdentifier>,
    pretty: Option<bool>,
}

impl CardCollectionRequest {
    /// Construct a builder for a `CardCollectionRequest`
    pub fn builder() -> CardCollectionRequestBuilder {
        CardCollectionRequestBuilder::default()
    }
}

impl ScryfallRequest for CardCollectionRequest {
    type Response = List<Card>;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?
            .join("/cards/collection")
            .map_err(ScryfallApiError::UrlParse)?;

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

/// Builder for constructing a [`CardCollectionRequest`]
#[derive(Debug, Default)]
pub struct CardCollectionRequestBuilder {
    identifiers: Vec<CardIdentifier>,
    pretty: Option<bool>,
}

impl CardCollectionRequestBuilder {
    /// Constructs a new `CardCollectionRequestBuilder`
    pub fn new(identifiers: Vec<CardIdentifier>) -> Self {
        Self {
            identifiers,
            ..Default::default()
        }
    }

    /// Sets the array of Card identifiers for the request
    pub fn identifiers(mut self, identifiers: Vec<CardIdentifier>) -> Self {
        self.identifiers = identifiers;
        self
    }

    /// Sets the flag for prettifying the JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Builds the [`CardCollectionRequest`]
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

/// Request type for the `/cards/:code/:number(/:lang)` Scryfall endpoint
///
/// Returns a signle card with the given set `code` and collector `number`.
/// Also supports optional `lang` field to retrieve a non-English version of the card
#[derive(Debug, Clone)]
pub struct CardBySetAndIdRequest {
    set_code: String,
    collector_number: String,
    language: Option<Language>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl CardBySetAndIdRequest {
    /// Construct a new builder for a `CardBySetAndIdRequest`
    pub fn builder() -> CardBySetAndIdRequestBuilder {
        CardBySetAndIdRequestBuilder::default()
    }
}

impl ScryfallRequest for CardBySetAndIdRequest {
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

/// Builder for constructing a [`CardBySetAndIdRequest`]
#[derive(Debug, Default)]
pub struct CardBySetAndIdRequestBuilder {
    set_code: String,
    collector_number: String,
    language: Option<Language>,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl CardBySetAndIdRequestBuilder {
    /// Constructs a new `CardBySetAndIdRequestBuilder`
    pub fn new(set_code: impl AsRef<str>, collector_number: impl AsRef<str>) -> Self {
        Self {
            set_code: set_code.as_ref().to_string(),
            collector_number: collector_number.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Sets the 3-5 letter `set_code` field to select the Set the card belongs to
    pub fn set_code(mut self, set_code: impl AsRef<str>) -> Self {
        self.set_code = set_code.as_ref().to_string();
        self
    }

    /// Sets the `collector_number` field to retrieve the card from the Set
    pub fn collector_number(mut self, collector_number: impl AsRef<str>) -> Self {
        self.collector_number = collector_number.as_ref().to_string();
        self
    }

    /// Sets the [`Language`] field to retrieve the non-English version of the Card
    pub fn language(mut self, lang: Language) -> Self {
        self.language = Some(lang);
        self
    }

    /// Sets the [`DataFormat`] for the response
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    /// * [`DataFormat::Text`]
    /// * [`DataFormat::Image`]
    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    /// Sets the card face to return when using the `image` data format
    pub fn face(mut self, face: impl AsRef<str>) -> Self {
        self.face = Some(face.as_ref().to_string());
        self
    }

    /// Sets the image version to return when using the `image` data format
    pub fn image_version(mut self, img_version: ImageVersion) -> Self {
        self.version = Some(img_version);
        self
    }

    /// Sets the flag for prettifying the JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Builds the [`CardBySetAndIdRequest`]
    pub fn build(self) -> Result<CardBySetAndIdRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json | DataFormat::Text | DataFormat::Image)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
        }

        Ok(CardBySetAndIdRequest {
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

/// ID type to determine what endpoint to call when calling `/cards/:card_id/:id` Scryfall endpoint
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CardId {
    /// Use the `/cards/:id` endpoint
    Card(Uuid),

    /// Use the `/cards/multiverse/:id` endpoint
    Multiverse(u32),

    /// Use the `/cards/mtgo/:id` endpoint
    Mtgo(u32),

    /// Use the `/cards/arena/:id` endpoint
    Arena(u32),

    /// Use the `/cards/tcgplayer/:id` endpoint
    TcgPlayer(u32),

    /// Use the `/cards/cardmarket/:id` endpoint
    CardMarket(u32),
}

impl CardId {
    /// Determines the subpath of the URL to construct for the `/cards/:card_id/:id` endpoint
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

/// Request type for calling the `/cards/:card_id/:id` endpoint
///
/// The [`CardId`] is used to construct the URL that is used for the request and appends the inner
/// ID value to the end
///
/// E.g. A [`CardId::Mtgo`] would construct `/cards/mtgo/:id` URL
#[derive(Debug, Clone)]
pub struct CardFromIdRequest {
    id: CardId,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl CardFromIdRequest {
    /// Constructs the builder for a `CardFromIdRequest`
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

/// Builder for constructing a [`CardFromIdRequest`]
#[derive(Debug, Default)]
pub struct CardFromIdRequestBuilder {
    id: CardId,
    format: Option<DataFormat>,
    face: Option<String>,
    version: Option<ImageVersion>,
    pretty: Option<bool>,
}

impl CardFromIdRequestBuilder {
    /// Constructs a new `CardFromIdRequestBuilder`
    pub fn new(id: CardId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Sets the [`CardId`] for the request
    pub fn id(mut self, id: CardId) -> Self {
        self.id = id;
        self
    }

    /// Sets the [`DataFormat`] for the response
    ///
    /// Supports:
    /// * [`DataFormat::Json`]
    /// * [`DataFormat::Text`]
    /// * [`DataFormat::Image`]
    pub fn data_format(mut self, fmt: DataFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    /// Sets the card face to return when using the `image` data format
    pub fn face(mut self, face: impl AsRef<str>) -> Self {
        self.face = Some(face.as_ref().to_string());
        self
    }

    /// Sets the image version to return when using the `image` data format
    pub fn image_version(mut self, img_version: ImageVersion) -> Self {
        self.version = Some(img_version);
        self
    }

    /// Sets the flag for prettifying the JSON output
    pub fn pretty(mut self, flag: bool) -> Self {
        self.pretty = Some(flag);
        self
    }

    /// Builds the [`CardFromIdRequest`]
    pub fn build(self) -> Result<CardFromIdRequest, ScryfallApiError> {
        if let Some(fmt) = self.format
            && !matches!(fmt, DataFormat::Json | DataFormat::Text | DataFormat::Image)
        {
            return Err(ScryfallApiError::InvalidDataFormat(fmt));
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
mod url_tests {
    use super::*;

    fn ident(id_str: &str) -> CardIdentifier {
        CardIdentifier {
            id: Some(Uuid::from_str(id_str).expect("valid Scryfall card uuid")),
            mtgo_id: None,
            multiverse_id: None,
            oracle_id: None,
            illustration_id: None,
            name: None,
            set: None,
            collector_number: None,
        }
    }

    #[test]
    fn card_search_minimal() {
        let req = CardSearchRequest::builder()
            .query("austere command")
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/search?q=austere%20command"
        );
    }

    #[test]
    fn card_search_full() {
        let req = CardSearchRequest::builder()
            .query("foo bar")
            .unique_mode(UniqueMode::Prints)
            .sort_order(SortOrder::Set)
            .sort_direction(SortDirection::Desc)
            .include_extras(true)
            .include_multilingual(false)
            .include_variations(true)
            .page(3)
            .data_format(DataFormat::Json)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/search?q=foo%20bar&unique=prints&order=set&dir=desc&include_extras=true&include_multilingual=false&include_variations=true&page=3&format=json&pretty=true"
        );
    }

    #[test]
    fn card_search_csv_format_allowed() {
        let req = CardSearchRequest::builder()
            .query("c:r")
            .data_format(DataFormat::Csv)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/search?q=c:r&format=csv"
        );
    }

    #[test]
    fn card_search_rejects_image_format() {
        let err = CardSearchRequest::builder()
            .query("c:r")
            .data_format(DataFormat::Image)
            .build()
            .expect_err("Image is not a valid format for /cards/search");
        assert!(matches!(
            err,
            ScryfallApiError::InvalidDataFormat(DataFormat::Image)
        ));
    }

    #[test]
    fn card_search_rejects_text_format() {
        let err = CardSearchRequest::builder()
            .query("c:r")
            .data_format(DataFormat::Text)
            .build()
            .expect_err("Text is not a valid format for /cards/search");
        assert!(matches!(
            err,
            ScryfallApiError::InvalidDataFormat(DataFormat::Text)
        ));
    }

    #[test]
    fn named_card_exact_minimal() {
        let req = NamedCardRequest::builder()
            .exact_name("austere command")
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/named?exact=austere+command"
        );
    }

    #[test]
    fn named_card_fuzzy_full() {
        let req = NamedCardRequest::builder()
            .fuzzy_search("aust com")
            .set_code("mh2")
            .data_format(DataFormat::Image)
            .face("back")
            .image_version(ImageVersion::ArtCrop)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/named?fuzzy=aust+com&set=mh2&format=image&face=back&version=art_crop&pretty=true"
        );
    }

    #[test]
    fn named_card_missing_exact_and_fuzzy_errors() {
        let err = NamedCardRequest::builder()
            .build()
            .expect_err("either exact or fuzzy is required");
        match err {
            ScryfallApiError::ExpectedFieldsOneOf(fields) => {
                assert_eq!(fields, vec!["exact".to_string(), "fuzzy".to_string()]);
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn named_card_rejects_csv_format() {
        let err = NamedCardRequest::builder()
            .exact_name("a")
            .data_format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /cards/named");
        assert!(matches!(
            err,
            ScryfallApiError::InvalidDataFormat(DataFormat::Csv)
        ));
    }

    #[test]
    fn autocomplete_minimal() {
        let req = CardAutoCompleteRequest::builder()
            .query("aus")
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/autocomplete?q=aus"
        );
    }

    #[test]
    fn autocomplete_full() {
        let req = CardAutoCompleteRequest::builder()
            .query("aus")
            .format(DataFormat::Json)
            .pretty(true)
            .include_extras(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/autocomplete?q=aus&format=json&pretty=true&include_extras=true"
        );
    }

    #[test]
    fn autocomplete_rejects_csv_format() {
        let err = CardAutoCompleteRequest::builder()
            .query("aus")
            .format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /cards/autocomplete");
        assert!(matches!(
            err,
            ScryfallApiError::InvalidDataFormat(DataFormat::Csv)
        ));
    }

    #[test]
    fn random_card_minimal() {
        let req = RandomCardRequest::builder().build().expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/random");
    }

    #[test]
    fn random_card_full() {
        let req = RandomCardRequest::builder()
            .query("t:dragon")
            .face("back")
            .image_version(ImageVersion::Normal)
            .data_format(DataFormat::Image)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/random?q=t%3Adragon&face=back&version=normal&format=image&pretty=true"
        );
    }

    #[test]
    fn random_card_rejects_csv_format() {
        let err = RandomCardRequest::builder()
            .data_format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /cards/random");
        assert!(matches!(
            err,
            ScryfallApiError::InvalidDataFormat(DataFormat::Csv)
        ));
    }

    #[test]
    fn card_collection_url_pretty() {
        let req = CardCollectionRequest::builder()
            .identifiers(vec![ident("56ebc372-aabd-4174-a943-c7bf59e5028d")])
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/collection?pretty=true"
        );
    }

    #[test]
    fn card_collection_url_default() {
        let req = CardCollectionRequest::builder()
            .identifiers(vec![ident("56ebc372-aabd-4174-a943-c7bf59e5028d")])
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/collection");
    }

    #[test]
    fn card_collection_body_passes_through_identifiers() {
        let ids = vec![
            ident("56ebc372-aabd-4174-a943-c7bf59e5028d"),
            ident("f2b9983e-20d4-4d12-9e2c-ec6d9a345787"),
        ];
        let req = CardCollectionRequest::builder()
            .identifiers(ids.clone())
            .build()
            .expect("valid request");
        let body = req.body();
        assert_eq!(body.len(), 2);
        assert_eq!(body[0].id, ids[0].id);
        assert_eq!(body[1].id, ids[1].id);
    }

    #[test]
    fn card_collection_rejects_empty_identifiers() {
        let err = CardCollectionRequest::builder()
            .identifiers(vec![])
            .build()
            .expect_err("identifiers cannot be empty");
        assert!(matches!(err, ScryfallApiError::InvalidData(_)));
    }

    #[test]
    fn card_by_set_and_id_no_lang() {
        let req = CardBySetAndIdRequest::builder()
            .set_code("mh2")
            .collector_number("42")
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/mh2/42");
    }

    #[test]
    fn card_by_set_and_id_with_japanese_lang() {
        let req = CardBySetAndIdRequest::builder()
            .set_code("mh2")
            .collector_number("42")
            .language(Language::Japanese)
            .data_format(DataFormat::Image)
            .face("back")
            .image_version(ImageVersion::Png)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/mh2/42/ja?format=image&face=back&version=png&pretty=true"
        );
    }

    #[test]
    fn card_by_set_and_id_with_spanish_lang() {
        let req = CardBySetAndIdRequest::builder()
            .set_code("neo")
            .collector_number("100")
            .language(Language::Spanish)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/neo/100/es");
    }

    #[test]
    fn card_by_set_and_id_with_phyrexian_lang() {
        let req = CardBySetAndIdRequest::builder()
            .set_code("one")
            .collector_number("100")
            .language(Language::Phyrexian)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/one/100/ph");
    }

    #[test]
    fn card_by_set_and_id_rejects_csv_format() {
        let err = CardBySetAndIdRequest::builder()
            .set_code("mh2")
            .collector_number("42")
            .data_format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /cards/:code/:number");
        assert!(matches!(
            err,
            ScryfallApiError::InvalidDataFormat(DataFormat::Csv)
        ));
    }

    #[test]
    fn card_from_id_card_uuid() {
        let id = Uuid::from_str("56ebc372-aabd-4174-a943-c7bf59e5028d")
            .expect("valid Scryfall card uuid");
        let req = CardFromIdRequest::builder()
            .id(CardId::Card(id))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/56ebc372-aabd-4174-a943-c7bf59e5028d"
        );
    }

    #[test]
    fn card_from_id_mtgo() {
        let req = CardFromIdRequest::builder()
            .id(CardId::Mtgo(123))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/mtgo/123");
    }

    #[test]
    fn card_from_id_multiverse() {
        let req = CardFromIdRequest::builder()
            .id(CardId::Multiverse(456))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/multiverse/456"
        );
    }

    #[test]
    fn card_from_id_arena() {
        let req = CardFromIdRequest::builder()
            .id(CardId::Arena(789))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/arena/789");
    }

    #[test]
    fn card_from_id_tcgplayer() {
        let req = CardFromIdRequest::builder()
            .id(CardId::TcgPlayer(12))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/tcgplayer/12");
    }

    #[test]
    fn card_from_id_cardmarket() {
        let req = CardFromIdRequest::builder()
            .id(CardId::CardMarket(34))
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(url.as_str(), "https://api.scryfall.com/cards/cardmarket/34");
    }

    #[test]
    fn card_from_id_default_card_uuid() {
        let req = CardFromIdRequest::builder().build().expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/56ebc372-aabd-4174-a943-c7bf59e5028d"
        );
    }

    #[test]
    fn card_from_id_with_query_params() {
        let req = CardFromIdRequest::builder()
            .id(CardId::Mtgo(123))
            .data_format(DataFormat::Image)
            .face("back")
            .image_version(ImageVersion::BorderCrop)
            .pretty(true)
            .build()
            .expect("valid request");
        let url = req.to_url().expect("valid url");
        assert_eq!(
            url.as_str(),
            "https://api.scryfall.com/cards/mtgo/123?format=image&face=back&version=border_crop&pretty=true"
        );
    }

    #[test]
    fn card_from_id_rejects_csv_format() {
        let err = CardFromIdRequest::builder()
            .id(CardId::Mtgo(123))
            .data_format(DataFormat::Csv)
            .build()
            .expect_err("Csv is not a valid format for /cards/:id");
        assert!(matches!(
            err,
            ScryfallApiError::InvalidDataFormat(DataFormat::Csv)
        ));
    }
}
