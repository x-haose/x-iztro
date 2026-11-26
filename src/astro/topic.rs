//! 按主题选宫：知识包主题（`topics.<slug>`）的宫位定义落到盘上某一层，连同选中宫相关的三类四化。
//!
//! 本命层按本命宫名定位；运限层（大限至流时）按该层重排的宫名定位，三方四正以该层的本宫起算。
//! `bodyPalace` / `originalPalace` 是本命盘上算出的标记（身宫随出生时辰、来因宫随生年天干），
//! 运限只轮换宫名、不重算这两个标记，所以在任何一层都落在本命的那一宫。
//! 主题文本（[`crate::text::topic_to_text`]、[`crate::text::horoscope_topic_to_text`]）与
//! 结构化结果（[`crate::Astrolabe::topic_palaces`]、[`crate::HoroscopeRef::topic_palaces`]）都由本模块的
//! `select` 选宫。

use crate::data::keyed::keyed_enum;
use crate::data::stars::{MUTAGEN, StarKey};
use crate::data::types::{HOROSCOPE_SCOPES, HoroscopeName, Mutagen, Palace, Scope};
use crate::error::IztroError;
use crate::knowledge::{KnowledgePack, TopicEntry};
use crate::models::astrolabe::{Astrolabe, PalaceTarget};
use crate::models::horoscope::{HoroscopeData, HoroscopeItem, HoroscopeRef};

/// 主题里的一组宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopicGroupKind {
    /// `surround` 的一宫：连同它在该层的三方四正
    Surround,
    /// `palaces` 的一宫：只取本宫
    Only,
}

keyed_enum!(TopicGroupKind, "topic group kind" {
    Surround => "surround",
    Only => "only",
});

/// 选中宫在主题里的角色。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopicRole {
    /// `surround` 组的本宫
    Target,
    /// `surround` 组的对宫
    Opposite,
    /// `surround` 组的财帛位（本宫起顺数第九宫）
    Wealth,
    /// `surround` 组的官禄位（本宫起顺数第五宫）
    Career,
    /// `palaces` 里只取本宫的宫
    Related,
}

keyed_enum!(TopicRole, "topic role" {
    Target => "target",
    Opposite => "opposite",
    Wealth => "wealth",
    Career => "career",
    Related => "related",
});

/// 主题定义里的一组宫落到盘上的位置。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicGroup {
    /// 组的种类
    pub kind: TopicGroupKind,
    /// 主题定义里的宫位标识：宫名标识（按所选层的宫名定位）、`bodyPalace` 或 `originalPalace`
    pub key: String,
    /// 本宫索引（0-11，寅宫为 0）
    pub target: usize,
    /// 对宫索引；`Only` 组为 `None`
    pub opposite: Option<usize>,
    /// 财帛位索引；`Only` 组为 `None`
    pub wealth: Option<usize>,
    /// 官禄位索引；`Only` 组为 `None`
    pub career: Option<usize>,
}

/// 一宫宫干飞出的一化。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flight {
    /// 四化
    pub mutagen: Mutagen,
    /// 宫干在该四化位上的星
    pub star: StarKey,
    /// 这颗星所在的宫位索引；星不在盘上（自定义四化表指向流耀一类）时为 `None`
    pub palace: Option<usize>,
}

/// 选中的一宫。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicPalace {
    /// 宫位索引
    pub index: usize,
    /// 角色，取它在 [`TopicSelection::groups`] 里首次出现的那一组
    pub role: TopicRole,
    /// 首次出现的组在 [`TopicSelection::groups`] 里的下标
    pub group: usize,
    /// 所选层的宫名（本命层即本命宫名）
    pub name: Palace,
    /// 本命宫名
    pub natal_name: Palace,
    /// 宫干飞化，按禄、权、科、忌；落宫等于 `index` 即自化
    pub flying: [Flight; 4],
}

/// 四化星落在选中宫里的一条。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutagenLanding {
    /// 四化
    pub mutagen: Mutagen,
    /// 化出的星
    pub star: StarKey,
    /// 星所在的宫位索引，必为选中宫之一
    pub palace: usize,
}

/// 运限某一层的四化星落在选中宫里的一条。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeMutagenLanding {
    /// 层级
    pub scope: Scope,
    /// 层级名标识：大限层未起运时为 [`HoroscopeName::Childhood`]
    pub name: HoroscopeName,
    /// 四化、星与落宫
    pub landing: MutagenLanding,
}

