#![cfg(feature = "rkyv")]

use std::{hint::unreachable_unchecked, marker::PhantomData, ptr};

use chrono::{Date, DateTime, Datelike, TimeZone, Utc};
use rkyv::{
    option::ArchivedOption,
    out_field,
    ser::{ScratchSpace, Serializer},
    string::{ArchivedString, StringResolver},
    vec::{ArchivedVec, VecResolver},
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Archived, Fallible, Resolver, Serialize,
};

use crate::prelude::{CountryCode, Username};

pub struct Map<Archivable> {
    phantom: PhantomData<Archivable>,
}

impl<A, O> ArchiveWith<Vec<O>> for Map<A>
where
    A: ArchiveWith<O>,
{
    type Archived = ArchivedVec<<A as ArchiveWith<O>>::Archived>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &Vec<O>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_len(field.len(), pos, resolver, out)
    }
}

impl<A, O, S> SerializeWith<Vec<O>, S> for Map<A>
where
    S: Fallible + ScratchSpace + Serializer,
    A: ArchiveWith<O> + SerializeWith<O, S>,
{
    fn serialize_with(field: &Vec<O>, s: &mut S) -> Result<Self::Resolver, S::Error> {
        // Wrapper for O so that we have an Archive and Serialize implementation
        // and ArchivedVec::serialize_from_* is happy about the bound constraints
        struct RefWrapper<'o, A, O>(&'o O, PhantomData<A>);

        impl<A: ArchiveWith<O>, O> Archive for RefWrapper<'_, A, O> {
            type Archived = <A as ArchiveWith<O>>::Archived;
            type Resolver = <A as ArchiveWith<O>>::Resolver;

            unsafe fn resolve(
                &self,
                pos: usize,
                resolver: Self::Resolver,
                out: *mut Self::Archived,
            ) {
                A::resolve_with(self.0, pos, resolver, out)
            }
        }

        impl<A, O, S> Serialize<S> for RefWrapper<'_, A, O>
        where
            A: ArchiveWith<O> + SerializeWith<O, S>,
            S: Fallible + Serializer,
        {
            fn serialize(&self, s: &mut S) -> Result<Self::Resolver, S::Error> {
                A::serialize_with(self.0, s)
            }
        }

        let iter = field
            .iter()
            .map(|value| RefWrapper::<'_, A, O>(value, PhantomData));

        ArchivedVec::serialize_from_iter(iter, s)
    }
}

impl<A, O, D> DeserializeWith<ArchivedVec<<A as ArchiveWith<O>>::Archived>, Vec<O>, D> for Map<A>
where
    A: ArchiveWith<O> + DeserializeWith<<A as ArchiveWith<O>>::Archived, O, D>,
    D: Fallible,
{
    fn deserialize_with(
        field: &ArchivedVec<<A as ArchiveWith<O>>::Archived>,
        d: &mut D,
    ) -> Result<Vec<O>, D::Error> {
        field
            .iter()
            .map(|value| <A as DeserializeWith<_, _, D>>::deserialize_with(value, d))
            .collect()
    }
}

// ##### wrapper for Options #####

// Copy-paste from Option's impls for the most part
impl<A, O> ArchiveWith<Option<O>> for Map<A>
where
    A: ArchiveWith<O>,
{
    type Archived = ArchivedOption<<A as ArchiveWith<O>>::Archived>;
    type Resolver = Option<<A as ArchiveWith<O>>::Resolver>;

    unsafe fn resolve_with(
        field: &Option<O>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        match resolver {
            None => {
                let out = out.cast::<ArchivedOptionVariantNone>();
                ptr::addr_of_mut!((*out).0).write(ArchivedOptionTag::None);
            }
            Some(resolver) => {
                let out = out.cast::<ArchivedOptionVariantSome<<A as ArchiveWith<O>>::Archived>>();
                ptr::addr_of_mut!((*out).0).write(ArchivedOptionTag::Some);

                let value = if let Some(value) = field.as_ref() {
                    value
                } else {
                    unreachable_unchecked();
                };

                let (fp, fo) = out_field!(out.1);
                A::resolve_with(value, pos + fp, resolver, fo);
            }
        }
    }
}

impl<A, O, S> SerializeWith<Option<O>, S> for Map<A>
where
    S: Fallible,
    A: ArchiveWith<O> + SerializeWith<O, S>,
{
    fn serialize_with(field: &Option<O>, s: &mut S) -> Result<Self::Resolver, S::Error> {
        field
            .as_ref()
            .map(|value| A::serialize_with(value, s))
            .transpose()
    }
}

impl<A, O, D> DeserializeWith<ArchivedOption<<A as ArchiveWith<O>>::Archived>, Option<O>, D>
    for Map<A>
