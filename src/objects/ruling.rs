use super::types::RulingSource;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ruling {
    pub object: String,
    pub oracle_id: Uuid,
    pub source: RulingSource,
    pub published_at: Date,
    pub comment: String,
}
