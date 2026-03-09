use serde::{Deserialize, Serialize};

use crate::data::stars::StarKey;
use crate::data::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub key: StarKey,
    pub name: String,
    pub star_type: StarType,
    pub scope: Scope,
    pub brightness: Option<Brightness>,
    pub mutagen: Option<Mutagen>,
}
