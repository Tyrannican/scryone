//! Request types and builders for Catalogs in Scryfall API
//!
//! More details can be found here: <https://scryfall.com/docs/api/catalogs>
use crate::objects::catalog::Catalog;

use super::{BASE_URL, ScryfallApiError, ScryfallRequest};
use url::Url;

/// Request type for calling the `/catalog/:catalog_type` Scryfall endpoint
#[derive(Debug, Clone)]
pub struct CatalogRequest {
    catalog_type: CatalogType,
}

impl CatalogRequest {
    /// Constructs a new builder for a `CatalogRequest`
    pub fn builder() -> CatalogRequestBuilder {
        CatalogRequestBuilder::default()
    }
}

impl ScryfallRequest for CatalogRequest {
    type Response = Catalog;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let path = format!("/catalog/{}", self.catalog_type);
        url = url.join(&path)?;

        Ok(url)
    }
}

/// Builder for constructing a [`CatalogRequest`]
#[derive(Debug, Default)]
pub struct CatalogRequestBuilder {
    catalog_type: CatalogType,
}

impl CatalogRequestBuilder {
    /// Constructs a new `CatalogRequestBuilder`
    pub fn new(catalog_type: CatalogType) -> Self {
        Self { catalog_type }
    }

    /// Sets the [`CatalogType`] for the request
    pub fn catalog_type(mut self, catalog_type: CatalogType) -> Self {
        self.catalog_type = catalog_type;
        self
    }

    /// Builds the [`CatalogRequest`]
    pub fn build(self) -> Result<CatalogRequest, ScryfallApiError> {
        Ok(CatalogRequest {
            catalog_type: self.catalog_type,
        })
    }
}

/// Catalog types that can be used in [`CatalogRequest`]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum CatalogType {
    /// Use the `/catalog/card-names` endpoint
    #[default]
    CardNames,
    /// Use the `/catalog/artist-names` endpoint
    ArtistNames,
    /// Use the `/catalog/word-bank` endpoint
    WordBank,
    /// Use the `/catalog/supertypes` endpoint
    SuperTypes,
    /// Use the `/catalog/card-types` endpoint
    CardTypes,
    /// Use the `/catalog/artifact-types` endpoint
    ArtifactTypes,
    /// Use the `/catalog/battle-types` endpoint
    BattleTypes,
    /// Use the `/catalog/creature-types` endpoint
    CreatureTypes,
    /// Use the `/catalog/enchantment-types` endpoint
    EnchantmentTypes,
    /// Use the `/catalog/land-types` endpoint
    LandTypes,
    /// Use the `/catalog/planeswalker-types` endpoint
    PlaneswalkerTypes,
    /// Use the `/catalog/spell-types` endpoint
    SpellTypes,
    /// Use the `/catalog/powers` endpoint
    Powers,
    /// Use the `/catalog/toughnesses` endpoint
    Toughnesses,
    /// Use the `/catalog/loyalties` endpoint
    Loyalties,
    /// Use the `/catalog/keyword-abilities` endpoint
    KeywordAbilities,
    /// Use the `/catalog/keyword-actions` endpoint
    KeywordActions,
    /// Use the `/catalog/ability-words` endpoint
    AbilityWords,
    /// Use the `/catalog/flavor-words` endpoint
    FlavorWords,
    /// Use the `/catalog/watermarks` endpoint
    Watermarks,
}

impl std::fmt::Display for CatalogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CardNames => write!(f, "card-names"),
            Self::ArtistNames => write!(f, "artist-names"),
            Self::WordBank => write!(f, "word-bank"),
            Self::SuperTypes => write!(f, "supertypes"),
            Self::CardTypes => write!(f, "card-types"),
            Self::ArtifactTypes => write!(f, "artifact-types"),
            Self::BattleTypes => write!(f, "battle-types"),
            Self::CreatureTypes => write!(f, "creature-types"),
            Self::EnchantmentTypes => write!(f, "enchantment-types"),
            Self::LandTypes => write!(f, "land-types"),
            Self::PlaneswalkerTypes => write!(f, "planeswalker-types"),
            Self::SpellTypes => write!(f, "spell-types"),
            Self::Powers => write!(f, "powers"),
            Self::Toughnesses => write!(f, "toughnesses"),
            Self::Loyalties => write!(f, "loyalties"),
            Self::KeywordAbilities => write!(f, "keyword-abilities"),
            Self::KeywordActions => write!(f, "keyword-actions"),
            Self::AbilityWords => write!(f, "ability-words"),
            Self::FlavorWords => write!(f, "flavor-words"),
            Self::Watermarks => write!(f, "watermarks"),
        }
    }
}
