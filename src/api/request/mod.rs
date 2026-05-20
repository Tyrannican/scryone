//! Request types and builders for constructing Scryfall API Requests

use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::api::error::ScryfallApiError;
pub mod bulk;
pub mod card;
pub mod catalog;
pub mod migration;
pub mod ruling;
pub mod set;
pub mod symbology;

pub use bulk::*;
pub use card::*;
pub use catalog::*;
pub use migration::*;
pub use ruling::*;
pub use set::*;
pub use symbology::*;

pub(crate) const BASE_URL: &str = "https://api.scryfall.com";

macro_rules! add_query_pair {
    ($url:expr, $field:expr, $key:literal) => {
        if let Some(val) = $field {
            $url.query_pairs_mut().append_pair($key, &val.to_string());
        }
    };
}

pub(crate) use add_query_pair;

/// Scryfall GET Requests
pub trait ScryfallRequest {
    type Response: DeserializeOwned;

    /// Generates a URL for a request with appropriate query parameters
    fn to_url(&self) -> Result<Url, ScryfallApiError>;
}

/// Scryfall POST Requests
pub trait ScryfallPostRequest {
    type Body: Serialize;

    /// Generates a Body for the implementing Request
    fn body(&self) -> &Self::Body;
}

/// Paginated requests
pub trait PaginatedRequest {
    type Item;

    /// Retrieve the data from the current page and URL for the next page, if any
    fn parts(self) -> (Vec<Self::Item>, Option<Url>);
}

/// Type of Unique Cards to return when requesting uniques
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum UniqueMode {
    /// Unique cards
    #[default]
    Cards,

    /// Unique Art
    Art,

    /// Unique print variations
    Prints,
}

impl std::fmt::Display for UniqueMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cards => write!(f, "cards"),
            Self::Art => write!(f, "art"),
            Self::Prints => write!(f, "prints"),
        }
    }
}

/// Order in which to sort a collection of cards
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum SortOrder {
    /// Sort by name
    #[default]
    Name,

    /// Sort by Magic Set
    Set,

    /// Sort by released date
    Released,

    /// Sort by card rarity
    Rarity,

    /// Sort by card color
    Color,

    /// Sort by price in USD
    Usd,

    /// Sort by price in MTGO Tickets
    Tix,

    /// Sort by price in EUR
    Eur,

    /// Sort by Mana Value (converted mana cost)
    Cmc,

    /// Sort by card power
    Power,

    /// Sort by card toughness
    Toughness,

    /// Sort by EDHREC rank
    EdhRec,

    /// Sort by Penny Dreadful rank
    Penny,

    /// Sort by artist name
    Artist,

    /// Sort by card review
    Review,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::Set => write!(f, "set"),
            Self::Released => write!(f, "released"),
            Self::Rarity => write!(f, "rarity"),
            Self::Color => write!(f, "color"),
            Self::Usd => write!(f, "usd"),
            Self::Tix => write!(f, "tix"),
            Self::Eur => write!(f, "eur"),
            Self::Cmc => write!(f, "cmc"),
            Self::Power => write!(f, "power"),
            Self::Toughness => write!(f, "toughness"),
            Self::EdhRec => write!(f, "edhrec"),
            Self::Penny => write!(f, "penny"),
            Self::Artist => write!(f, "artist"),
            Self::Review => write!(f, "review"),
        }
    }
}

/// Direction in which to sort (if applicable)
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum SortDirection {
    /// Let Scryfall decide
    #[default]
    Auto,

    /// Sort in Ascending order
    Asc,

    /// Sort in Descending order
    Desc,
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

/// Format in which to return the Responses from the API
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataFormat {
    /// JSON format
    Json,

    /// Comma Separated Value format
    Csv,

    /// Raw text format
    Text,

    /// Image format
    Image,

    /// Raw file bytes
    File,
}

impl std::fmt::Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Csv => write!(f, "csv"),
            Self::Text => write!(f, "text"),
            Self::Image => write!(f, "image"),
            Self::File => write!(f, "file"),
        }
    }
}

/// Version of the image requested
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageVersion {
    /// Small print size
    Small,

    /// Normal print size
    Normal,

    /// Large print size
    Large,

    /// Card as a PNG
    Png,

    /// Just the Art of the card
    ArtCrop,

    /// The card with the border cropped
    BorderCrop,
}

impl std::fmt::Display for ImageVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small => write!(f, "small"),
            Self::Normal => write!(f, "normal"),
            Self::Large => write!(f, "large"),
            Self::Png => write!(f, "png"),
            Self::ArtCrop => write!(f, "art_crop"),
            Self::BorderCrop => write!(f, "border_crop"),
        }
    }
}
