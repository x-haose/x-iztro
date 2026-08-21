//! 盘面视图：把本命盘与运限合成盘统一成规则可见的十二宫。

use crate::data::stars::StarKey;
use crate::data::types::{Brightness, EarthlyBranch, Mutagen, Palace, Scope, StarType};
use crate::models::astrolabe::Astrolabe;
use crate::models::horoscope::HoroscopeData;
use crate::models::palace::PalaceData;
use crate::models::star::Star;
use crate::utils::fix_index;

use super::{BrightnessSource, PatternConfig, PatternHit, PatternKey, StarAt};

/// 煞星六颗：火铃羊陀空劫（页面善荫朝纲一节所列，化忌另按四化判）。
pub const SHA6: [StarKey; 6] = [
    StarKey::HuoxingMin,
    StarKey::LingxingMin,
    StarKey::QingyangMin,
    StarKey::TuoluoMin,
    StarKey::DikongMin,
    StarKey::DijieMin,
];

/// 四煞：羊陀火铃。
pub const SHA4: [StarKey; 4] = [
    StarKey::QingyangMin,
    StarKey::TuoluoMin,
    StarKey::HuoxingMin,
    StarKey::LingxingMin,
];

/// 六吉星：辅弼昌曲魁钺（三吉化另按四化判）。
pub const JI6: [StarKey; 6] = [
    StarKey::ZuofuMin,
    StarKey::YoubiMin,
    StarKey::WenchangMin,
    StarKey::WenquMin,
    StarKey::TiankuiMin,
    StarKey::TianyueMin,
];

/// 空亡星：旬空、空亡、截路（默认派）、截空（中州派）。
pub const KONG_WANG: [StarKey; 4] = [
    StarKey::Xunkong,
    StarKey::Kongwang,
    StarKey::Jielu,
    StarKey::Jiekong,
];

/// 空曜：天空、地空、截空、旬空（页面禄衰马困一节原话）。
pub const KONG_YAO: [StarKey; 4] = [
    StarKey::Tiankong,
    StarKey::DikongMin,
    StarKey::Jiekong,
    StarKey::Xunkong,
];

/// 杀破狼。
pub const SPL: [StarKey; 3] = [StarKey::QishaMaj, StarKey::PojunMaj, StarKey::TanlangMaj];

/// 机月同梁。
pub const JYTL: [StarKey; 4] = [
    StarKey::TianjiMaj,
    StarKey::TaiyinMaj,
    StarKey::TiantongMaj,
    StarKey::TianliangMaj,
];

/// 庙旺。
pub const BRIGHT: [Brightness; 2] = [Brightness::Miao, Brightness::Wang];

/// 落陷（含「不」：页面石中隐玉一节把太阳戌宫「不」也称反背，只有含「不」才自洽）。
pub const DARK: [Brightness; 2] = [Brightness::Xian, Brightness::Bu];

/// 本命辅星与其运限流曜的对应：运限视角下流曜等同该辅星参与判定。
///
/// 判定只认大限（运）与流年（流）两层的流曜——流月/日/时的流曜（yue/ri/shi）
/// 不参与格局；全量对照表见 [`crate::astro::horoscope::natal_counterpart_of_flow_star`]。
fn flow_counterparts(key: StarKey) -> &'static [StarKey] {
    use StarKey::*;
    match key {
        LucunMin => &[Yunlu, Liulu],
        TianmaMin => &[Yunma, Liuma],
        WenchangMin => &[Yunchang, Liuchang],
        WenquMin => &[Yunqu, Liuqu],
        QingyangMin => &[Yunyang, Liuyang],
        TuoluoMin => &[Yuntuo, Liutuo],
        TiankuiMin => &[Yunkui, Liukui],
        TianyueMin => &[Yunyue, Liuyue],
        Hongluan => &[Yunluan, Liuluan],
        Tianxi => &[Yunxi, Liuxi],
        _ => &[],
    }
}

/// 视图里的一个宫位：本命宫位数据加上该视角下的流曜。
pub struct ViewPalace<'a> {
    /// 本命宫位
    pub natal: &'a PalaceData,
    /// 该视角下落在此宫的流曜（本命视角为空）
    pub flow: Vec<&'a Star>,
}

