//! Documentation relating to the [`Ruling`] object in Scryfall
//!
//! Rulings represent Oracle rulings, Wizards of the Coast set release notes, or Scryfall notes for
//! a particular card
//!
//! If two cards have the same name, they will have the same set of rulings objects. If a card has
//! rulings, it usually has more than one.
//!
//! Rulings with a `scryfall` source have been added by the Scryfall team, either to provide
//! additional context for the card, or explain how the card works in an unofficial format (such as
//! Duel Commander)
//!
//! More information can be found here: <https://scryfall.com/docs/api/rulings>

use super::types::RulingSource;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ruling {
    /// Content type for the object, always `ruling`
    pub object: String,

    /// Oracle ID for the card this ruling is associated with
    pub oracle_id: Uuid,

    /// The source of this ruling, either `wotc` or `scryfall`
    pub source: RulingSource,

    /// The dat when the ruling or note was published
    pub published_at: Date,

    /// The text of the ruling
    pub comment: String,
}
