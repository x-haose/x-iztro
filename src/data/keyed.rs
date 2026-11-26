//! 带语言无关标识的枚举：一张「变体 => 标识」表展开出全部标识相关实现。

/// 报错时逐一列出期望取值的上限；超过它（如星耀 164 个、格局 64 个）只指明是哪类标识。
const LISTED_KEYS_MAX: usize = 12;

/// 期望取值的列举写法：`'a'`、`'a' or 'b'`、`'a', 'b' or 'c'`。
pub(crate) fn expected_keys(keys: &[&str]) -> String {
    let quoted: Vec<String> = keys.iter().map(|k| format!("'{k}'")).collect();
    match quoted.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} or {last}", rest.join(", ")),
    }
}

/// 未知标识的报错正文；`what` 是该类标识的称呼（如 `heavenly stem`）。
pub(crate) fn unknown_key_message(what: &str, key: &str, keys: &[&str]) -> String {
    if keys.len() <= LISTED_KEYS_MAX {
        format!("invalid {what} '{key}': expected {}", expected_keys(keys))
    } else {
        format!("unknown {what} '{key}'")
    }
}

/// 为带语言无关标识的枚举从一张「变体 => 标识」表生成全部标识相关实现。
///
/// 标识是该枚举在绑定线协议、DTO、知识包与 serde 里唯一的取值形态，表只写这一份：
/// `KEYS` / `as_key` / `from_key`、按标识的 `Serialize` / `Deserialize`，以及启用
/// `schemars` feature 时列出全部标识的 JSON Schema 都由它展开。`$what` 是报错里对
/// 该类标识的称呼。
macro_rules! keyed_enum {
    ($ty:ident, $what:literal { $($variant:ident => $key:literal),+ $(,)? }) => {
        impl $ty {
            /// 全部语言无关标识，顺序同变体声明顺序
            pub const KEYS: &'static [&'static str] = &[$($key),+];

            /// 语言无关标识
            pub fn as_key(self) -> &'static str {
                match self {
                    $($ty::$variant => $key),+
                }
            }

            /// 由语言无关标识还原；未知标识返回 `None`
            pub fn from_key(key: &str) -> Option<Self> {
                match key {
                    $($key => Some($ty::$variant),)+
                    _ => None,
                }
            }
        }

        impl serde::Serialize for $ty {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_key())
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let key = <String as serde::Deserialize>::deserialize(deserializer)?;
                $ty::from_key(&key).ok_or_else(|| {
                    serde::de::Error::custom($crate::data::keyed::unknown_key_message(
                        $what, &key, $ty::KEYS,
                    ))
                })
            }
        }

        #[cfg(feature = "schemars")]
        impl schemars::JsonSchema for $ty {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                stringify!($ty).into()
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                concat!(module_path!(), "::", stringify!($ty)).into()
            }

            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({ "type": "string", "enum": $ty::KEYS })
            }
        }
    };
}

pub(crate) use keyed_enum;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_keys_reads_as_a_list() {
        assert_eq!(expected_keys(&["a"]), "'a'");
        assert_eq!(expected_keys(&["a", "b"]), "'a' or 'b'");
        assert_eq!(expected_keys(&["a", "b", "c"]), "'a', 'b' or 'c'");
    }
}
