use super::types::MigrationPlan;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardMigration {
    pub object: String,
    pub uri: Url,
    pub id: Uuid,
    pub performed_at: Date,
    pub old_scryfall_id: Uuid,
    pub migration_strategy: MigrationPlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_scryfall_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}
