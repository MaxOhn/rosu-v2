macro_rules! def_enum {
    // Actually defining the enum and implementing Deserialize on it
    (@BASE $type:tt { $($variant:ident = $n:literal,)* }) => {
        #[allow(clippy::upper_case_acronyms, missing_docs)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $type {
            $($variant = $n,)*
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                d.deserialize_option(super::EnumVisitor::<$type>::new())
            }
        }
    };

    // Implementing From<$type> for u8, TryFrom<u8>, and Serialize
    (@SIGN u8 $type:tt { $($variant:ident = $n:literal,)* }) => {
        impl From<$type> for u8 {
            fn from(v: $type) -> Self {
                v as u8
            }
        }

        impl std::convert::TryFrom<u8> for $type {
            type Error = crate::error::OsuError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($n => Ok(<$type>::$variant),)*
                    _ => Err(crate::error::ParsingError::$type(value).into()),
                }
            }
        }

        impl serde::Serialize for $type {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_u8(*self as u8)
            }
        }
    };

    // Implementing From<$type> for i8, TryFrom<i8>, and Serialize
    (@SIGN i8 $type:tt { $($variant:ident = $n:literal,)* }) => {
        impl From<$type> for i8 {
            fn from(v: $type) -> Self {
                v as i8
            }
        }

        impl std::convert::TryFrom<i8> for $type {
            type Error = crate::error::OsuError;

            fn try_from(value: i8) -> Result<Self, Self::Error> {
                match value {
                    $($n => Ok(<$type>::$variant),)*
                    _ => Err(crate::error::ParsingError::$type(value).into()),
                }
            }
        }

        impl serde::Serialize for $type {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_i8(*self as i8)
            }
        }
    };

    // Got neither u8 nor i8
    (@SIGN $other:tt $($_:tt)*) => {
        compile_error!(concat!("Expected `u8` or `i8` as type, not ", stringify!($other)));
    };

    // Provide visit_u64 for visitor
    // Note that the type has to implement Default
    (@VISIT_DIGIT u8 $type:tt { $($variant:ident = $n:literal,)* }) => {
        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            match v {
                $($n => Ok(<$type>::$variant),)*
                _ => {
                    Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &stringify!($($n),*)))
                },
            }
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_any(self)
        }

        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok($type::default())
        }
    };

    // Provide visit_i64 and visit_u64 for visitor
    (@VISIT_DIGIT i8 $type:tt { $($variant:ident = $n:literal,)* }) => {
        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
            match v {
                $($n => Ok(<$type>::$variant),)*
                _ => {
                    Err(serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &stringify!($($n),*)))
                },
            }
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            match v as i64 {
                $($n => Ok(<$type>::$variant),)*
                _ => {
                    Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &stringify!($($n),*)))
                },
            }
        }

        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_any(self)
        }
    };

    // Provide visit_map for visitor
    (@VISIT_MAP $type:tt { $($variant:ident = $n:literal,)* }) => {
        fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut result = None;

            while let Some(key) = map.next_key::<&str>()? {
                match key {
                    "id" | "name" => {
                        result.replace(map.next_value()?);
                    }
                    _ => {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
            }

            result.ok_or_else(|| serde::de::Error::missing_field("id or name"))
        }
    };

    // Main macro with variants as serde strings
    ($sign:tt $type:tt { $($variant:ident = $n:literal,)* }) => {
        def_enum!(@BASE $type { $($variant = $n,)* });
        def_enum!(@SIGN $sign $type { $($variant = $n,)* });

        impl<'de> serde::de::Visitor<'de> for super::EnumVisitor<$type> {
            type Value = $type;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", concat!(stringify!($($n),*),$(", \"",stringify!($variant),"\"",)*))
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $(stringify!($variant) => Ok(<$type>::$variant),)*
                    _ => {
                        Err(serde::de::Error::unknown_variant(s, &[stringify!($($variant),*)]))
                    },
                }
            }

            def_enum!(@VISIT_DIGIT $sign $type { $($variant = $n,)* });
            def_enum!(@VISIT_MAP $type { $($variant = $n,)* });
        }
    };

    // Main macro with specified serde strings
    ($sign:tt $type:tt { $($variant:ident = $n:literal ($alt:literal),)* }) => {
        def_enum!(@BASE $type { $($variant = $n,)* });
        def_enum!(@SIGN $sign $type { $($variant = $n,)* });

        impl<'de> serde::de::Visitor<'de> for super::EnumVisitor<$type> {
            type Value = $type;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                 write!(f, "{}", concat!(stringify!($($n),*),", ",stringify!($($alt),*)))
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $($alt => Ok(<$type>::$variant),)*
                    _ => {
                        Err(serde::de::Error::unknown_variant(s, &[stringify!($($alt),*)]))
                    },
                }
            }

            def_enum!(@VISIT_DIGIT $sign $type { $($variant = $n,)* });
            def_enum!(@VISIT_MAP $type { $($variant = $n,)* });
        }
    }
}

mod cursor;
mod grade;
mod mode;
mod mods;

/// Beatmap(set) related types
pub mod beatmap;
/// Comment related types
pub mod comments;
/// Forum post related types
pub mod forum;
/// User kudosu related types
pub mod kudosu;
/// Multiplayer match related types
pub mod matches;
/// News related types
pub mod news;
/// Ranking related types
pub mod ranking;
/// User event related types
pub mod recent_event;
/// Score related types
pub mod score;
/// Seasonal background related types
pub mod seasonal_backgrounds;
/// User related types
pub mod user;
/// Wiki related types
pub mod wiki;

pub use cursor::Cursor;
pub use grade::Grade;
pub use mode::GameMode;
pub use mods::GameMods;

use serde::{Deserialize, Deserializer, Serializer};
use std::marker::PhantomData;

struct EnumVisitor<T> {
    phantom: PhantomData<T>,
}

impl<T> EnumVisitor<T> {
    fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

fn inflate_acc<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
    let acc: f32 = Deserialize::deserialize(d)?;

    Ok(100.0 * acc)
}

fn deflate_acc<S: Serializer>(f: &f32, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_f32(*f / 100.0)
}
