//! Documentation relating to the [`ScryfallError`] object in Scryfall
//!
//! An Error object represents a failure to find information or understand the input that is
//! provided to the API.
//! Error objects are always transmitteed with the appropriate `4XX` or `5XX` status codes
//!
//! More information can be found here: <https://scryfall.com/docs/api/errors>

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScryfallError {
    /// Content type for the object, always `error`
    pub object: String,

    /// HTTP status code
    pub status: u16,

    /// Details explaining the error
    pub details: String,

    /// Additional context for the main error
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,

    /// If the input generated non-failure warnings, these are added here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}
