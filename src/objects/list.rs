//! Documentation relating to the [`List`] object in Scryfall
//!
//! Represents a requested sequence of other objects (Cards, Sets, etc).
//!
//! Lists may be paginated, and also include information about issues raised when generating the
//! list
//!
//! More information can be found here: <https://scryfall.com/docs/api/lists>

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List<T> {
    /// Content type for the object, always `list`
    pub object: String,

    /// Array of requested `T` objects, in order
    pub data: Vec<T>,

    /// True if this list is paginated and there is more data beyond the current page
    pub has_more: bool,

    /// Full API URL to the next page, if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<Url>,

    /// Total number of cards across all pages if the list is of `Card` objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cards: Option<u32>,

    /// Warnings generated when generating this list, if applicable
    /// These are non-fatal issues that indicate the List is not complete due to malformed
    /// information provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
