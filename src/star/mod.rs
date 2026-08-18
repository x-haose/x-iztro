//! 安星
//!
//! 分两层：`location` / `major` / `minor` / `adjective` / `decorative` 收已算好的
//! 宫位索引，是排盘流水线的构件；`query` 收出生数据本身，是对外的安星入口。

/// 杂耀
pub mod adjective;
/// 长生12神、博士12神、岁前12神、将前12神
pub mod decorative;
/// 各星耀落宫索引
pub mod location;
/// 十四主星
pub mod major;
/// 十四辅星
pub mod minor;
/// 按出生数据安星
pub mod query;

use crate::data::stars::{MUTAGEN, StarKey};
use crate::data::types::*;
use crate::i18n::translate_star;
use crate::models::star::Star;

/// 组装一颗主星或辅星。
///
/// 亮度由 `config` 的亮度表按落宫 `palace_index` 查得；四化取该星在
/// `yearly_stem` 化出的四颗星中的位次，`yearly_stem` 为 `None` 表示这颗星
/// 不参与四化（如中州派下不带四化的辅星）。
pub(crate) fn make_star(
    key: StarKey,
    star_type: StarType,
    scope: Scope,
    palace_index: usize,
    yearly_stem: Option<HeavenlyStem>,
    lang: Language,
    config: &Config,
) -> Star {
    let brightness = config.brightness_of(key, palace_index);
    let mutagen = yearly_stem.and_then(|stem| {
        config
            .mutagens_of(stem)
            .iter()
            .position(|&k| k == key)
            .map(|i| MUTAGEN[i])
    });
    Star {
        key,
        name: translate_star(key, lang).to_string(),
        star_type,
        scope,
        brightness,
        mutagen,
    }
}
