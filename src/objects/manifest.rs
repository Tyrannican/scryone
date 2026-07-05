//! Documentation relating to [`CardManifest`] objects in Scryfall
//!
//! More detailed information can be found here: <https://scryfall.com/docs/api/cards/manifest>

use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::Language;

/// Efficient information about a given card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardManifest {
    /// A Unique ID for the card in Scryfall's database
    id: Uuid,

    /// The name of this card. If this card has multiple faces, this field will contain both names
    /// separated by `␣//␣`
    name: String,

    /// This Card's set code
    set_code: String,

    /// Language printing of the card
    lang: Language,

    /// When this card was created
    created_at: Option<Date>,

    /// When the data relating to this card was last updated
    data_updated_at: Option<Date>,

    /// When the image for this card was last updated
    image_updated_at: Option<Date>,
}

/// Sort order for a list of [`CardManifest`] objects
/// The direction is always descending
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CardManifestSortOrder {
    /// Sort card by their release date: Newest -> Oldest
    #[default]
    Released,

    /// Sort card by their last image update: Most recent -> Least recent
    #[serde(rename = "imageupdated")]
    ImageUpdated,
}

impl std::fmt::Display for CardManifestSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Released => write!(f, "released"),
            Self::ImageUpdated => write!(f, "imageupdated"),
        }
    }
}