/// 规则可见的盘面：十二宫、命宫身宫位置、该视角的四化与口径开关。
pub struct ChartView<'a> {
    astrolabe: &'a Astrolabe,
    horoscope: Option<&'a HoroscopeData>,
    scope: Scope,
    palaces: Vec<ViewPalace<'a>>,
    /// 该视角下的宫名（本命为本命宫名，运限为以运限命宫重排的宫名）
    names: [Palace; 12],
    soul: usize,
    body: Option<usize>,
    /// 运限视角的四化 [禄, 权, 科, 忌]；本命视角为 `None`（读星耀自带的生年四化）
    scope_mutagen: Option<[StarKey; 4]>,
    config: PatternConfig,
}

impl<'a> ChartView<'a> {
    /// 本命视图。
    pub fn natal(astrolabe: &'a Astrolabe, config: &PatternConfig) -> Self {
        let palaces = astrolabe
            .palaces
            .iter()
            .map(|p| ViewPalace {
                natal: p,
                flow: Vec::new(),
            })
            .collect();
        let mut names = [Palace::Soul; 12];
        for (i, p) in astrolabe.palaces.iter().enumerate() {
            names[i] = p.name;
        }
        ChartView {
            astrolabe,
            horoscope: None,
            scope: Scope::Origin,
            palaces,
            names,
            soul: astrolabe
                .palaces
                .iter()
                .position(|p| p.name == Palace::Soul)
                .unwrap_or(0),
            body: astrolabe.palaces.iter().position(|p| p.is_body_palace),
            scope_mutagen: None,
            config: *config,
        }
    }

    /// 运限视图：以该层命宫为命宫，合并该层流曜与四化。
    ///
    /// 流曜取法：大限视角只含运曜，流年及以下视角合并运曜与流曜
    /// （流月/流日/流时本身无流曜）。iztro 的 `hasHoroscopeStars` 对任何运限层
    /// 都无条件合并运曜与流曜，此处大限视角不并流曜是有意收窄：
    /// 大限层的成格证据不随查询时点落在哪个流年而变。
    /// 运限视角没有身宫概念，`body()` 为 `None`。
    pub fn at(
        astrolabe: &'a Astrolabe,
        horoscope: &'a HoroscopeData,
        scope: Scope,
        config: &PatternConfig,
    ) -> Self {
        let Some(item) = horoscope.scope_item(scope) else {
            return Self::natal(astrolabe, config);
        };
        let layers: Vec<&Vec<Vec<Star>>> = match scope {
            Scope::Decadal => horoscope.decadal.stars.iter().collect(),
            _ => horoscope
                .decadal
                .stars
                .iter()
                .chain(horoscope.yearly.base.stars.iter())
                .collect(),
        };
        let palaces = astrolabe
            .palaces
            .iter()
            .enumerate()
            .map(|(i, p)| ViewPalace {
                natal: p,
                flow: layers
                    .iter()
                    .filter_map(|l| l.get(i))
                    .flat_map(|stars| stars.iter())
                    .collect(),
            })
            .collect();
        let mut names = [Palace::Soul; 12];
        for (i, n) in item.palace_names.iter().take(12).enumerate() {
            names[i] = *n;
        }
        let mut mutagen = [StarKey::ZiweiMaj; 4];
        for (i, m) in item.mutagen.iter().take(4).enumerate() {
            mutagen[i] = *m;
        }
        ChartView {
            astrolabe,
            horoscope: Some(horoscope),
            scope,
            palaces,
            names,
            soul: fix_index(item.index as i32, 12),
            body: None,
            scope_mutagen: Some(mutagen),
            config: *config,
        }
    }

    // ---------- 基本访问 ----------

