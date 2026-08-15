//! Golden star 测试：`star` 模块各安星入口逐值对照 JS 输出。
//!
//! 数据由 tests/golden/generate_star.mjs 生成，星耀一律记 key。
//! 覆盖 8 例出生数据（含闰月、晚子时、跨季、男女、五种五行局）的全部安星函数，
//! 干支全组合 × 六个运限层级的流耀，两个起始索引函数的全枚举，
//! 以及低层落宫函数按各自入参域的全覆盖。

/// 只按年支落宫的那几个函数共用的形状
type BranchToIndex = fn(EarthlyBranch) -> usize;

use serde_json::Value;
use std::fs;
use x_iztro::astro::horoscope::get_horoscope_stars;
use x_iztro::data::stars::StarKey;
use x_iztro::data::types::*;
use x_iztro::models::star::Star;
use x_iztro::star::decorative::{get_changsheng12_start_index, get_jiangqian12_start_index};
use x_iztro::star::query::{self, StarParam};

const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/star_cases.json");

fn load() -> Value {
    let raw =
        fs::read_to_string(GOLDEN).expect("缺少 star_cases.json，先跑 node generate_star.mjs");
    serde_json::from_str(&raw).expect("star_cases.json 不是合法 JSON")
}

fn idx(v: &Value, field: &str) -> usize {
    v[field]
        .as_u64()
        .unwrap_or_else(|| panic!("缺少索引字段 {field}")) as usize
}

/// 十二宫星耀 → 每宫的 key 列表
fn star_keys(groups: &[Vec<Star>; 12]) -> Vec<Vec<String>> {
    groups
        .iter()
        .map(|palace| palace.iter().map(|s| s.key.as_key().to_string()).collect())
        .collect()
}

fn shen_keys(shen: &[StarKey; 12]) -> Vec<String> {
    shen.iter().map(|k| k.as_key().to_string()).collect()
}

fn expect_star_groups(v: &Value) -> Vec<Vec<String>> {
    v.as_array()
        .expect("星耀分布不是数组")
        .iter()
        .map(|palace| {
            palace
                .as_array()
                .expect("宫内星耀不是数组")
                .iter()
                .map(|s| s.as_str().unwrap().to_string())
                .collect()
        })
        .collect()
}

fn expect_list(v: &Value) -> Vec<String> {
    v.as_array()
        .expect("十二神不是数组")
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect()
}

