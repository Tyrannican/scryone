//! Documentation relating to the [`Set`] object in Scryfall
//!
//! Set objects represent a group of related Magic cards. All Card objects on Scryfall belone to
//! exactly one set.
//!
//! Official sets always have a three-letter set code, such as `zen`
//!
//! More information can be found here: <https://scryfall.com/docs/api/sets>

use super::types::*;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

/// Representation of a collection of Magic cards
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Set {
    /// Content type for the object, always `set`
    pub object: String,

    /// Unique ID for this set on Scryfall (immutable)
    pub id: Uuid,

    /// Unique three to six letter code for this set
    pub code: String,

    /// The English name for the set
    pub name: String,

    /// Classification for the set (see [`SetType`])
    pub set_type: SetType,

    /// True if this set was only released in a digital format
    pub digital: bool,

    /// True if this set contains only foil cards
    pub foil_only: bool,

    /// True if this set contains only non-foil cards
    pub nonfoil_only: bool,

    /// Link to this set's permapage on Scryfall's website
    pub scryfall_uri: Url,

    /// Link to this set object on Scryfall's API
    pub uri: Url,

    /// URI to an SVG file for this set's icon on Scryfall's CDN
    pub icon_svg_uri: Url,

    /// Scryfall API URI that can be requested to begin paginating over cards in this set
    pub search_uri: Url,

    /// Unique code for this set on MTGO, which may differ from the regular code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtgo_code: Option<String>,

    /// Unique code for this set on Arena, which may differ from the regular code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arena_code: Option<String>,

    /// This set's ID on [TCGPlayer's API](https://docs.tcgplayer.com/docs), also known as the
    /// `groupId`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcgplayer_id: Option<u32>,

    /// The date this set was released or the first card that was printed in the set (-8 GMT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<Date>,

    /// The block code for this set, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_code: Option<String>,

    /// The set code for the parent set, if any. `promo` and `token` sets often have parent sets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_set_code: Option<String>,

    /// The denominator for this set's printed collector numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_size: Option<u32>,
}
