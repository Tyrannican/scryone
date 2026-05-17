//! Documentation relating to the [`CardMigration`] object in Scryfall
//!
//! More information can be found here: <https://scryfall.com/docs/api/migrations>

use super::types::MigrationPlan;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

/// Card Migration strategy (Beta on Scryfall)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardMigration {
    /// Content type for the object, always `migration`
    pub object: String,

    /// Link to the current object on Scryfall's API
    pub uri: Url,

    /// The Migration's unique UUID
    pub id: Uuid,

    /// The date this migration was performed
    pub performed_at: Date,

    /// The `id` of the affected API Card object
    pub old_scryfall_id: Uuid,

    /// Indicator of the type of migration performed
    pub migration_strategy: MigrationPlan,

    /// Replacement `id` of the API Card object if this is a `merge`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_scryfall_id: Option<Uuid>,

    /// Note left by the Scryfall team about this migration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Additional context Scryfall has provided for the migration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}
