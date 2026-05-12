//! Documentation relating to `Card` objects in Scryfall
//!
//! More detailed information can be found here: https://scryfall.com/docs/api/cards
use jiff::civil::Date;
use serde::{
    Deserialize, Serialize,
    de::{Deserializer, IntoDeserializer},
};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

use super::types::*;

pub type UriMap = HashMap<String, Url>;

/// # Card Objects
///
/// Card objects represent individual Magic: The Gathering cards that players could obtain and add
/// to their collection (with a few minor exceptions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    // Core properties
    pub id: Uuid,
    pub object: String,
    pub lang: Language,
    pub layout: Layout,
    pub prints_search_uri: Url,
    pub rulings_uri: Url,
    pub scryfall_uri: Url,
    pub uri: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arena_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtgo_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtgo_foil_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiverse_ids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcgplayer_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcgplayer_etched_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardmarket_id: Option<u32>,

    // Gameplay fields
    pub cmc: f32,
    pub name: String,
    pub keywords: Vec<String>,
    pub reserved: bool,
    pub type_line: String,
    pub legalities: Legality,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_parts: Option<Vec<RelatedCard>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_faces: Option<Vec<CardFace>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<String>,
    #[serde(
        default,
        deserialize_with = "parse_mana_cost_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub mana_cost: Option<Vec<CostSymbol>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penny_rank: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produced_mana: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<String>,

    // Print Fields
    pub booster: bool,
    pub border_color: BorderColor,
    pub collector_number: String,
    pub digital: bool,
    pub finishes: Vec<CardFinish>,
    pub frame: CardFrame,
    pub full_art: bool,
    pub games: Vec<GameType>,
    pub highres_image: bool,
    pub image_status: ImageStatus,
    pub oversized: bool,
    pub prices: PriceData,
    pub rarity: Rarity,
    pub related_uris: UriMap,
    pub released_at: Date,
    pub reprint: bool,
    pub scryfall_set_uri: Url,
    pub set_name: String,
    pub set_search_uri: Url,
    pub set_type: String,
    pub set_uri: Url,
    pub set: String,
    pub set_id: Uuid,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_back_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attraction_lights: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_warning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_effects: Option<Vec<FrameEffect>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustration_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uris: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_type_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_uris: Option<UriMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_of: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_stamp: Option<SecurityStamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,

    #[serde(
        rename = "preview.previewed_at",
        skip_serializing_if = "Option::is_none"
    )]
    pub previewed_at: Option<Date>,
    #[serde(rename = "preview.source_uri", skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<Url>,
    #[serde(rename = "preview.source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardFace {
    #[serde(deserialize_with = "parse_mana_cost")]
    pub mana_cost: Vec<CostSymbol>,
    pub name: String,
    pub object: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmc: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_indicator: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustration_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uris: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_type_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelatedCard {
    pub id: Uuid,
    pub object: String,
    pub component: RelatedCardRole,
    pub name: String,
    pub type_line: String,
    pub uri: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legality {
    pub standard: LegalStatus,
    pub future: LegalStatus,
    pub historic: LegalStatus,
    pub timeless: LegalStatus,
    pub gladiator: LegalStatus,
    pub pioneer: LegalStatus,
    pub modern: LegalStatus,
    pub legacy: LegalStatus,
    pub pauper: LegalStatus,
    pub vintage: LegalStatus,
    pub penny: LegalStatus,
    pub commander: LegalStatus,
    pub oathbreaker: LegalStatus,
    pub standardbrawl: LegalStatus,
    pub brawl: LegalStatus,
    pub alchemy: LegalStatus,
    pub paupercommander: LegalStatus,
    pub duel: LegalStatus,
    pub oldschool: LegalStatus,
    pub premodern: LegalStatus,
    pub predh: LegalStatus,
    pub tlr: LegalStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub png: Url,
    pub border_crop: Url,
    pub art_crop: Url,
    pub large: Url,
    pub normal: Url,
    pub small: Url,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PriceData {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub usd_etched: Option<String>,
    pub eur: Option<String>,
    pub eur_foil: Option<String>,
    pub eur_etched: Option<String>,
    pub tix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtgo_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiverse_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustration_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_number: Option<String>,
}

fn parse_mana_cost_opt<'de, D>(deserializer: D) -> Result<Option<Vec<CostSymbol>>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Option<String> = Option::deserialize(deserializer)?;
    let Some(raw) = raw.filter(|s| !s.is_empty()) else {
        return Ok(None);
    };

    if raw.contains("//") {
        return Ok(None);
    }

    raw.split_inclusive('}')
        .map(|c| CostSymbol::deserialize(c.into_deserializer()))
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

fn parse_mana_cost<'de, D>(deserializer: D) -> Result<Vec<CostSymbol>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Option<String> = Option::deserialize(deserializer)?;
    let Some(raw) = raw.filter(|s| !s.is_empty()) else {
        return Ok(Vec::new());
    };
    raw.split_inclusive('}')
        .map(|c| CostSymbol::deserialize(c.into_deserializer()))
        .collect::<Result<Vec<_>, _>>()
}
