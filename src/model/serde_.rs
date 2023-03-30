use time::format_description::{
    modifier::{Day, Hour, Minute, Month, OffsetHour, OffsetMinute, Second, Year},
    Component, FormatItem,
};

const DATE_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Year(Year::default())),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Month(Month::default())),
    FormatItem::Literal(b"-"),
    FormatItem::Component(Component::Day(Day::default())),
];

const TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::Hour(<Hour>::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::Minute(<Minute>::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::Second(<Second>::default())),
];

const PRIMITIVE_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Compound(DATE_FORMAT),
    FormatItem::Literal(b"T"),
    FormatItem::Compound(TIME_FORMAT),
];

const OFFSET_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::OffsetHour(OffsetHour::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::OffsetMinute(OffsetMinute::default())),
];

#[cfg(feature = "serialize")]
const OFFSET_DATETIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Compound(PRIMITIVE_FORMAT),
    FormatItem::Compound(OFFSET_FORMAT),
];

pub(super) mod datetime {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer,
    };
    use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

    use super::{OFFSET_FORMAT, PRIMITIVE_FORMAT};

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<OffsetDateTime, D::Error> {
        d.deserialize_str(DateTimeVisitor)
    }

    #[cfg(feature = "serialize")]
    pub fn serialize<S: serde::ser::Serializer>(
        datetime: &OffsetDateTime,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::Serialize;

        datetime
            .format(&super::OFFSET_DATETIME_FORMAT)
            .expect("incorrect format")
            .serialize(s)
    }

    pub(super) struct DateTimeVisitor;

    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = OffsetDateTime;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a datetime string")
        }

        #[inline]
        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            if v.len() < 19 {
                return Err(Error::custom(format!(
                    "string too short for a datetime: `{v}`"
                )));
            }

            let (prefix, suffix) = v.split_at(19);

            let primitive =
                PrimitiveDateTime::parse(prefix, PRIMITIVE_FORMAT).map_err(Error::custom)?;

            let offset = if suffix == "Z" {
                UtcOffset::UTC
            } else {
                UtcOffset::parse(suffix, OFFSET_FORMAT).map_err(Error::custom)?
            };

            Ok(primitive.assume_offset(offset))
        }
    }
}

pub(super) mod option_datetime {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer,
    };
    use time::OffsetDateTime;

    use super::datetime::DateTimeVisitor;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        d.deserialize_option(OptionDateTimeVisitor)
    }

    #[cfg(feature = "serialize")]
    pub fn serialize<S: serde::ser::Serializer>(
        datetime: &Option<OffsetDateTime>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::Serialize;

        datetime
            .map(|datetime| {
                datetime
                    .format(&super::OFFSET_DATETIME_FORMAT)
                    .expect("incorrect format")
            })
            .serialize(s)
    }

    struct OptionDateTimeVisitor;

    impl<'de> Visitor<'de> for OptionDateTimeVisitor {
        type Value = Option<OffsetDateTime>;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("an optional datetime string")
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_str(DateTimeVisitor).map(Some)
        }

        #[inline]
        fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
            self.visit_unit()
        }

        #[inline]
        fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }
}

pub(super) mod adjust_acc {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
        let acc = <f32 as Deserialize>::deserialize(d)?;

        Ok(100.0 * acc)
    }

    #[cfg(feature = "serialize")]
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
