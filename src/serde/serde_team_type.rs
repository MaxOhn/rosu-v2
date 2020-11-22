use crate::model::TeamType;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

struct TeamTypeVisitor;

impl<'de> Visitor<'de> for TeamTypeVisitor {
    type Value = TeamType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8 or a stringified number")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let team_type = match v {
            "0" => TeamType::HeadToHead,
            "1" => TeamType::TagCoop,
            "2" => TeamType::TeamVS,
            "3" => TeamType::TagTeamVS,
            _ => {
                return Err(Error::invalid_value(
                    Unexpected::Str(v),
                    &r#""0", "1", "2", or "3""#,
                ))
            }
        };
        Ok(team_type)
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(TeamType::from(v as u8))
    }
}

impl<'de> Deserialize<'de> for TeamType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(TeamTypeVisitor)
    }
}
