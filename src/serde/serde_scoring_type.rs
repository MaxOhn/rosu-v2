use crate::model::ScoringType;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

struct ScoringTypeVisitor;

impl<'de> Visitor<'de> for ScoringTypeVisitor {
    type Value = ScoringType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8 or a stringified number")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "0" => Ok(ScoringType::Score),
            "1" => Ok(ScoringType::Accuracy),
            "2" => Ok(ScoringType::Combo),
            "3" => Ok(ScoringType::ScoreV2),
            _ => Err(Error::invalid_value(
                Unexpected::Str(v),
                &r#""0", "1", "2", or "3""#,
            )),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(ScoringType::from(v as u8))
    }
}

impl<'de> Deserialize<'de> for ScoringType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(ScoringTypeVisitor)
    }
}
