use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List<T> {
    pub object: String,
    pub data: Vec<T>,
    pub has_more: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cards: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
