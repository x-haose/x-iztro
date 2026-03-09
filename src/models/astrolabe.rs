use serde::{Deserialize, Serialize};

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::palace::PalaceData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Astrolabe {
    pub gender: Gender,
    pub solar_date: String,
    pub lunar_date: String,
    pub chinese_date: String,
    pub time: String,
    pub time_range: String,
    pub sign: String,
    pub zodiac: String,
    pub earthly_branch_of_soul_palace: EarthlyBranch,
    pub earthly_branch_of_body_palace: EarthlyBranch,
    pub soul: StarKey,
    pub body: StarKey,
    pub five_elements_class: FiveElementsClass,
    pub palaces: Vec<PalaceData>,
}
