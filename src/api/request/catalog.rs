use crate::objects::catalog::Catalog;

use super::{BASE_URL, ScryfallApiError, ScryfallRequest};
use url::Url;

#[derive(Debug, Clone)]
pub struct CatalogRequest {
    catalog_type: CatalogType,
}

impl CatalogRequest {
    pub fn builder() -> CatalogRequestBuilder {
        CatalogRequestBuilder::default()
    }
}

impl ScryfallRequest for CatalogRequest {
    type Response = Catalog;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let path = format!("/catalog/{}", self.catalog_type.to_string());
        url = url.join(&path)?;

        Ok(url)
    }
}

#[derive(Default)]
pub struct CatalogRequestBuilder {
    catalog_type: CatalogType,
}

impl CatalogRequestBuilder {
    pub fn new(catalog_type: CatalogType) -> Self {
        Self { catalog_type }
    }

    pub fn catalog_type(mut self, catalog_type: CatalogType) -> Self {
        self.catalog_type = catalog_type;
        self
    }

    pub fn build(self) -> Result<CatalogRequest, ScryfallApiError> {
        Ok(CatalogRequest {
            catalog_type: self.catalog_type,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum CatalogType {
    #[default]
    CardNames,
    ArtistNames,
    WordBank,
    SuperTypes,
    CardTypes,
    ArtifactTypes,
    BattleTypes,
    CreatureTypes,
    EnchantmentTypes,
    LandTypes,
    PlaneswalkerTypes,
    SpellTypes,
    Powers,
    Toughnesses,
    Loyalties,
    KeywordAbilities,
    KeywordActions,
    AbilityWords,
    FlavorWords,
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
