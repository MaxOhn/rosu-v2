use std::{
    fmt::{Formatter, Result as FmtResult},
    marker::PhantomData,
};

use serde::{
    de::{Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use time::format_description::{
    modifier::{Day, Month, Year},
    Component, FormatItem,
};

const DATE_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Year(Year::default())),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Month(Month::default())),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Day(Day::default())),
];

pub(super) mod datetime {

    use serde::Deserializer;
    use time::{serde::rfc3339, OffsetDateTime};

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<OffsetDateTime, D::Error> {
        rfc3339::deserialize(d)
    }

    #[cfg(feature = "serialize")]
    pub fn serialize<S: serde::ser::Serializer>(
        datetime: &OffsetDateTime,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        rfc3339::serialize(datetime, s)
    }
}

pub(super) mod option_datetime {

    use serde::Deserializer;
    use time::{serde::rfc3339, OffsetDateTime};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        rfc3339::option::deserialize(d)
    }

    #[cfg(feature = "serialize")]
    #[allow(clippy::ref_option, reason = "required by serde")]
    pub fn serialize<S: serde::ser::Serializer>(
        datetime: &Option<OffsetDateTime>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        rfc3339::option::serialize(datetime, s)
    }
}

pub(super) mod adjust_acc {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
        let acc = <f32 as Deserialize>::deserialize(d)?;

        Ok(100.0 * acc)
    }

    #[cfg(feature = "serialize")]
    // Required to take a reference by serde
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: serde::ser::Serializer>(f: &f32, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_f32(*f / 100.0)
    }
}

pub(super) mod from_option {

    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>, T>(d: D) -> Result<T, D::Error>
    where
        T: Default + Deserialize<'de>,
    {
        Option::<T>::deserialize(d).map(Option::unwrap_or_default)
    }
}

pub(super) mod date {
    use std::fmt;

    use serde::{
        de::{Error, SeqAccess, Visitor},
        Deserializer,
    };
    use time::Date;

    use super::DATE_FORMAT;

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Date, D::Error> {
        d.deserialize_any(DateVisitor)
    }

    #[cfg(feature = "serialize")]
    // Required to take a reference by serde
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: serde::ser::Serializer>(date: &Date, s: S) -> Result<S::Ok, S::Error> {
        use serde::Serialize;

        (date.year(), date.ordinal()).serialize(s)
    }

    pub(super) struct DateVisitor;

    impl<'de> Visitor<'de> for DateVisitor {
        type Value = Date;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a `Date`")
        }

        #[inline]
        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            Date::parse(v, DATE_FORMAT).map_err(Error::custom)
        }

        #[inline]
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let year = seq
                .next_element()?
                .ok_or_else(|| Error::custom("expected year"))?;

            let ordinal = seq
                .next_element()?
                .ok_or_else(|| Error::custom("expected day of the year"))?;

            Date::from_ordinal_date(year, ordinal).map_err(Error::custom)
        }
    }
}

#[doc(hidden)]
pub struct DeserializedList<T>(pub(crate) Vec<T>);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for DeserializedList<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ListVisitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for ListVisitor<T> {
            type Value = Vec<T>;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("a struct with a single list field")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                const ERR: &str = "struct must contain exactly one field";

                let _: IgnoredAny = map.next_key()?.ok_or_else(|| DeError::custom(ERR))?;
                let list = map.next_value()?;

                if map.next_key::<IgnoredAny>()?.is_some() {
                    return Err(DeError::custom(ERR));
                }

                Ok(list)
            }
        }

        d.deserialize_map(ListVisitor(PhantomData)).map(Self)
    }
}