    /// 所属星盘。
    pub fn astrolabe(&self) -> &'a Astrolabe {
        self.astrolabe
    }

    /// 运限数据（本命视图为 `None`）。
    pub fn horoscope(&self) -> Option<&'a HoroscopeData> {
        self.horoscope
    }

    /// 视角。
    pub fn scope(&self) -> Scope {
        self.scope
    }

    /// 口径开关。
    pub fn config(&self) -> &PatternConfig {
        &self.config
    }

    /// 命宫索引。
    pub fn soul(&self) -> usize {
        self.soul
    }

    /// 身宫索引（运限视角无身宫）。
    pub fn body(&self) -> Option<usize> {
        self.body
    }

    /// 命宫与身宫（去重）：「身命」类格局的候选宫。
    pub fn soul_and_body(&self) -> Vec<usize> {
        match self.body {
            Some(b) if b != self.soul => vec![self.soul, b],
            _ => vec![self.soul],
        }
    }

    /// 视角下某宫名所在的索引。
    pub fn index_of(&self, name: Palace) -> usize {
        self.names
            .iter()
            .position(|n| *n == name)
            .unwrap_or(self.soul)
    }

    /// 视角下的宫名。
    pub fn name_of(&self, i: usize) -> Palace {
        self.names[fix_index(i as i32, 12)]
    }

    /// 宫位地支。
    pub fn branch(&self, i: usize) -> EarthlyBranch {
        self.palace(i).natal.earthly_branch
    }

    /// 宫位视图，索引对 12 取模。
    pub fn palace(&self, i: usize) -> &ViewPalace<'a> {
        &self.palaces[fix_index(i as i32, 12)]
    }

    // ---------- 位置关系 ----------

    /// 对宫。
    pub fn opposite(&self, i: usize) -> usize {
        fix_index(i as i32 + 6, 12)
    }

    /// 三合宫（对命宫即官禄、财帛）。
    pub fn trine(&self, i: usize) -> [usize; 2] {
        [fix_index(i as i32 + 4, 12), fix_index(i as i32 + 8, 12)]
    }

    /// 三方四正：本宫、对宫、两三合宫。
    pub fn surround(&self, i: usize) -> [usize; 4] {
        [
            fix_index(i as i32, 12),
            self.opposite(i),
            fix_index(i as i32 + 4, 12),
            fix_index(i as i32 + 8, 12),
        ]
    }

    /// 前一宫（对命宫即兄弟宫）。
    pub fn prev(&self, i: usize) -> usize {
        fix_index(i as i32 - 1, 12)
    }

    /// 后一宫（对命宫即父母宫）。
    pub fn next(&self, i: usize) -> usize {
        fix_index(i as i32 + 1, 12)
    }

    /// 暗合宫（六合位）：子丑、寅亥、卯戌、辰酉、巳申、午未。
    pub fn hidden(&self, i: usize) -> usize {
        let b = self.branch(i).index() as i32;
        let partner = fix_index(1 - b, 12); // 子(0)↔丑(1)、寅(2)↔亥(11)、卯↔戌、辰↔酉、巳↔申、午↔未
        (0..12)
            .find(|j| self.branch(*j).index() == partner)
            .unwrap_or(i)
    }

    /// 「A、B 夹 i」：一侧有 A 集合任一星、另一侧有 B 集合任一星。
    /// 返回 `(A 所在宫, B 所在宫)`。
    pub fn jia(&self, i: usize, a: &[StarKey], b: &[StarKey]) -> Option<(usize, usize)> {
        let (p, n) = (self.prev(i), self.next(i));
        if self.has_any(p, a) && self.has_any(n, b) {
            Some((p, n))
        } else if self.has_any(p, b) && self.has_any(n, a) {
            Some((n, p))
        } else {
            None
        }
    }

    // ---------- 星耀查询 ----------

    /// 宫内全部星（本命主辅杂 + 视角流曜）。
    pub fn stars(&self, i: usize) -> impl Iterator<Item = &'a Star> + '_ {
        let p = self.palace(i);
        p.natal
            .major_stars
            .iter()
            .chain(p.natal.minor_stars.iter())
            .chain(p.natal.adjective_stars.iter())
            .chain(p.flow.iter().copied())
    }

    /// 宫内是否有该星；运限视角下对应流曜亦算（受 `flow_stars` 开关控制）。
    pub fn has(&self, i: usize, key: StarKey) -> bool {
        self.find(i, key).is_some()
    }

    /// 宫内是否有集合任一星。
    pub fn has_any(&self, i: usize, keys: &[StarKey]) -> bool {
        keys.iter().any(|k| self.has(i, *k))
    }

    /// 宫内是否有集合全部星。
    pub fn has_all(&self, i: usize, keys: &[StarKey]) -> bool {
        keys.iter().all(|k| self.has(i, *k))
    }

    /// 三方四正是否见该星。
    pub fn in_surround(&self, i: usize, key: StarKey) -> bool {
        self.surround(i).iter().any(|p| self.has(*p, key))
    }

    /// 三方四正是否见集合任一星。
    pub fn surround_has_any(&self, i: usize, keys: &[StarKey]) -> bool {
        self.surround(i).iter().any(|p| self.has_any(*p, keys))
    }

    /// 三方四正是否集齐集合全部星（可分散在不同宫）。
    pub fn surround_has_all(&self, i: usize, keys: &[StarKey]) -> bool {
        keys.iter().all(|k| self.in_surround(i, *k))
    }

    /// 在宫内找到该星（含等价流曜），返回星本身。
    pub fn find(&self, i: usize, key: StarKey) -> Option<&'a Star> {
        let flow = if self.config.flow_stars {
            flow_counterparts(key)
        } else {
            &[]
        };
        self.stars(i)
            .find(|s| s.key == key || flow.contains(&s.key))
    }

    /// 在三方四正里找到该星，返回 `(宫索引, 星)`。
    pub fn find_in_surround(&self, i: usize, key: StarKey) -> Option<(usize, &'a Star)> {
        self.surround(i)
            .into_iter()
            .find_map(|p| self.find(p, key).map(|s| (p, s)))
    }

    /// 全盘找到该星，返回 `(宫索引, 星)`。
    pub fn locate(&self, key: StarKey) -> Option<(usize, &'a Star)> {
        (0..12).find_map(|p| self.find(p, key).map(|s| (p, s)))
    }

    /// 宫内主星（`StarType::Major`），不借。
    pub fn majors(&self, i: usize) -> Vec<&'a Star> {
        self.palace(i)
            .natal
            .major_stars
            .iter()
            .filter(|s| s.star_type == StarType::Major)
            .collect()
    }

    /// 宫内主星，空宫且 `borrow` 开启时借对宫主星。返回 `(星, 是否借来)`。
    pub fn majors_borrowed(&self, i: usize) -> Vec<(&'a Star, bool)> {
        let own = self.majors(i);
        if own.is_empty() && self.config.borrow {
            self.majors(self.opposite(i))
                .into_iter()
                .map(|s| (s, true))
                .collect()
        } else {
            own.into_iter().map(|s| (s, false)).collect()
        }
    }

    /// 宫内该主星（空宫且 `borrow` 开启时借对宫），返回 `(星的实际落宫, 星)`：
    /// 借来的星实际落在对宫，证据里记的是它真正待的宫。
    pub fn find_major_at(&self, i: usize, key: StarKey) -> Option<(usize, &'a Star)> {
        let own = fix_index(i as i32, 12);
        self.majors_borrowed(i)
            .into_iter()
            .find(|(s, _)| s.key == key)
            .map(|(s, borrowed)| (if borrowed { self.opposite(i) } else { own }, s))
    }

    /// 宫内是否有该主星（含借宫）。
    pub fn has_major(&self, i: usize, key: StarKey) -> bool {
        self.find_major_at(i, key).is_some()
    }

    /// 是否空宫（无主星）。
    pub fn is_empty(&self, i: usize) -> bool {
        self.majors(i).is_empty()
    }

    /// 某星是否独坐（宫内唯一主星）。
    pub fn alone(&self, i: usize, key: StarKey) -> bool {
        let m = self.majors(i);
        m.len() == 1 && m[0].key == key
    }

    // ---------- 亮度与四化 ----------

    /// 宫内该星是否处于集合亮度之一。
    pub fn brightness_in(&self, i: usize, key: StarKey, set: &[Brightness]) -> bool {
        self.find(i, key).is_some_and(|s| s.with_brightness(set))
    }

    /// 宫内太阳/太阴是否「明」（庙旺）：按 [`BrightnessSource`] 决定依据。
    pub fn sun_moon_bright(&self, i: usize, key: StarKey) -> bool {
        match self.config.brightness_source {
            BrightnessSource::Table => self.brightness_in(i, key, &BRIGHT),
            BrightnessSource::Positional => {
                self.has(i, key) && positional(key, self.branch(i)) == Some(true)
            }
        }
    }

    /// 宫内太阳/太阴是否「暗」（落陷）：按 [`BrightnessSource`] 决定依据。
    pub fn sun_moon_dark(&self, i: usize, key: StarKey) -> bool {
        match self.config.brightness_source {
            BrightnessSource::Table => self.brightness_in(i, key, &DARK),
            BrightnessSource::Positional => {
                self.has(i, key) && positional(key, self.branch(i)) == Some(false)
            }
        }
    }

    /// 该视角下某星的四化：本命读生年四化，运限读该层四化。
    pub fn mutagen_of(&self, star: &Star) -> Option<Mutagen> {
        match self.scope_mutagen {
            None => star.mutagen,
            Some(m) => [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji]
                .into_iter()
                .zip(m)
                .find(|(_, k)| *k == star.key)
                .map(|(mu, _)| mu),
        }
    }

    /// 宫内带该四化的星。
    pub fn mutagen_star(&self, i: usize, m: Mutagen) -> Option<&'a Star> {
        self.stars(i).find(|s| self.mutagen_of(s) == Some(m))
    }

    /// 宫内是否有该四化。
    pub fn has_mutagen(&self, i: usize, m: Mutagen) -> bool {
        self.mutagen_star(i, m).is_some()
    }

    /// 三方四正是否见该四化，返回 `(宫索引, 星)`。
    pub fn find_mutagen_in_surround(&self, i: usize, m: Mutagen) -> Option<(usize, &'a Star)> {
        self.surround(i)
            .into_iter()
            .find_map(|p| self.mutagen_star(p, m).map(|s| (p, s)))
    }

    /// 三方四正无煞：火铃羊陀空劫与化忌一颗都不见。
    pub fn no_sha(&self, i: usize) -> bool {
        !self.surround_has_any(i, &SHA6) && self.find_mutagen_in_surround(i, Mutagen::Ji).is_none()
    }

    /// 宫的博士十二神。
    pub fn boshi12(&self, i: usize) -> StarKey {
        self.palace(i).natal.boshi12
    }

    // ---------- 命中构造 ----------

    /// 把星与落宫记成证据。
    pub fn star_at(&self, i: usize, star: &Star) -> StarAt {
        StarAt {
            star: star.key,
            palace: fix_index(i as i32, 12),
            brightness: star.brightness,
            mutagen: self.mutagen_of(star),
        }
    }

    /// 由 `(宫, 星)` 列表构造一次命中。
    pub fn hit(&self, key: PatternKey, palace: usize, stars: Vec<(usize, &Star)>) -> PatternHit {
        PatternHit {
            key,
            scope: self.scope,
            palace: fix_index(palace as i32, 12),
            variant: None,
            broken: false,
            stars: stars.into_iter().map(|(p, s)| self.star_at(p, s)).collect(),
        }
    }
}

