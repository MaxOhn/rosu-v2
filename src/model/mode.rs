use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Available game modes
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(as = "Self")
)]
#[repr(u8)]
pub enum GameMode {
    /// osu!standard
    Osu = 0,
    /// osu!taiko
    Taiko = 1,
    /// osu!catch
    Catch = 2,
    /// osu!mania
    Mania = 3,
}

impl From<u8> for GameMode {
    #[inline]
    fn from(mode: u8) -> Self {
        match mode {
            0 => GameMode::Osu,
            1 => GameMode::Taiko,
            2 => GameMode::Catch,
            3 => GameMode::Mania,
            _ => GameMode::Osu,
        }
    }
}

impl Default for GameMode {
    #[inline]
    fn default() -> Self {
        Self::Osu
    }
}

impl fmt::Display for GameMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Osu => f.write_str("osu"),
            Self::Taiko => f.write_str("taiko"),
            Self::Catch => f.write_str("fruits"),
            Self::Mania => f.write_str("mania"),
        }
    }
}

struct ModeVisitor;

impl<'de> Visitor<'de> for ModeVisitor {
    type Value = GameMode;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a u8 or a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let mode = match v {
            "0" | "osu" | "osu!" => GameMode::Osu,
            "1" | "taiko" | "tko" => GameMode::Taiko,
            "2" | "ctb" | "fruits" => GameMode::Catch,
            "3" | "mania" | "mna" => GameMode::Mania,
            _ => {
                return Err(Error::invalid_value(
                    Unexpected::Str(v),
                    &r#""0", "osu", "osu!", "1", "taiko", "tko", "2", "ctb", "fruits", "3", "mania", or "mna""#,
                ))
            }
        };

        Ok(mode)
    }

    #[inline]
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(GameMode::Osu),
            1 => Ok(GameMode::Taiko),
            2 => Ok(GameMode::Catch),
            3 => Ok(GameMode::Mania),
            _ => Err(Error::invalid_value(
                Unexpected::Unsigned(v),
                &"0, 1, 2, or 3",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for GameMode {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(ModeVisitor)
    }
}

#[cfg(feature = "serialize")]
impl serde::Serialize for GameMode {
    #[inline]
    fn serialize<S: serde::ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(*self as u8)
    }
}
