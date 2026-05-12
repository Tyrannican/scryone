use super::types::*;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Set {
    pub object: String,
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub set_type: SetType,
    pub digital: bool,
    pub foil_only: bool,
    pub nonfoil_only: bool,
    pub scryfall_uri: Url,
    pub uri: Url,
    pub icon_svg_uri: Url,
    pub search_uri: Url,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtgo_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arena_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcgplayer_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_set_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_size: Option<u32>,
}