/// 太阳/太阴的传统位置明暗：`Some(true)` 明、`Some(false)` 暗、`None` 中性或非日月。
fn positional(key: StarKey, branch: EarthlyBranch) -> Option<bool> {
    use EarthlyBranch::*;
    match key {
        StarKey::TaiyangMaj => match branch {
            Yin | Mao | Chen | Si | Wu => Some(true),
            You | Xu | Hai | Zi | Chou => Some(false),
            _ => None,
        },
        StarKey::TaiyinMaj => match branch {
            You | Xu | Hai | Zi | Chou => Some(true),
            Mao | Chen | Si | Wu | Wei => Some(false),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod counterpart_tests {
    use super::*;
    use crate::astro::horoscope::natal_counterpart_of_flow_star;

    /// 判定用的运/流对应关系必须是全量对照表的子集，两表不许分叉。
    #[test]
    fn flow_counterparts_agree_with_full_table() {
        use StarKey::*;
        for natal in [
            LucunMin,
            TianmaMin,
            WenchangMin,
            WenquMin,
            QingyangMin,
            TuoluoMin,
            TiankuiMin,
            TianyueMin,
            Hongluan,
            Tianxi,
        ] {
            for flow in flow_counterparts(natal) {
                assert_eq!(
                    natal_counterpart_of_flow_star(*flow),
                    Some(natal),
                    "{flow:?} 在两张对照表中的本命对应不一致"
                );
            }
        }
    }
}
