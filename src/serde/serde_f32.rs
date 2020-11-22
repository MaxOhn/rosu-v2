use serde::{
    de::{Error, Visitor},
    Deserializer,
};
use std::{fmt, str::FromStr};

struct F32Visitor;

impl<'de> Visitor<'de> for F32Visitor {
    type Value = Option<f32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a f32, a stringified number, or null")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        f32::from_str(v).map(Some).map_err(Error::custom)
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(Some(v as f32))
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(Self)
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

pub(crate) fn to_maybe_f32<'de, D: Deserializer<'de>>(d: D) -> Result<Option<f32>, D::Error> {
    d.deserialize_option(F32Visitor)
}

pub(crate) fn to_f32<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
    Ok(d.deserialize_option(F32Visitor)?.unwrap_or_else(|| {
        debug!("WARN: Serializing None to f32 as 0.0");
        0.0
    }))
}