where
    D: Fallible,
    A: ArchiveWith<O> + DeserializeWith<<A as ArchiveWith<O>>::Archived, O, D>,
{
    fn deserialize_with(
        field: &ArchivedOption<<A as ArchiveWith<O>>::Archived>,
        d: &mut D,
    ) -> Result<Option<O>, D::Error> {
        match field {
            ArchivedOption::Some(value) => Ok(Some(A::deserialize_with(value, d)?)),
            ArchivedOption::None => Ok(None),
        }
    }
}

#[repr(u8)]
enum ArchivedOptionTag {
    None,
    Some,
}

#[repr(C)]
struct ArchivedOptionVariantNone(ArchivedOptionTag);

#[repr(C)]
struct ArchivedOptionVariantSome<T>(ArchivedOptionTag, T);

pub struct CountryCodeWrapper;

impl ArchiveWith<CountryCode> for CountryCodeWrapper {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    #[inline]
    unsafe fn resolve_with(
        field: &CountryCode,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedString::resolve_from_str(field.as_str(), pos, resolver, out);
    }
}

impl<S: Fallible + Serializer> SerializeWith<CountryCode, S> for CountryCodeWrapper {
    #[inline]
    fn serialize_with(field: &CountryCode, s: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(field.as_str(), s)
    }
}

impl<D: Fallible> DeserializeWith<ArchivedString, CountryCode, D> for CountryCodeWrapper {
    #[inline]
    fn deserialize_with(field: &ArchivedString, _: &mut D) -> Result<CountryCode, D::Error> {
        Ok(CountryCode::from_str(field.as_str()))
    }
}

pub struct UsernameWrapper;

impl ArchiveWith<Username> for UsernameWrapper {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    #[inline]
    unsafe fn resolve_with(
        field: &Username,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedString::resolve_from_str(field.as_str(), pos, resolver, out);
    }
}

impl<S: Fallible + Serializer> SerializeWith<Username, S> for UsernameWrapper {
    #[inline]
    fn serialize_with(field: &Username, s: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(field.as_str(), s)
    }
}

impl<D: Fallible> DeserializeWith<ArchivedString, Username, D> for UsernameWrapper {
    #[inline]
    fn deserialize_with(field: &ArchivedString, _: &mut D) -> Result<Username, D::Error> {
        Ok(Username::from_str(field.as_str()))
    }
}

pub type UsernameMap = Map<UsernameWrapper>;
pub type UsernameMapMap = Map<Map<UsernameWrapper>>;

pub struct DateTimeWrapper;

impl ArchiveWith<DateTime<Utc>> for DateTimeWrapper {
    type Archived = Archived<i64>;
    type Resolver = Resolver<i64>;

    #[inline]
    unsafe fn resolve_with(
        field: &DateTime<Utc>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        Archive::resolve(&field.timestamp_millis(), pos, resolver, out);
    }
}

impl<D: Fallible> DeserializeWith<i64, DateTime<Utc>, D> for DateTimeWrapper {
    #[inline]
    fn deserialize_with(field: &Archived<i64>, _: &mut D) -> Result<DateTime<Utc>, D::Error> {
        Ok(Utc.timestamp_millis(*field))
    }
}

impl<S: Fallible> SerializeWith<DateTime<Utc>, S> for DateTimeWrapper {
    #[inline]
    fn serialize_with(_: &DateTime<Utc>, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

pub type DateTimeMap = Map<DateTimeWrapper>;

pub struct DateWrapper;

pub struct ArchivedDateUtc {
    year: Archived<i32>,
    ordinal: Archived<u32>,
}

pub struct DateUtcResolver {
    year: Resolver<i32>,
    ordinal: Resolver<u32>,
}

impl ArchiveWith<Date<Utc>> for DateWrapper {
    type Archived = ArchivedDateUtc;
    type Resolver = DateUtcResolver;

    #[inline]
    unsafe fn resolve_with(
        field: &Date<Utc>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        let (fp, fo) = {
            let fo = (&mut (*out).year) as *mut i32;
            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
        };
        #[allow(clippy::unit_arg)]
        field.year().resolve(pos + fp, resolver.year, fo);

        let (fp, fo) = {
            let fo = (&mut (*out).ordinal) as *mut u32;
            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
        };
        #[allow(clippy::unit_arg)]
        field.ordinal().resolve(pos + fp, resolver.ordinal, fo);
    }
}

impl<S: Fallible> SerializeWith<Date<Utc>, S> for DateWrapper {
    #[inline]
    fn serialize_with(field: &Date<Utc>, s: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(DateUtcResolver {
            year: Serialize::<S>::serialize(&field.year(), s)?,
            ordinal: Serialize::<S>::serialize(&field.ordinal(), s)?,
        })
    }
}

impl<D: Fallible> DeserializeWith<ArchivedDateUtc, Date<Utc>, D> for DateWrapper {
    #[inline]
    fn deserialize_with(field: &ArchivedDateUtc, _: &mut D) -> Result<Date<Utc>, D::Error> {
        Ok(Utc.yo(field.year, field.ordinal))
    }
}
