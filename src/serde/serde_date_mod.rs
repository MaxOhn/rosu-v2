pub(crate) mod serde_maybe_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<DateTime<Utc>>, D::Error> {
        let v: &str = match Deserialize::deserialize(d) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };
        Utc.datetime_from_str(v, FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}

pub(crate) mod serde_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer};

    #[cfg(feature = "serialize")]
    use serde::Serializer;

    const FORMAT: &str = "%F %T";

    #[cfg(feature = "serialize")]
    pub fn serialize<S: Serializer>(date: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error> {
        let v = date.format(FORMAT).to_string();
        s.serialize_str(&v)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<Utc>, D::Error> {
        let v = String::deserialize(d)?;
        Utc.datetime_from_str(&v, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
