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

const OFFSET_DATE_TIME_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Compound(DATE_FORMAT),
    FormatItem::Literal(b"T"),
    FormatItem::Compound(TIME_FORMAT),
    FormatItem::Literal(b"Z"),
];

const UTC_OFFSET_FORMAT: &[FormatItem<'_>] = &[
    FormatItem::Component(Component::OffsetHour(OffsetHour::default())),
    FormatItem::Literal(b":"),
    FormatItem::Component(Component::OffsetMinute(OffsetMinute::default())),
];

const OFFSET_DATE_TIME_FORMAT_FULL: &[FormatItem<'_>] = &[
    FormatItem::Compound(DATE_FORMAT),
    FormatItem::Literal(b"T"),
    FormatItem::Compound(TIME_FORMAT),
    FormatItem::Compound(UTC_OFFSET_FORMAT),
];

pub(super) mod datetime {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serialize, Serializer,
    };
    use time::{OffsetDateTime, PrimitiveDateTime};

    use super::OFFSET_DATE_TIME_FORMAT;

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<OffsetDateTime, D::Error> {
        d.deserialize_any(DateTimeVisitor)
    }

    pub fn serialize<S: Serializer>(datetime: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error> {
        datetime.unix_timestamp_nanos().serialize(s)
    }

    pub(super) struct DateTimeVisitor;

    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = OffsetDateTime;

        #[inline]
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a `PrimitiveDateTime`")
        }

        #[inline]
        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            PrimitiveDateTime::parse(v, OFFSET_DATE_TIME_FORMAT)
                .map(PrimitiveDateTime::assume_utc)
                .map_err(Error::custom)
        }

        #[inline]
        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            self.visit_i128(v as i128)
        }

        #[inline]
        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            self.visit_i128(v as i128)
        }

        #[inline]
        fn visit_i128<E: Error>(self, v: i128) -> Result<Self::Value, E> {
            OffsetDateTime::from_unix_timestamp_nanos(v).map_err(Error::custom)
        }
    }
}

pub(super) mod datetime_full {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serialize, Serializer,
    };
    use time::OffsetDateTime;

    use super::OFFSET_DATE_TIME_FORMAT_FULL;

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<OffsetDateTime, D::Error> {
        d.deserialize_any(DateTimeVisitor)
    }

    pub fn serialize<S: Serializer>(datetime: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error> {
        datetime.unix_timestamp_nanos().serialize(s)
    }

    pub(super) struct DateTimeVisitor;

    impl<'de> Visitor<'de> for DateTimeVisitor {
        type Value = OffsetDateTime;

        #[inline]
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("an `OffsetDateTime`")
        }

        #[inline]
        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            OffsetDateTime::parse(v, OFFSET_DATE_TIME_FORMAT_FULL).map_err(Error::custom)
        }

        #[inline]
        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            self.visit_i128(v as i128)
        }

        #[inline]
        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            self.visit_i128(v as i128)
        }

        #[inline]
        fn visit_i128<E: Error>(self, v: i128) -> Result<Self::Value, E> {
            OffsetDateTime::from_unix_timestamp_nanos(v).map_err(Error::custom)
        }
    }
}

pub(super) mod option_datetime {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serialize, Serializer,
    };
    use time::OffsetDateTime;

    use super::datetime::DateTimeVisitor;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        d.deserialize_option(OptionDateTimeVisitor)
    }

    pub fn serialize<S: Serializer>(
        datetime: &Option<OffsetDateTime>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        datetime
            .map(OffsetDateTime::unix_timestamp_nanos)
            .serialize(s)
    }

    struct OptionDateTimeVisitor;

    impl<'de> Visitor<'de> for OptionDateTimeVisitor {
        type Value = Option<OffsetDateTime>;

        #[inline]
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("an optional `PrimitiveDateTime`")
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_any(DateTimeVisitor).map(Some)
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

pub(super) mod option_datetime_full {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serialize, Serializer,
    };
    use time::OffsetDateTime;

    use super::datetime_full::DateTimeVisitor;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        d.deserialize_option(OptionDateTimeVisitor)
    }

    pub fn serialize<S: Serializer>(
        datetime: &Option<OffsetDateTime>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        datetime
            .map(OffsetDateTime::unix_timestamp_nanos)
            .serialize(s)
    }

    struct OptionDateTimeVisitor;

    impl<'de> Visitor<'de> for OptionDateTimeVisitor {
        type Value = Option<OffsetDateTime>;

        #[inline]
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("an optional `OffsetDateTime`")
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_any(DateTimeVisitor).map(Some)
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
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
        let acc = <f32 as Deserialize>::deserialize(d)?;

        Ok(100.0 * acc)
    }

    pub fn serialize<S: Serializer>(f: &f32, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_f32(*f / 100.0)
    }
}

pub(super) mod date {
    use std::fmt;

    use serde::{
        de::{Error, SeqAccess, Visitor},
        Deserializer, Serialize, Serializer,
    };
    use time::Date;

    use super::DATE_FORMAT;

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Date, D::Error> {
        d.deserialize_any(DateVisitor)
    }

    pub fn serialize<S: Serializer>(date: &Date, s: S) -> Result<S::Ok, S::Error> {
        (date.year(), date.ordinal()).serialize(s)
    }

    pub(super) struct DateVisitor;

    impl<'de> Visitor<'de> for DateVisitor {
        type Value = Date;

        #[inline]
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
