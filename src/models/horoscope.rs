use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::error::IztroError;
use crate::models::astrolabe::Astrolabe;
use crate::models::star::Star;

/// 单个运限层级（大限/流年/流月/流日/流时）的数据。
#[derive(Debug, Clone)]
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
    /// 该运限天干引发的四化星 [禄, 权, 科, 忌]（四化表四槽）
    pub mutagen: Vec<StarKey>,
    /// 该运限天干四槽之外的附加四化星，按禄、权、科、忌分组；默认四化表下四组皆空
    pub extra_mutagen: [Vec<StarKey>; 4],
    /// 流耀在十二宫的分布（无流耀的层级为 None）
    pub stars: Option<Vec<Vec<Star>>>,
}

impl HoroscopeItem {
    /// 该运限天干化出的全部（星, 四化）：按禄、权、科、忌，每类先四槽之星、后附加星。
    pub fn mutagens(&self) -> impl Iterator<Item = (StarKey, Mutagen)> + '_ {
        crate::data::types::mutagen_pairs(&self.mutagen, &self.extra_mutagen)
    }
}

/// 农历月在流月列表中的分段。
///
/// 闰月在 `fix_leap` 打开时按前后半月拆成两段分别起运限，其余情形整月一段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthPart {
    /// 整月一段（非闰月，或闰月但不拆分）
    Normal,
    /// 闰月前半段（初一至十五）
    First,
    /// 闰月后半段（十六至月末）
    Second,
}

crate::data::keyed::keyed_enum!(MonthPart, "month part" {
    Normal => "normal",
    First => "first",
    Second => "second",
});

/// 大限定位方式：按起运先后的序号，或该限所在的本命宫名。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecadalTarget {
    /// 起运先后序号（0 为第一个大限）
    Ordinal(usize),
    /// 该大限所在的本命宫名
    Name(Palace),
}

impl From<usize> for DecadalTarget {
    fn from(ordinal: usize) -> Self {
        DecadalTarget::Ordinal(ordinal)
    }
}

impl From<Palace> for DecadalTarget {
    fn from(name: Palace) -> Self {
        DecadalTarget::Name(name)
    }
}

/// 大限列表项：通用运限字段之外，带该限所在的本命宫名与虚岁、年份区间。
///
/// 通用字段经 `Deref` 直接可读（`d.heavenly_stem`、`d.palace_names`）。
#[derive(Debug, Clone)]
pub struct DecadalHoroscope {
    /// 通用运限字段
    pub base: HoroscopeItem,
    /// 该大限所在的本命宫名
    pub palace_name: Palace,
    /// 起止虚岁（含两端）
    pub age_range: (u32, u32),
    /// 起止农历年份（含两端）
    pub year_range: (i64, i64),
}

impl std::ops::Deref for DecadalHoroscope {
    type Target = HoroscopeItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// 流年列表项：通用运限字段之外，带该年对应的虚岁与农历年份。
#[derive(Debug, Clone)]
pub struct YearlyHoroscope {
    /// 通用运限字段
    pub base: HoroscopeItem,
    /// 该流年对应的虚岁
    pub age: u32,
    /// 农历年份
    pub year: i64,
}

impl std::ops::Deref for YearlyHoroscope {
    type Target = HoroscopeItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// 流月列表项：在流年字段之外，带农历月份、闰月标记与该段的农历日区间。
#[derive(Debug, Clone)]
pub struct MonthlyHoroscope {
    /// 通用运限字段
    pub base: HoroscopeItem,
    /// 该流月对应的虚岁
    pub age: u32,
    /// 农历年份
    pub year: i64,
    /// 农历月份（正月为 1，闰月与否见 `is_leap_month`）
    pub month: u32,
    /// 是否闰月
    pub is_leap_month: bool,
    /// 整月一段，还是闰月的前后半段
    pub part: MonthPart,
    /// 该段覆盖的农历日区间（含两端）
    pub day_range: (u32, u32),
}

impl std::ops::Deref for MonthlyHoroscope {
    type Target = HoroscopeItem;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// 小限数据。
///
/// 通用运限字段经 `Deref` 直接可读（`age.index`、`age.heavenly_stem`），
/// `base` 字段本身也保留，供需要整个 [`HoroscopeItem`] 时取用。
#[derive(Debug, Clone)]
pub struct AgeItem {
    /// 通用运限字段
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct YearlyItem {
    /// 通用运限字段
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
#[derive(Debug, Clone)]
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

/// 一次运限查询的结果连同发起它的星盘与查询精度。
///
/// 由 [`Astrolabe::horoscope`]、[`Astrolabe::horoscope_of_year`]、[`Astrolabe::horoscope_of_month`] 返回。
/// 解引用即得到 [`HoroscopeData`] 本身，而宫位与流耀查询不必再把星盘传进来 —— 运限本来就依附于某一张盘。
pub struct HoroscopeRef<'a> {
    /// 运限数据；各层恒齐全，比 `precision` 细的层级属于代表日
    pub(crate) data: HoroscopeData,
    /// 发起查询的星盘
    pub(crate) astrolabe: &'a Astrolabe,
    /// 查询精度，取值见 [`HoroscopeRef::precision`]
    pub(crate) precision: Scope,
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

    /// 取出运限数据，脱离星盘与查询精度。
    pub fn into_data(self) -> HoroscopeData {
        self.data
    }

    /// 查询精度：目标实际指定到的最细层级。按农历年取（[`Astrolabe::horoscope_of_year`]）为
    /// [`Scope::Yearly`]，按农历年月取（[`Astrolabe::horoscope_of_month`]）为 [`Scope::Monthly`]，
    /// 按日期取（[`Astrolabe::horoscope`]，给不给时辰都一样）为 [`Scope::Hourly`]。
    ///
    /// 数据里比它细的层级是内核替调用方挑的代表日（六月初一、月初一或交节次日）的，不是查询的答案：
    /// 文本（`to_text`）默认只写到这一层，宫位查询、格局与文本的层级参数比它细时报错。
    pub fn precision(&self) -> Scope {
        self.precision
    }

    /// 校验 `scope` 不比查询精度细；`param` 是报错时指明的入参名。
    ///
    /// # Errors
    /// `scope` 比 [`Self::precision`] 细时返回 [`IztroError::InvalidArgument`]。
    pub(crate) fn check_within_precision(
        &self,
        scope: Scope,
        param: &str,
    ) -> Result<(), IztroError> {
        if scope.depth() > self.precision.depth() {
            return Err(IztroError::InvalidArgument(format!(
                "invalid {param} '{}': the horoscope target only specifies layers down to '{}'",
                scope.as_key(),
                self.precision.as_key()
            )));
        }
        Ok(())
    }
}
