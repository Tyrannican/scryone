use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Catalog {
    pub object: String,
    pub uri: Url,
    pub total_values: u32,
    pub data: Vec<String>,
}
