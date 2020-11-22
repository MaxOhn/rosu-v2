use serde::{
    de::{Error, Visitor},
    Deserializer,
};
use std::{fmt, str::FromStr};

struct U64Visitor;

impl<'de> Visitor<'de> for U64Visitor {
    type Value = Option<u64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a u64, a stringified number, or null")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let v = i64::from_str(v).map_err(Error::custom)?;
        Ok(Some(v.max(0) as u64))
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Some(v))
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(Some(v.max(0) as u64))
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(Self)
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

pub(crate) fn to_maybe_u64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u64>, D::Error> {
    d.deserialize_option(U64Visitor)
}

pub(crate) fn to_u64<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    Ok(d.deserialize_option(U64Visitor)?.unwrap_or_else(|| {
        debug!("WARN: Serializing None to u64 as 0");
        0
    }))
}
