macro_rules! def_enum {
    (@BASE $type:tt { $($variant:ident = $n:literal,)* }) => {
        #[allow(clippy::upper_case_acronyms, missing_docs)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        #[repr(u8)]
        pub enum $type {
            $($variant = $n,)*
        }

        impl<'de> serde::Deserialize<'de> for $type {
            #[inline]
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                d.deserialize_option(super::EnumVisitor::<$type>::new())
            }
        }

        impl From<$type> for u8 {
            #[inline]
            fn from(v: $type) -> Self {
                v as u8
            }
        }

        impl std::convert::TryFrom<u8> for $type {
            type Error = crate::error::OsuError;

            #[inline]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($n => Ok(<$type>::$variant),)*
                    _ => Err(crate::error::ParsingError::$type(value).into()),
                }
            }
        }

        #[cfg(feature = "serialize")]
        impl serde::Serialize for $type {
            #[inline]
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_u8(*self as u8)
            }
        }
    };

    (@VISIT $type:tt { $($variant:ident = $n:literal,)* }) => {
        #[inline]
        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            match v {
                $($n => Ok(<$type>::$variant),)*
                _ => {
                    Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &stringify!($($n),*)))
                },
            }
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_any(self)
        }

        #[inline]
        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok($type::default())
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut result = None;

            while let Some(key) = map.next_key::<&str>()? {
                match key {
                    "id" | "name" => result = Some(map.next_value()?),
                    _ => {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
            }

            result.ok_or_else(|| serde::de::Error::missing_field("id or name"))
        }
    };

    // Main macro with variants as serde strings
    ($type:tt { $($variant:ident = $n:literal,)* }) => {
        def_enum!(@BASE $type { $($variant = $n,)* });

        impl<'de> serde::de::Visitor<'de> for super::EnumVisitor<$type> {
            type Value = $type;

            #[inline]
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(concat!(stringify!($($n),*),$(", \"",stringify!($variant),"\"",)*))
            }

            #[inline]
            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $(stringify!($variant) => Ok(<$type>::$variant),)*
                    _ => {
                        Err(serde::de::Error::unknown_variant(s, &[stringify!($($variant),*)]))
                    },
                }
            }

            def_enum!(@VISIT $type { $($variant = $n,)* });
        }
    };

    // Main macro with specified serde strings
    ($type:tt { $($variant:ident = $n:literal ($alt:literal),)* }) => {
        def_enum!(@BASE $type { $($variant = $n,)* });

        impl<'de> serde::de::Visitor<'de> for super::EnumVisitor<$type> {
            type Value = $type;

            #[inline]
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                 f.write_str(concat!(stringify!($($n),*),", ",stringify!($($alt),*)))
            }

            #[inline]
            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $($alt => Ok(<$type>::$variant),)*
                    _ => {
                        Err(serde::de::Error::unknown_variant(s, &[stringify!($($alt),*)]))
                    },
                }
            }

            def_enum!(@VISIT $type { $($variant = $n,)* });
        }
    }
}

mod grade;
mod serde_util;

/// Beatmap(set) related types
pub mod beatmap;

/// Comment related types
pub mod comments;

/// Event related types
pub mod event;

/// Forum post related types
pub mod forum;

/// User kudosu related types
pub mod kudosu;

/// Multiplayer match related types
pub mod matches;

/// Re-exports of `rosu-mods`
pub mod mods;

/// Multiplayer related types
pub mod multiplayer;

/// News related types
pub mod news;

/// Ranking related types
pub mod ranking;

/// Score related types
pub mod score;

/// Seasonal background related types
pub mod seasonal_backgrounds;

/// User related types
pub mod user;

/// Wiki related types
pub mod wiki;

use std::{collections::HashMap, marker::PhantomData};

pub use rosu_mods::GameMode;

pub use self::{grade::Grade, serde_util::DeserializedList};

use self::user::Username;

struct EnumVisitor<T>(PhantomData<T>);

impl<T> EnumVisitor<T> {
    const fn new() -> Self {
        Self(PhantomData)
    }
}

/// Trait alias for `Copy + Fn(u32, &str)`.
///
/// Awaiting <https://github.com/rust-lang/rust/issues/41517>
pub(crate) trait CacheUserFn: Copy + Fn(u32, &Username) {}

impl<T: Copy + Fn(u32, &Username)> CacheUserFn for T {}

/// A way to apply a given function to all users contained in `Self`.
pub(crate) trait ContainedUsers {
    /// Applies `f` to all (user id, username) pairs contained in `self`.
    #[cfg_attr(
        not(feature = "cache"),
        allow(dead_code, reason = "its only used to put all users in the cache")
    )]
    fn apply_to_users(&self, f: impl CacheUserFn);
}

impl<T: ContainedUsers> ContainedUsers for Box<T> {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        (**self).apply_to_users(f);
    }
}

impl<T: ContainedUsers> ContainedUsers for Option<T> {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        if let Some(item) = self {
            item.apply_to_users(f);
        }
    }
}

impl<T: ContainedUsers, E> ContainedUsers for Result<T, E> {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        if let Ok(ok) = self {
            ok.apply_to_users(f);
        }
    }
}

impl<T: ContainedUsers> ContainedUsers for Vec<T> {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        for item in self {
            item.apply_to_users(f);
        }
    }
}

impl<K, T: ContainedUsers, S> ContainedUsers for HashMap<K, T, S> {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        for value in self.values() {
            value.apply_to_users(f);
        }
    }
}