#[test]
fn star_module_matches_js() {
    let golden = load();
    let config = Config::default();
    let mut failures = Vec::new();

    for case in golden["cases"].as_array().expect("cases 不是数组") {
        let p = &case["param"];
        let solar_date = p["solarDate"].as_str().unwrap();
        let time_index = p["timeIndex"].as_u64().unwrap() as u8;
        let gender = match p["gender"].as_str().unwrap() {
            "男" => Gender::Male,
            _ => Gender::Female,
        };
        let fix_leap = p["fixLeap"].as_bool().unwrap();
        let tag = format!(
            "{solar_date} ti={time_index} {}",
            p["gender"].as_str().unwrap()
        );

        let param = StarParam {
            solar_date,
            time_index,
            gender,
            fix_leap,
            from: None,
            language: Language::ZhCN,
            config: &config,
        };

        let mut check = |field: &str, want: usize, got: usize| {
            if want != got {
                failures.push(format!("{tag} {field}: JS={want} Rust={got}"));
            }
        };

        let start = query::get_start_index(&param).expect("起宫计算失败");
        check(
            "startIndex.ziwei",
            idx(&case["startIndex"], "ziweiIndex"),
            start.ziwei,
        );
        check(
            "startIndex.tianfu",
            idx(&case["startIndex"], "tianfuIndex"),
            start.tianfu,
        );

        let lytm = query::get_lu_yang_tuo_ma_index(&param).expect("禄羊陀马计算失败");
        let g = &case["luYangTuoMa"];
        check("lu", idx(g, "luIndex"), lytm.lu);
        check("yang", idx(g, "yangIndex"), lytm.yang);
        check("tuo", idx(g, "tuoIndex"), lytm.tuo);
        check("ma", idx(g, "maIndex"), lytm.ma);

        let kui_yue = query::get_kui_yue_index(&param).expect("魁钺计算失败");
        check("kui", idx(&case["kuiYue"], "kuiIndex"), kui_yue.kui);
        check("yue", idx(&case["kuiYue"], "yueIndex"), kui_yue.yue);

        let chang_qu = query::get_chang_qu_index(&param).expect("昌曲计算失败");
        check("chang", idx(&case["changQu"], "changIndex"), chang_qu.chang);
        check("qu", idx(&case["changQu"], "quIndex"), chang_qu.qu);

        let kong_jie = query::get_kong_jie_index(&param).expect("空劫计算失败");
        check("kong", idx(&case["kongJie"], "kongIndex"), kong_jie.kong);
        check("jie", idx(&case["kongJie"], "jieIndex"), kong_jie.jie);

        let timely = query::get_timely_star_index(&param).expect("台辅封诰计算失败");
        check("taifu", idx(&case["timely"], "taifuIndex"), timely.taifu);
        check(
            "fenggao",
            idx(&case["timely"], "fenggaoIndex"),
            timely.fenggao,
        );

        let luan_xi = query::get_luan_xi_index(&param).expect("鸾喜计算失败");
        check(
            "hongluan",
            idx(&case["luanXi"], "hongluanIndex"),
            luan_xi.hongluan,
        );
        check(
            "tianxi",
            idx(&case["luanXi"], "tianxiIndex"),
            luan_xi.tianxi,
        );

        let daily = query::get_daily_star_index(&param).expect("日系星计算失败");
        let g = &case["daily"];
        check("santai", idx(g, "santaiIndex"), daily.santai);
        check("bazuo", idx(g, "bazuoIndex"), daily.bazuo);
        check("enguang", idx(g, "enguangIndex"), daily.enguang);
        check("tiangui", idx(g, "tianguiIndex"), daily.tiangui);

        let monthly = query::get_monthly_star_index(&param).expect("月系星计算失败");
        let g = &case["monthly"];
        check("jieshen", idx(g, "yuejieIndex"), monthly.jieshen);
        check("tianyao", idx(g, "tianyaoIndex"), monthly.tianyao);
        check("tianxing", idx(g, "tianxingIndex"), monthly.tianxing);
        check("yinsha", idx(g, "yinshaIndex"), monthly.yinsha);
        check("tianyue", idx(g, "tianyueIndex"), monthly.tianyue);
        check("tianwu", idx(g, "tianwuIndex"), monthly.tianwu);

        let yearly = query::get_yearly_star_index(&param).expect("年系星计算失败");
        let g = &case["yearly"];
        for (field, got) in [
            ("xianchiIndex", yearly.xianchi),
            ("huagaiIndex", yearly.huagai),
            ("guchenIndex", yearly.guchen),
            ("guasuIndex", yearly.guasu),
            ("tiancaiIndex", yearly.tiancai),
            ("tianshouIndex", yearly.tianshou),
            ("tianchuIndex", yearly.tianchu),
            ("posuiIndex", yearly.posui),
            ("feilianIndex", yearly.feilian),
            ("longchiIndex", yearly.longchi),
            ("fenggeIndex", yearly.fengge),
            ("tiankuIndex", yearly.tianku),
            ("tianxuIndex", yearly.tianxu),
            ("tianguanIndex", yearly.tianguan),
            ("tianfuIndex", yearly.tianfu),
            ("tiandeIndex", yearly.tiande),
            ("yuedeIndex", yearly.yuede),
            ("tiankongIndex", yearly.tiankong),
            ("jieluIndex", yearly.jielu),
            ("kongwangIndex", yearly.kongwang),
            ("xunkongIndex", yearly.xunkong),
            ("tianshangIndex", yearly.tianshang),
            ("tianshiIndex", yearly.tianshi),
            ("jiekongIndex", yearly.jiekong),
            ("jieshaAdjIndex", yearly.jiesha),
            ("nianjieIndex", yearly.nianjie),
            ("dahaoAdjIndex", yearly.dahao),
        ] {
            check(field, idx(g, field), got);
        }

        let mut check_group = |field: &str, want: Vec<Vec<String>>, got: Vec<Vec<String>>| {
            if want != got {
                failures.push(format!("{tag} {field}:\n  JS  ={want:?}\n  Rust={got:?}"));
            }
        };

        check_group(
            "majorStars",
            expect_star_groups(&case["majorStars"]),
            star_keys(&query::get_major_stars(&param).expect("主星计算失败")),
        );
        check_group(
            "minorStars",
            expect_star_groups(&case["minorStars"]),
            star_keys(&query::get_minor_stars(&param).expect("辅星计算失败")),
        );
        check_group(
            "adjectiveStars",
            expect_star_groups(&case["adjectiveStars"]),
            star_keys(&query::get_adjective_stars(&param).expect("杂耀计算失败")),
        );

        let mut check_shen = |field: &str, want: Vec<String>, got: Vec<String>| {
            if want != got {
                failures.push(format!("{tag} {field}:\n  JS  ={want:?}\n  Rust={got:?}"));
            }
        };

        check_shen(
            "changsheng12",
            expect_list(&case["changsheng12"]),
            shen_keys(&query::get_changsheng12(&param).expect("长生12计算失败")),
        );
        check_shen(
            "boshi12",
            expect_list(&case["boshi12"]),
            shen_keys(&query::get_boshi12(&param).expect("博士12计算失败")),
        );

        let (suiqian12, jiangqian12) = query::get_yearly12(&param).expect("流年12神计算失败");
        check_shen(
            "suiqian12",
            expect_list(&case["yearly12"]["suiqian12"]),
            shen_keys(&suiqian12),
        );
        check_shen(
            "jiangqian12",
            expect_list(&case["yearly12"]["jiangqian12"]),
            shen_keys(&jiangqian12),
        );
    }

    assert!(
        failures.is_empty(),
        "star 模块差异:\n{}",
        failures.join("\n")
    );
}

