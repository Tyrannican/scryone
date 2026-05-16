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

pub trait ScryfallRequest {
    type Response: DeserializeOwned;

    fn to_url(&self) -> Result<Url, ScryfallApiError>;
}

pub trait ScryfallPostRequest {
    type Body: Serialize;
    fn body(&self) -> &Self::Body;
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum UniqueMode {
    #[default]
    Cards,
    Art,
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

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum SortOrder {
    #[default]
    Name,
    Set,
    Released,
    Rarity,
    Color,
    Usd,
    Tix,
    Eur,
    Cmc,
    Power,
    Toughness,
    EdhRec,
    Penny,
    Artist,
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

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum SortDirection {
    #[default]
    Auto,
    Asc,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataFormat {
    Json,
    Csv,
    Text,
    Image,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageVersion {
    Small,
    Normal,
    Large,
    Png,
    ArtCrop,
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
