use serde::{
    de::{Error, Unexpected, Visitor},
    Deserializer,
};
use std::{fmt, str::FromStr};

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = Option<bool>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a bool, a stringified bool, null, or 0 or 1 in either number, string or char format",
        )
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        if let Ok(b) = bool::from_str(v) {
            return Ok(Some(b));
        }
        u8::from_str(v)
            .map(|n| match n {
                0 => Ok(Some(false)),
                1 => Ok(Some(true)),
                _ => Err(Error::invalid_value(
                    Unexpected::Unsigned(n as u64),
                    &"0 or 1",
                )),
            })
            .map_err(|_| {
                Error::invalid_value(Unexpected::Str(v), &r#""true", "false", "0", or "1""#)
            })?
    }

    fn visit_char<E: Error>(self, v: char) -> Result<Self::Value, E> {
        match v {
            '0' => Ok(Some(false)),
            '1' => Ok(Some(true)),
            _ => Err(Error::invalid_value(Unexpected::Char(v), &"'0' or '1'")),
        }
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Some(v))
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(Self)
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

pub(crate) fn to_maybe_bool<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
    d.deserialize_option(BoolVisitor)
}

pub(crate) fn to_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    Ok(d.deserialize_any(BoolVisitor)?.unwrap_or_else(|| {
        debug!("WARN: Serializing None to bool as false");
        false
    }))
}
