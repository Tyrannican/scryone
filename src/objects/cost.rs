use serde::{Deserialize, Serialize};

use crate::objects::Color;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ManaCost {
    pub object: String,
    pub cost: String,
    pub cmc: f32,
    pub colors: Vec<Color>,
    pub colorless: bool,
    pub monocolored: bool,
    pub multicolored: bool,
}
