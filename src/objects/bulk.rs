use super::types::BulkDataType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BulkData {
    pub object: String,
    pub id: Uuid,
    pub uri: Url,
    #[serde(rename = "type")]
    pub bulk_type: BulkDataType,
    pub name: String,
    pub description: String,
    pub download_uri: Url,
    pub updated_at: Timestamp,
    pub size: usize,
    pub content_type: String,
    pub content_encoding: String,
}