/// iztro 在 `getHoroscopeStar` 的本命层级把红鸾写成了 `hongluanMin`，
/// 该标识不在其 i18n 表中，任何语言下都原样漏成星名；本命红鸾的正确标识是 `hongluan`。
/// x-iztro 安放正确标识，这是与 iztro 唯一的取值差异，在此显式断言：
/// 出现处必须恰好是这一格，别处一律零容忍。
const IZTRO_HONGLUAN_TYPO: &str = "hongluanMin";
const HONGLUAN: &str = "hongluan";

#[test]
fn horoscope_stars_match_js() {
    let golden = load();
    let mut failures = Vec::new();
    let mut typo_cells = 0usize;

    for case in golden["horoscopeStars"]
        .as_array()
        .expect("horoscopeStars 不是数组")
    {
        let stem = HeavenlyStem::from_key(&pinyin_stem(case["stem"].as_str().unwrap()))
            .expect("天干标识无效");
        let branch = EarthlyBranch::from_key(&pinyin_branch(case["branch"].as_str().unwrap()))
            .expect("地支标识无效");
        let scope = Scope::from_key(case["scope"].as_str().unwrap()).expect("运限层级无效");

        let mut want = expect_star_groups(&case["stars"]);
        let got = star_keys(&get_horoscope_stars(stem, branch, scope, Language::ZhCN));

        // 把 iztro 的错字标识换成正确标识后，其余必须逐值相同
        for palace in &mut want {
            for key in palace.iter_mut() {
                if key == IZTRO_HONGLUAN_TYPO {
                    assert_eq!(scope, Scope::Origin, "错字标识只应出现在本命层级");
                    *key = HONGLUAN.to_string();
                    typo_cells += 1;
                }
            }
        }

        if want != got {
            failures.push(format!(
                "{}{} {}:\n  JS  ={want:?}\n  Rust={got:?}",
                case["stem"].as_str().unwrap(),
                case["branch"].as_str().unwrap(),
                case["scope"].as_str().unwrap()
            ));
        }
    }

    // 干支 120 组合各有一颗本命红鸾；数目不符说明 iztro 改了行为或替换写错了
    assert_eq!(typo_cells, 120, "iztro 错字标识出现次数与预期不符");
    assert!(failures.is_empty(), "流耀差异:\n{}", failures.join("\n"));
}

