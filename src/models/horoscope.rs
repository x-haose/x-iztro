use serde::{Deserialize, Serialize};

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::star::Star;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoroscopeItem {
    pub index: usize,
    pub name: String,
    pub heavenly_stem: HeavenlyStem,
    pub earthly_branch: EarthlyBranch,
    pub palace_names: Vec<Palace>,
    pub mutagen: Vec<StarKey>,
    pub stars: Option<Vec<Vec<Star>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeItem {
    #[serde(flatten)]
    pub base: HoroscopeItem,
    pub nominal_age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoroscopeData {
    pub solar_date: String,
    pub lunar_date: String,
    pub decadal: HoroscopeItem,
    pub age: AgeItem,
    pub yearly: HoroscopeItem,
    pub monthly: HoroscopeItem,
    pub daily: HoroscopeItem,
    pub hourly: HoroscopeItem,
}
