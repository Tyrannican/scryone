use super::types::Color;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardSymbol {
    pub object: String,
    pub symbol: String,
    pub english: String,
    pub transposable: bool,
    pub represents_mana: bool,
    pub appears_in_mana_costs: bool,
    pub funny: bool,
    pub colors: Vec<Color>,
    pub hybrid: bool,
    pub phyrexian: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loose_variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana_value: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatherer_alternatives: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg_uri: Option<Url>,
}