/// 一个主题在某一层选中的宫与相关四化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicSelection {
    /// 主题 slug
    pub topic: String,
    /// 所选层；本命为 [`Scope::Origin`]
    pub scope: Scope,
    /// 主题定义的各组，先 `surround` 后 `palaces`，各按定义顺序；落不到盘上的
    /// `originalPalace`（来因宫不在盘上）不成组
    pub groups: Vec<TopicGroup>,
    /// 选中的宫，每宫一次，按组的顺序、组内按本宫、对宫、财帛位、官禄位；即主题文本的成段顺序
    pub palaces: Vec<TopicPalace>,
    /// 生年四化落在选中宫的，按禄、权、科、忌
    pub birth_mutagens: Vec<MutagenLanding>,
    /// 大限至所选层各层四化落在选中宫的，按层级再按禄、权、科、忌；本命层为空
    pub scope_mutagens: Vec<ScopeMutagenLanding>,
}

impl TopicSelection {
    /// 选中宫的索引是否包含 `index`
    pub fn contains(&self, index: usize) -> bool {
        self.palaces.iter().any(|p| p.index == index)
    }
}

/// 包里名为 `topic` 的主题。
///
/// # Errors
/// 包里没有这个主题时返回 [`IztroError::InvalidArgument`]，列出包里的全部主题。
pub(crate) fn entry<'p>(
    pack: &'p KnowledgePack,
    topic: &str,
) -> Result<&'p TopicEntry, IztroError> {
    pack.topic(topic).ok_or_else(|| {
        let known: Vec<&str> = pack
            .topics_in_order()
            .into_iter()
            .map(|(slug, _)| slug)
            .collect();
        IztroError::InvalidArgument(format!(
            "unknown topic '{topic}': the pack defines {}",
            if known.is_empty() {
                "no topics".to_string()
            } else {
                known.join(", ")
            }
        ))
    })
}

/// 大限至 `scope` 各层，按层级先后；`scope` 为本命时为空。
pub(crate) fn layers_through(
    horoscope: &HoroscopeData,
    scope: Scope,
) -> Vec<(Scope, &HoroscopeItem)> {
    HOROSCOPE_SCOPES
        .into_iter()
        .filter(|s| s.depth() <= scope.depth())
        .filter_map(|s| horoscope.scope_item(s).map(|item| (s, item)))
        .collect()
}

