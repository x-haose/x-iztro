use serde::{Deserialize, Serialize};

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::astrolabe::Astrolabe;
use crate::models::star::Star;

/// 单个运限层级（大限/流年/流月/流日/流时）的数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoroscopeItem {
    /// 该运限所在宫位索引（0-11，寅宫为 0）
    pub index: usize,
    /// 层级显示名（大限/童限/流年/流月/流日/流时，按输出语言翻译）
    pub name: String,
    /// 层级的语言无关标识；大限层未起运时为 `Childhood`（童限），
    /// 与 `Decadal` 是不同的解盘语义，靠它而非译文区分
    pub name_key: HoroscopeName,
    /// 该运限天干
    pub heavenly_stem: HeavenlyStem,
    /// 该运限地支
    pub earthly_branch: EarthlyBranch,
    /// 以该运限所在宫位为命宫推排的十二宫名（按宫位索引排列）
    pub palace_names: Vec<Palace>,
    /// 该运限天干引发的四化星 [禄, 权, 科, 忌]
    pub mutagen: Vec<StarKey>,
    /// 流耀在十二宫的分布（无流耀的层级为 None）
    pub stars: Option<Vec<Vec<Star>>>,
}

/// 小限数据。
///
/// 通用运限字段经 `Deref` 直接可读（`age.index`、`age.heavenly_stem`），
/// `base` 字段本身也保留，供需要整个 [`HoroscopeItem`] 时取用。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeItem {
    /// 通用运限字段
    #[serde(flatten)]
    pub base: HoroscopeItem,
    /// 虚岁
    pub nominal_age: u32,
}

impl std::ops::Deref for AgeItem {
    type Target = HoroscopeItem;

    fn deref(&self) -> &HoroscopeItem {
        &self.base
    }
}

/// 流年十二神：按目标年支起的岁前十二神与将前十二神。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyDecStar {
    /// 将前十二神（按宫位索引排列）
    pub jiangqian12: Vec<StarKey>,
    /// 岁前十二神（按宫位索引排列）
    pub suiqian12: Vec<StarKey>,
}

/// 流年数据（含流年十二神）。
///
/// 通用运限字段经 `Deref` 直接可读（`yearly.mutagen`、`yearly.palace_names`），
/// `base` 字段本身也保留，供需要整个 [`HoroscopeItem`] 时取用。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyItem {
    /// 通用运限字段
    #[serde(flatten)]
    pub base: HoroscopeItem,
    /// 流年十二神
    pub yearly_dec_star: YearlyDecStar,
}

impl std::ops::Deref for YearlyItem {
    type Target = HoroscopeItem;

    fn deref(&self) -> &HoroscopeItem {
        &self.base
    }
}

/// 一次运限查询的完整结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoroscopeData {
    /// 目标阳历日期（与查询入参一致）
    pub solar_date: String,
    /// 目标农历日期中文表示
    pub lunar_date: String,
    /// 大限（未起运时为童限）
    pub decadal: HoroscopeItem,
    /// 小限
    pub age: AgeItem,
    /// 流年
    pub yearly: YearlyItem,
    /// 流月
    pub monthly: HoroscopeItem,
    /// 流日
    pub daily: HoroscopeItem,
    /// 流时
    pub hourly: HoroscopeItem,
}

/// 一次运限查询的结果连同发起它的星盘。
///
/// 由 [`Astrolabe::horoscope`] 返回。解引用即得到 [`HoroscopeData`] 本身，
/// 而宫位与流耀查询不必再把星盘传进来 —— 运限本来就依附于某一张盘。
pub struct HoroscopeRef<'a> {
    pub(crate) data: HoroscopeData,
    pub(crate) astrolabe: &'a Astrolabe,
}

impl<'a> std::ops::Deref for HoroscopeRef<'a> {
    type Target = HoroscopeData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a> HoroscopeRef<'a> {
    /// 运限数据本身。
    pub fn data(&self) -> &HoroscopeData {
        &self.data
    }

    /// 发起这次运限查询的星盘。
    pub fn astrolabe(&self) -> &'a Astrolabe {
        self.astrolabe
    }

    /// 取出运限数据，脱离星盘。
    pub fn into_data(self) -> HoroscopeData {
        self.data
    }
}
