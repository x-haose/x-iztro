use serde::{Deserialize, Serialize};

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::star::Star;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decadal {
    pub range: (u32, u32),
    pub heavenly_stem: HeavenlyStem,
    pub earthly_branch: EarthlyBranch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalaceData {
    pub index: usize,
    pub name: Palace,
    pub is_body_palace: bool,
    pub is_original_palace: bool,
    pub heavenly_stem: HeavenlyStem,
    pub earthly_branch: EarthlyBranch,
    pub major_stars: Vec<Star>,
    pub minor_stars: Vec<Star>,
    pub adjective_stars: Vec<Star>,
    pub changsheng12: StarKey,
    pub boshi12: StarKey,
    pub jiangqian12: StarKey,
    pub suiqian12: StarKey,
    pub decadal: Decadal,
    pub ages: Vec<u32>,
}