/// 按主题定义选宫。`layers` 为大限至所选层各层（本命为空），宫名按最后一层定位。
///
/// # Errors
/// 主题里有不认识的宫位标识时返回 [`IztroError::InvalidArgument`]。
pub(crate) fn select(
    astrolabe: &Astrolabe,
    layers: &[(Scope, &HoroscopeItem)],
    topic: &str,
    entry: &TopicEntry,
) -> Result<TopicSelection, IztroError> {
    let layer = layers.last().map(|(_, item)| *item);
    let locate = |key: &str| -> Result<Option<usize>, IztroError> {
        let target = PalaceTarget::from_key(key).ok_or_else(|| {
            IztroError::InvalidArgument(format!(
                "topic '{topic}' has an invalid palace key '{key}': expected a palace name key like 'soulPalace', or 'bodyPalace'/'originalPalace'"
            ))
        })?;
        Ok(match (target, layer) {
            (PalaceTarget::Name(name), Some(item)) => Some(
                item.palace_names
                    .iter()
                    .position(|p| *p == name)
                    .expect("运限层十二宫名齐全"),
            ),
            _ => astrolabe.palace(target).map(|p| p.index),
        })
    };

    let mut groups = Vec::new();
    for key in entry.surround.iter().flatten() {
        if let Some(i) = locate(key)? {
            let sp = astrolabe.surrounded_palaces(i).expect("宫位索引来自本盘");
            groups.push(TopicGroup {
                kind: TopicGroupKind::Surround,
                key: key.clone(),
                target: i,
                opposite: Some(sp.opposite.index),
                wealth: Some(sp.wealth.index),
                career: Some(sp.career.index),
            });
        }
    }
    for key in entry.palaces.iter().flatten() {
        if let Some(i) = locate(key)? {
            groups.push(TopicGroup {
                kind: TopicGroupKind::Only,
                key: key.clone(),
                target: i,
                opposite: None,
                wealth: None,
                career: None,
            });
        }
    }

    let mut palaces: Vec<TopicPalace> = Vec::new();
    for (g, group) in groups.iter().enumerate() {
        let members = match group.kind {
            TopicGroupKind::Surround => vec![
                (TopicRole::Target, Some(group.target)),
                (TopicRole::Opposite, group.opposite),
                (TopicRole::Wealth, group.wealth),
                (TopicRole::Career, group.career),
            ],
            TopicGroupKind::Only => vec![(TopicRole::Related, Some(group.target))],
        };
        for (role, index) in members {
            let index = index.expect("surround 组四宫齐全");
            if palaces.iter().any(|p| p.index == index) {
                continue;
            }
            let data = &astrolabe.palaces[index];
            let places = data.mutaged_places(&astrolabe.palaces);
            let stars = data.mutagen_stars(&MUTAGEN);
            let flying = std::array::from_fn(|k| Flight {
                mutagen: MUTAGEN[k],
                star: stars[k],
                palace: places[k],
            });
            palaces.push(TopicPalace {
                index,
                role,
                group: g,
                name: layer.map_or(data.name, |item| item.palace_names[index]),
                natal_name: data.name,
                flying,
            });
        }
    }

    let selected = |i: usize| palaces.iter().any(|p| p.index == i);
    let mut birth_mutagens: Vec<MutagenLanding> = astrolabe
        .palaces
        .iter()
        .flat_map(|p| {
            p.major_stars
                .iter()
                .chain(&p.minor_stars)
                .chain(&p.adjective_stars)
                .filter_map(move |s| {
                    s.mutagen.map(|m| MutagenLanding {
                        mutagen: m,
                        star: s.key,
                        palace: p.index,
                    })
                })
        })
        .filter(|m| selected(m.palace))
        .collect();
    birth_mutagens.sort_by_key(|m| MUTAGEN.iter().position(|x| *x == m.mutagen));

    // 落宫与宫干飞化（`PalaceData::mutaged_places`）同一判定：星在该宫的主星、辅星或杂耀里
    let mut scope_mutagens = Vec::new();
    for (scope, item) in layers {
        for (star, mutagen) in item.mutagen.iter().zip(MUTAGEN) {
            if let Some(palace) = astrolabe.palaces.iter().position(|p| p.has(&[*star]))
                && selected(palace)
            {
                scope_mutagens.push(ScopeMutagenLanding {
                    scope: *scope,
                    name: item.name_key,
                    landing: MutagenLanding {
                        mutagen,
                        star: *star,
                        palace,
                    },
                });
            }
        }
    }

    Ok(TopicSelection {
        topic: topic.to_string(),
        scope: layers.last().map_or(Scope::Origin, |(s, _)| *s),
        groups,
        palaces,
        birth_mutagens,
        scope_mutagens,
    })
}

impl Astrolabe {
    /// 主题在本命盘上选中的宫：各组位置、每宫的角色与宫干飞化，以及落在这些宫里的生年四化。
    /// 与 [`crate::text::topic_to_text`] 同一套选宫，成段顺序即 [`TopicSelection::palaces`] 的顺序。
    ///
    /// # Errors
    /// 包里没有名为 `topic` 的主题，或主题里有不认识的宫位标识时返回 [`IztroError::InvalidArgument`]。
    pub fn topic_palaces(
        &self,
        pack: &KnowledgePack,
        topic: &str,
    ) -> Result<TopicSelection, IztroError> {
        select(self, &[], topic, entry(pack, topic)?)
    }
}

impl HoroscopeRef<'_> {
    /// 主题在 `scope` 层选中的宫：宫名按该层定位、三方四正以该层的本宫起算，另给大限至该层各层
    /// 落在选中宫里的四化。`Scope::Origin` 等同 [`Astrolabe::topic_palaces`]。
    ///
    /// # Errors
    /// `scope` 比查询精度（[`HoroscopeRef::precision`]）细，或同 [`Astrolabe::topic_palaces`]，
    /// 返回 [`IztroError::InvalidArgument`]。
    pub fn topic_palaces(
        &self,
        pack: &KnowledgePack,
        topic: &str,
        scope: Scope,
    ) -> Result<TopicSelection, IztroError> {
        self.check_within_precision(scope, "scope")?;
        let layers = layers_through(self.data(), scope);
        select(self.astrolabe(), &layers, topic, entry(pack, topic)?)
    }
}
