use crate::model::Language;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

struct LanguageVisitor;

impl<'de> Visitor<'de> for LanguageVisitor {
    type Value = Language;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u8 or a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "0" | "any" => Ok(Language::Any),
            "1" | "other" => Ok(Language::Other),
            "2" | "english" => Ok(Language::English),
            "3" | "japanese" => Ok(Language::Japanese),
            "4" | "chinese" => Ok(Language::Chinese),
            "5" | "instrumental" => Ok(Language::Instrumental),
            "6" | "korean" => Ok(Language::Korean),
            "7" | "french" => Ok(Language::French),
            "8" | "german" => Ok(Language::German),
            "9" | "swedish" => Ok(Language::Swedish),
            "10" | "spanish" => Ok(Language::Spanish),
            "11" | "italian" => Ok(Language::Italian),
            "12" | "russian" => Ok(Language::Russian),
            "13" | "polish" => Ok(Language::Polish),
            "14" | "unspecified" => Ok(Language::Unspecified),
            _ => Err(Error::invalid_value(
                Unexpected::Str(v),
                &r#"
            "0", "any",
            "1", "other",
            "2", "english",
            "3", "japanese",
            "4", "chinese",
            "5", "instrumental",
            "6", "korean",
            "7", "french",
            "8", "german",
            "9", "swedish",
            "10", "spanish",
            "11", "italian",
            "12", "russian",
            "13", "polish",
            "14", or "unspecified"
            "#,
            )),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Language::from(v as u8))
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(LanguageVisitor)
    }
}
