use crate::model::Genre;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

struct GenreVisitor;

impl<'de> Visitor<'de> for GenreVisitor {
    type Value = Genre;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an u8 or a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "0" | "any" => Ok(Genre::Any),
            "1" | "unspecified" => Ok(Genre::Unspecified),
            "2" | "videogame" => Ok(Genre::VideoGame),
            "3" | "anime" => Ok(Genre::Anime),
            "4" | "rock" => Ok(Genre::Rock),
            "5" | "pop" => Ok(Genre::Pop),
            "6" | "other" => Ok(Genre::Other),
            "7" | "novelty" => Ok(Genre::Novelty),
            "9" | "hiphip" => Ok(Genre::HipHop),
            "10" | "electronic" => Ok(Genre::Electronic),
            "11" | "metal" => Ok(Genre::Metal),
            "12" | "classical" => Ok(Genre::Classical),
            "13" | "folk" => Ok(Genre::Folk),
            "14" | "jazz" => Ok(Genre::Jazz),
            _ => Err(Error::invalid_value(
                Unexpected::Str(v),
                &r#"
            "0", "any",
            "1"m "unspecified",
            "2", "videogame",
            "3", "anime",
            "4", "rock",
            "5", "pop",
            "6", "other",
            "7", "novelty",
            "9", "hiphop",
            "10", "electronic",
            "11", "metal",
            "12", "classical",
            "13", "folk",
            "14", or "jazz"
            "#,
            )),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Genre::from(v as u8))
    }
}

impl<'de> Deserialize<'de> for Genre {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(GenreVisitor)
    }
}
