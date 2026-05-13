//! Documentation relating to the [`BulkData`] object in Scryfall
//!
//! More detailed information can be found here: <https://scryfall.com/docs/api/bulk-data>
use super::types::BulkDataType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

/// Data relating to the Bulk Data requested from Scryfall
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BulkData {
    /// Content type of the object, always `bulk_data`
    pub object: String,

    /// Unique ID for this bulk item
    pub id: Uuid,

    /// Scryfall API URI for this file
    pub uri: Url,

    /// Type of bulk data requested
    #[serde(rename = "type")]
    pub bulk_type: BulkDataType,

    /// Name of the file
    pub name: String,

    /// Description for the file
    pub description: String,

    /// URI that hosts the file for fetching
    pub download_uri: Url,

    /// Timestamp for when the file was last updated
    pub updated_at: Timestamp,

    /// Size of the file in bytes
    pub size: usize,

    /// MIME-type of the file
    pub content_type: String,

    /// Encoding that is used to transmit the file when it is downloaded
    pub content_encoding: String,
}
