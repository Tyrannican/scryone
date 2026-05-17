//! Documentation relating to the [`ManaCost`] object in Scryfall
//!
//! This is only applicable when calling the `/symbology/parse-mana` endpoint in Scryfall
//!
//! Scryfall understands most community shorthand for manacosts (such as `2WW`).
//! Symbols can also be out of order, lowercase, or have multiple color costs (e.g. `2{g}2` for
//! `4{G}`)
//!
//! More detailed information can be found here:
//! <https://scryfall.com/docs/api/card-symbols/parse-mana>
use serde::{Deserialize, Serialize};

use crate::objects::Color;

/// Representation of a "cost" for a Magic card
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ManaCost {
    /// Content type for the object, always `mana_cost`
    pub object: String,

    /// The normalised cost with correctly-ordered and wrapped mana symbols
    pub cost: String,

    /// The mana value. Unsets can have fractional costs
    pub cmc: f32,

    /// The colors of a given cost
    pub colors: Vec<Color>,

    /// True if the cost is colorless
    pub colorless: bool,

    /// True if the cost is mono-colored
    pub monocolored: bool,

    /// True if the cost is multicolored
    pub multicolored: bool,
}
