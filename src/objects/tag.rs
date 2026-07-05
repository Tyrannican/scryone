//! Documentation relating to [`Tag`] objects in Scryfall
//!
//! More detailed information can be found here: <https://scryfall.com/docs/api/tags>

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

/// There are two types of tags:
///
/// * Art Tags (type `illustration`): Describes what is depicted in a card's artwork (e.g.
///   creatures, settings, etc).
///
/// * Oracle Tags (type `oracle`): Describes the functional role of a card, such as removal, ramp,
///   or draw
///
/// These tags are both sourced from the community-maintained [Tagger Project](https://tagger.scryfall.com/).
///
/// Tags are available as Bulk Data files and are updated daily on Scryfall.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// A Content type for this object, always `tag`
    pub object: String,

    /// A unique and stable UUID for this tag
    pub id: Uuid,

    /// A URL-safe identifier for the tag e.g. `squirrel`. Slugs may change over time so are
    /// considered unstable for use as a reference
    pub slug: String,

    /// A human-readable name for the tag (e.g. `Squirrel`)
    pub label: String,

    /// A link to this Tag on the Tagger site.
    pub uri: Url,

    /// The tag type
    #[serde(rename = "type")]
    pub tag_type: TagType,

    /// An optional description of what this tag represents
    pub description: Option<String>,

    /// UUIDs of parent tags in the tag hierarchy within the bulk file
    pub parent_ids: Option<Vec<Uuid>>,

    /// UUIDs of child tags in the tag hierarchy within the bulk file
    pub child_ids: Option<Vec<Uuid>>,

    /// Alternative names the community uses for this tag
    pub aliases: Option<Vec<String>>,

    /// An array of [`Tagging`] objects associating this tag with specific cards
    pub taggings: Vec<Tagging>,
}

/// Type of Tag a [`Tag`] represents
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum TagType {
    /// Illustration Tagging
    Illustration,

    /// Oracle Tagging
    Oracle,
}

impl std::fmt::Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Illustration => write!(f, "illustration"),
            Self::Oracle => write!(f, "oracle"),
        }
    }
}

/// ID used to differentiate between `illustration` and `oracle` [`Tagging`] objects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaggingId {
    /// Tagging is an `illustration` tagging
    Illustration {
        /// Card Illustration ID
        illustration_id: Uuid,
    },

    /// Tagging is an `oracle` tagging
    Oracle {
        /// Card Oracle ID
        oracle_id: Uuid,
    },
}

/// Tagging which linkgs a tag to a specific card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tagging {
    /// ID of the card
    #[serde(flatten)]
    pub id: TaggingId,

    /// How prominently the tag applies to the card
    pub weight: TagWeighting,

    /// Optional note providing additional context to the specific taggin
    pub annotation: Option<String>,
}

/// The weight of a [`Tagging`] implying how strongly it applies to a specific Card
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "snake_case")]
pub enum TagWeighting {
    /// Subject is exemplary for the image or card text
    VeryStrong,

    /// Subject is a primary ofcus of the image or card text
    Strong,

    /// A normal tagging with no special weight applied
    Median,

    /// The subject is a minor detail or background element
    Weak,
}

impl std::fmt::Display for TagWeighting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VeryStrong => write!(f, "very_strong"),
            Self::Strong => write!(f, "strong"),
            Self::Median => write!(f, "median"),
            Self::Weak => write!(f, "weak"),
        }
    }
}
