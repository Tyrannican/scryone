//! Documentation relating to the [`Catalog`] object in Scryfall
//!
//! More detailed information can be found here: <https://scryfall.com/docs/api/catalogs>
use serde::{Deserialize, Serialize};
use url::Url;

/// A collection of Magic datapoints (words, card values, etc)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Catalog {
    /// Content type for the object, always `catalog`
    pub object: String,

    /// Link to the current catalog on Scryfall's API
    pub uri: Url,

    /// Number of items in the `data` array
    pub total_values: u32,

    /// Array of data points
    pub data: Vec<String>,
}
