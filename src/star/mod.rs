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
