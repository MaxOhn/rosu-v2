use crate::model::Team;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

struct TeamVisitor;

impl<'de> Visitor<'de> for TeamVisitor {
    type Value = Team;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8 or a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "0" | "none" => Ok(Team::None),
            "1" | "blue" => Ok(Team::Blue),
            "2" | "red" => Ok(Team::Red),
            _ => Err(Error::invalid_value(
                Unexpected::Str(v),
                &r#""0", "none", "1", "blue", "2", or "red""#,
            )),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Team::from(v as u8))
    }
}

impl<'de> Deserialize<'de> for Team {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(TeamVisitor)
    }
}
