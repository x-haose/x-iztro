use serde::{Deserialize, Serialize};

use crate::data::stars::StarKey;
use crate::data::types::*;

/// 一颗已安放的星耀。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    /// 星耀标识
    pub key: StarKey,
    /// 星耀名称（按排盘语言翻译）
    pub name: String,
    /// 星耀类型
    pub star_type: StarType,
    /// 作用范围（本命或某运限层级）
    pub scope: Scope,
    /// 亮度；无亮度表的星耀为 None
    pub brightness: Option<Brightness>,
    /// 生年四化；非四化星为 None
    pub mutagen: Option<Mutagen>,
}
