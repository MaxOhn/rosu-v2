use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Available game modes
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize), archive(as = "Self"))]
#[repr(u8)]
pub enum GameMode {
    /// osu!standard
    STD = 0,
    /// osu!taiko
    TKO = 1,
    /// osu!catch
    CTB = 2,
    /// osu!mania
    MNA = 3,
}

impl From<u8> for GameMode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => GameMode::STD,
            1 => GameMode::TKO,
            2 => GameMode::CTB,
            3 => GameMode::MNA,
            _ => GameMode::STD,
        }
    }
}

impl Default for GameMode {
    #[inline]
    fn default() -> Self {
        Self::STD
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::STD => f.write_str("osu"),
            Self::TKO => f.write_str("taiko"),
            Self::CTB => f.write_str("fruits"),
            Self::MNA => f.write_str("mania"),
        }
    }
}

struct ModeVisitor;

impl<'de> Visitor<'de> for ModeVisitor {
    type Value = GameMode;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a u8 or a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let mode = match v {
            "0" | "osu" | "osu!" => GameMode::STD,
            "1" | "taiko" | "tko" => GameMode::TKO,
            "2" | "ctb" | "fruits" => GameMode::CTB,
            "3" | "mania" | "mna" => GameMode::MNA,
            _ => {
                return Err(Error::invalid_value(
                    Unexpected::Str(v),
                    &r#""0", "osu", "osu!", "1", "taiko", "tko", "2", "ctb", "fruits", "3", "mania", or "mna""#,
                ))
            }
        };

        Ok(mode)
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(GameMode::STD),
            1 => Ok(GameMode::TKO),
            2 => Ok(GameMode::CTB),
            3 => Ok(GameMode::MNA),
            _ => Err(Error::invalid_value(
                Unexpected::Unsigned(v),
                &"0, 1, 2, or 3",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for GameMode {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(ModeVisitor)
    }
}

impl Serialize for GameMode {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(*self as u8)
    }
}
