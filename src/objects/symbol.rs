//! Documentation relating to the [`CardSymbol`] object in Scryfall
//!
//! Card Symbol objects represent an illustrated symbol that may appear in a card's mana cost or
//! Oracle text.
//!
//! Symbols are based on the notation used in the Comprehensive Rules for Magic:
//! <http://magic.wizards.com/en/game-info/gameplay/rules-and-formats/rules>
//!
//! More information can be found here: <https://scryfall.com/docs/api/card-symbols>

use super::types::{Color, CostSymbol};
use serde::{Deserialize, Serialize};
use url::Url;

/// Representation of a symbol that can appear in a card's mana cost
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardSymbol {
    /// Content type for this object, always `card_symbol`
    pub object: String,

    /// The plaintext (UTF-8) symbol. Often surrounded with curly braces `{}`
    pub symbol: CostSymbol,

    /// An English snipped that describes this symbol
    pub english: String,

    /// True if it's possible to write this symbol "backwards" (e.g. {P/U} and {U/P})
    /// Scryfall always returns the symbols in the correct order as they appear on a card
    pub transposable: bool,

    /// True if this is a mana symbol
    pub represents_mana: bool,

    /// True if this symbol appears in a mana cost on any Magic Card
    pub appears_in_mana_costs: bool,

    /// True if this symbol is only used on `funny` cards (e.g. Un-sets)
    pub funny: bool,

    /// Colors that this symbol represents
    pub colors: Vec<Color>,

    /// True if the symbol is a hybrid mana symbol (Note: Phyrexian symbols are not hybrid)
    pub hybrid: bool,

    /// True if this symbol is a Phyrexian mana symbol
    pub phyrexian: bool,

    /// Alternate version of this symbol, if it's possible to write it without curly braces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loose_variant: Option<String>,

    /// Decimal representing this symbol's mana value (a.k.a. converted mana cost)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana_value: Option<f32>,

    /// Plaintext version of this symbol that Gatherer uses on old cards to describe original
    /// printed text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gatherer_alternatives: Option<Vec<String>>,

    /// URI to an SVG image of this symbol on Scryfall's CDN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg_uri: Option<Url>,
}
