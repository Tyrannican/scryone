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

#[cfg(test)]
mod bulk_data_object_tests {
    use super::*;
    use crate::api::ScryfallApiError;

    #[test]
    fn deserialises_bulk_object() -> Result<(), ScryfallApiError> {
        let raw = r#"    {
      "object": "bulk_data",
      "id": "27bf3214-1271-490b-bdfe-c0be6c23d02e",
      "type": "oracle_cards",
      "updated_at": "2026-05-23T09:02:54.512+00:00",
      "uri": "https://api.scryfall.com/bulk-data/27bf3214-1271-490b-bdfe-c0be6c23d02e",
      "name": "Oracle Cards",
      "description": "A JSON file containing one Scryfall card object for each Oracle ID on Scryfall. The chosen sets for the cards are an attempt to return the most up-to-date recognizable version of the card.",
      "size": 173116018,
      "download_uri": "https://data.scryfall.io/oracle-cards/oracle-cards-20260523090254.json",
      "content_type": "application/json",
      "content_encoding": "gzip"
    }"#;

        let types = vec![
            "oracle_cards",
            "unique_artwork",
            "default_cards",
            "all_cards",
            "rulings",
        ];

        for t in types {
            let raw = raw.replace(
                "\"type\": \"oracle_cards\",",
                &format!("\"type\": \"{t}\","),
            );

            let obj = serde_json::from_str::<BulkData>(&raw)
                .map_err(|_| ScryfallApiError::InvalidData(raw.to_string()));

            assert!(obj.is_ok());
        }

        Ok(())
    }
}