#[test]
fn start_indexes_match_js() {
    let golden = load();
    let mut failures = Vec::new();

    for case in golden["changsheng12StartIndex"].as_array().unwrap() {
        let fe = FiveElementsClass::from_key(case["fiveElementsClass"].as_str().unwrap())
            .expect("五行局标识无效");
        let want = case["index"].as_u64().unwrap() as usize;
        let got = get_changsheng12_start_index(fe);
        if want != got {
            failures.push(format!("长生12起点 {fe:?}: JS={want} Rust={got}"));
        }
    }

    for case in golden["jiangqian12StartIndex"].as_array().unwrap() {
        let branch =
            EarthlyBranch::from_key(case["branch"].as_str().unwrap()).expect("地支标识无效");
        let want = case["index"].as_u64().unwrap() as usize;
        let got = get_jiangqian12_start_index(branch);
        if want != got {
            failures.push(format!("将前12起点 {branch:?}: JS={want} Rust={got}"));
        }
    }

    assert!(
        failures.is_empty(),
        "起始索引差异:\n{}",
        failures.join("\n")
    );
}

/// 低层落宫函数逐值对照 JS。
///
/// 这些函数不收出生数据、只收已算好的中间量，金标按各自入参域全覆盖：
/// 农历月 12、年支 12（火铃再乘 13 个时辰）、天干 10、
/// 天伤天使为两个派别 × 男女 × 年支 × 命宫索引。
#[test]
fn location_indices_match_js() {
    let golden = load();
    let loc = &golden["locations"];
    let mut failures = Vec::new();

    let branch_of = |case: &Value| {
        EarthlyBranch::from_key(case["branch"].as_str().expect("缺 branch")).expect("地支标识无效")
    };

    for case in loc["zuoYou"].as_array().expect("缺 zuoYou") {
        let month = case["lunarMonth"].as_u64().unwrap() as u32;
        let got = x_iztro::star::location::get_zuo_you_index(month);
        if got.zuo != idx(case, "zuoIndex") || got.you != idx(case, "youIndex") {
            failures.push(format!(
                "左辅右弼 月{month}: JS=({},{}) Rust=({},{})",
                idx(case, "zuoIndex"),
                idx(case, "youIndex"),
                got.zuo,
                got.you
            ));
        }
    }

    for case in loc["huoLing"].as_array().expect("缺 huoLing") {
        let time_index = case["timeIndex"].as_u64().unwrap() as u8;
        let got = x_iztro::star::location::get_huo_ling_index(branch_of(case), time_index);
        if got.huo != idx(case, "huoIndex") || got.ling != idx(case, "lingIndex") {
            failures.push(format!(
                "火星铃星 {:?}/{time_index}: JS=({},{}) Rust=({},{})",
                branch_of(case),
                idx(case, "huoIndex"),
                idx(case, "lingIndex"),
                got.huo,
                got.ling
            ));
        }
    }

    for case in loc["huagaiXianchi"].as_array().expect("缺 huagaiXianchi") {
        let got = x_iztro::star::location::get_huagai_xianchi_index(branch_of(case));
        if got.huagai != idx(case, "huagaiIndex") || got.xianchi != idx(case, "xianchiIndex") {
            failures.push(format!(
                "华盖咸池 {:?}: JS=({},{}) Rust=({},{})",
                branch_of(case),
                idx(case, "huagaiIndex"),
                idx(case, "xianchiIndex"),
                got.huagai,
                got.xianchi
            ));
        }
    }

    for case in loc["guGua"].as_array().expect("缺 guGua") {
        let got = x_iztro::star::location::get_gu_gua_index(branch_of(case));
        if got.guchen != idx(case, "guchenIndex") || got.guasu != idx(case, "guasuIndex") {
            failures.push(format!(
                "孤辰寡宿 {:?}: JS=({},{}) Rust=({},{})",
                branch_of(case),
                idx(case, "guchenIndex"),
                idx(case, "guasuIndex"),
                got.guchen,
                got.guasu
            ));
        }
    }

    let singles: [(&str, BranchToIndex); 3] = [
        ("jieshaAdj", x_iztro::star::location::get_jiesha_adj_index),
        ("dahao", x_iztro::star::location::get_dahao_index),
        ("nianjie", x_iztro::star::location::get_nianjie_index),
    ];
    for (name, f) in singles {
        for case in loc[name].as_array().unwrap_or_else(|| panic!("缺 {name}")) {
            let want = idx(case, "index");
            let got = f(branch_of(case));
            if want != got {
                failures.push(format!(
                    "{name} {:?}: JS={want} Rust={got}",
                    branch_of(case)
                ));
            }
        }
    }

    for case in loc["changQuByStem"].as_array().expect("缺 changQuByStem") {
        let stem =
            HeavenlyStem::from_key(case["stem"].as_str().expect("缺 stem")).expect("天干标识无效");
        let got = x_iztro::star::location::get_chang_qu_index_by_stem(stem);
        if got.chang != idx(case, "changIndex") || got.qu != idx(case, "quIndex") {
            failures.push(format!(
                "昌曲按天干 {stem:?}: JS=({},{}) Rust=({},{})",
                idx(case, "changIndex"),
                idx(case, "quIndex"),
                got.chang,
                got.qu
            ));
        }
    }

    for case in loc["tianshiTianshang"]
        .as_array()
        .expect("缺 tianshiTianshang")
    {
        let algorithm = match case["algorithm"].as_str().expect("缺 algorithm") {
            "zhongzhou" => Algorithm::Zhongzhou,
            _ => Algorithm::Default,
        };
        let gender = match case["gender"].as_str().expect("缺 gender") {
            "female" => Gender::Female,
            _ => Gender::Male,
        };
        let soul_index = idx(case, "soulIndex");
        let (shang, shi) = x_iztro::star::location::get_tianshang_tianshi_index(
            gender,
            branch_of(case),
            soul_index,
            algorithm,
        );
        if shang != idx(case, "tianshangIndex") || shi != idx(case, "tianshiIndex") {
            failures.push(format!(
                "天伤天使 {algorithm:?}/{gender:?}/{:?}/命{soul_index}: JS=({},{}) Rust=({shang},{shi})",
                branch_of(case),
                idx(case, "tianshangIndex"),
                idx(case, "tianshiIndex"),
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "低层落宫差异（{} 处）:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// 中文天干 → 标识
fn pinyin_stem(s: &str) -> String {
    let key = match s {
        "甲" => "jia",
        "乙" => "yi",
        "丙" => "bing",
        "丁" => "ding",
        "戊" => "wu",
        "己" => "ji",
        "庚" => "geng",
        "辛" => "xin",
        "壬" => "ren",
        _ => "gui",
    };
    format!("{key}Heavenly")
}

/// 中文地支 → 标识
fn pinyin_branch(s: &str) -> String {
    let key = match s {
        "子" => "zi",
        "丑" => "chou",
        "寅" => "yin",
        "卯" => "mao",
        "辰" => "chen",
        "巳" => "si",
        "午" => "wu",
        "未" => "wei",
        "申" => "shen",
        "酉" => "you",
        "戌" => "xu",
        _ => "hai",
    };
    format!("{key}Earthly")
}
