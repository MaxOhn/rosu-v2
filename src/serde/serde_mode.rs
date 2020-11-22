use crate::model::GameMode;

use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

struct ModeVisitor;

impl<'de> Visitor<'de> for ModeVisitor {
    type Value = GameMode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8 or a string")
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
                    &r#""0", "osu", "1", "taiko", "tko", "2",
                    "ctb", "fruits", "3", "mania", or "mna""#,
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
