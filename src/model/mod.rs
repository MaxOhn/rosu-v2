macro_rules! def_enum {
    // Actually defining the enum and implementing Deserialize on it
    (@BASE $type:tt { $($variant:ident = $n:literal,)* }) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $type {
            $($variant = $n,)*
        }

        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                d.deserialize_any(super::EnumVisitor::<$type>::new())
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

        impl Serialize for $type {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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

        impl Serialize for $type {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_i8(*self as i8)
            }
        }
    };

    // Got neither u8 nor i8
    (@SIGN $other:tt $($_:tt)*) => {
        compile_error!(concat!("Expected `u8` or `i8` as type, not ", stringify!($other)));
    };

    // Provide visit_u64 for visitor
    (@VISIT u8 $type:tt { $($variant:ident = $n:literal,)* }) => {
        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            match v {
                $($n => Ok(<$type>::$variant),)*
                _ => {
                    Err(Error::invalid_value(Unexpected::Unsigned(v), &stringify!($($n),*)))
                },
            }
        }
    };

    // Provide visit_i64 and visit_u64 for visitor
    (@VISIT i8 $type:tt { $($variant:ident = $n:literal,)* }) => {
        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            match v {
                $($n => Ok(<$type>::$variant),)*
                _ => {
                    Err(Error::invalid_value(Unexpected::Signed(v), &stringify!($($n),*)))
                },
            }
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            match v as i64 {
                $($n => Ok(<$type>::$variant),)*
                _ => {
                    Err(Error::invalid_value(Unexpected::Unsigned(v), &stringify!($($n),*)))
                },
            }
        }
    };

    // Main macro with variants as serde strings
    ($sign:tt $type:tt { $($variant:ident = $n:literal,)* }) => {
        def_enum!(@BASE $type { $($variant = $n,)* });
        def_enum!(@SIGN $sign $type { $($variant = $n,)* });

        impl<'de> Visitor<'de> for super::EnumVisitor<$type> {
            type Value = $type;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", concat!(stringify!($($n),*),$(", \"",stringify!($variant),"\"",)*))
            }

            fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $(stringify!($variant) => Ok(<$type>::$variant),)*
                    _ => {
                        Err(Error::invalid_value(Unexpected::Str(s), &stringify!($($variant),*)))
                    },
                }
            }

            def_enum!(@VISIT $sign $type { $($variant = $n,)* });
        }
    };

    // Main macro with specified serde strings
    ($sign:tt $type:tt { $($variant:ident = $n:literal ($alt:literal),)* }) => {
        def_enum!(@BASE $type { $($variant = $n,)* });
        def_enum!(@SIGN $sign $type { $($variant = $n,)* });

        impl<'de> Visitor<'de> for super::EnumVisitor<$type> {
            type Value = $type;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                 write!(f, "{}", concat!(stringify!($($n),*),", ",stringify!($($alt),*)))
            }

            fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $($alt => Ok(<$type>::$variant),)*
                    _ => {
                        Err(Error::invalid_value(Unexpected::Str(s), &stringify!($($alt),*)))
                    },
                }
            }

            def_enum!(@VISIT $sign $type { $($variant = $n,)* });
        }
    }
}

pub mod beatmap;
pub mod comments;
pub mod forum;
mod grade;
pub mod kudosu;
pub mod matches;
mod mode;
mod mods;
pub mod multiplayer;
pub mod news;
pub mod ranking;
pub mod recent_event;
pub mod score;
pub mod user;
pub mod wiki;

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
