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

use crate::prelude::{
    CountryCode, Genre, Language, RankStatus, ScoringType, Team, TeamType, Username,
};

///An archived [`RankStatus`]
#[repr(u8)]
pub enum ArchivedRankStatus {
    ///The archived counterpart of [`RankStatus::Graveyard`]
    #[allow(dead_code)]
    Graveyard,
    ///The archived counterpart of [`RankStatus::WIP`]
    #[allow(dead_code)]
    WIP,
    ///The archived counterpart of [`RankStatus::Pending`]
    #[allow(dead_code)]
    Pending,
    ///The archived counterpart of [`RankStatus::Ranked`]
    #[allow(dead_code)]
    Ranked,
    ///The archived counterpart of [`RankStatus::Approved`]
    #[allow(dead_code)]
    Approved,
    ///The archived counterpart of [`RankStatus::Qualified`]
    #[allow(dead_code)]
    Qualified,
    ///The archived counterpart of [`RankStatus::Loved`]
    #[allow(dead_code)]
    Loved,
}

///The resolver for an archived [`RankStatus`]
pub enum RankStatusResolver {
    ///The resolver for [`RankStatus::Graveyard`]
    #[allow(dead_code)]
    Graveyard,
    ///The resolver for [`RankStatus::WIP`]
    #[allow(dead_code)]
    WIP,
    ///The resolver for [`RankStatus::Pending`]
    #[allow(dead_code)]
    Pending,
    ///The resolver for [`RankStatus::Ranked`]
    #[allow(dead_code)]
    Ranked,
    ///The resolver for [`RankStatus::Approved`]
    #[allow(dead_code)]
    Approved,
    ///The resolver for [`RankStatus::Qualified`]
    #[allow(dead_code)]
    Qualified,
    ///The resolver for [`RankStatus::Loved`]
    #[allow(dead_code)]
    Loved,
}

const _: () = {
    #[repr(u8)]
    enum ArchivedTag {
        Graveyard,
        Wip,
        Pending,
        Ranked,
        Approved,
        Qualified,
        Loved,
    }

    impl Archive for RankStatus {
        type Archived = ArchivedRankStatus;
        type Resolver = RankStatusResolver;

        #[inline]
        unsafe fn resolve(&self, _: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
            match resolver {
                RankStatusResolver::Graveyard => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Graveyard);
                }
                RankStatusResolver::WIP => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Wip);
                }
                RankStatusResolver::Pending => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Pending);
                }
                RankStatusResolver::Ranked => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Ranked);
                }
                RankStatusResolver::Approved => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Approved);
                }
                RankStatusResolver::Qualified => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Qualified);
                }
                RankStatusResolver::Loved => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Loved);
                }
            }
        }
    }
};

///An archived [`Genre`]
#[repr(u8)]
pub enum ArchivedGenre {
    ///The archived counterpart of [`Genre::Any`]
    #[allow(dead_code)]
    Any,
    ///The archived counterpart of [`Genre::Unspecified`]
    #[allow(dead_code)]
    Unspecified,
    ///The archived counterpart of [`Genre::VideoGame`]
    #[allow(dead_code)]
    VideoGame,
    ///The archived counterpart of [`Genre::Anime`]
    #[allow(dead_code)]
    Anime,
    ///The archived counterpart of [`Genre::Rock`]
    #[allow(dead_code)]
    Rock,
    ///The archived counterpart of [`Genre::Pop`]
    #[allow(dead_code)]
    Pop,
    ///The archived counterpart of [`Genre::Other`]
    #[allow(dead_code)]
    Other,
    ///The archived counterpart of [`Genre::Novelty`]
    #[allow(dead_code)]
    Novelty,
    ///The archived counterpart of [`Genre::HipHop`]
    #[allow(dead_code)]
    HipHop,
    ///The archived counterpart of [`Genre::Electronic`]
    #[allow(dead_code)]
    Electronic,
    ///The archived counterpart of [`Genre::Metal`]
    #[allow(dead_code)]
    Metal,
    ///The archived counterpart of [`Genre::Classical`]
    #[allow(dead_code)]
    Classical,
    ///The archived counterpart of [`Genre::Folk`]
    #[allow(dead_code)]
    Folk,
    ///The archived counterpart of [`Genre::Jazz`]
    #[allow(dead_code)]
    Jazz,
}

///The resolver for an archived [`Genre`]
pub enum GenreResolver {
    ///The resolver for [`Genre::Any`]
    #[allow(dead_code)]
    Any,
    ///The resolver for [`Genre::Unspecified`]
    #[allow(dead_code)]
    Unspecified,
    ///The resolver for [`Genre::VideoGame`]
    #[allow(dead_code)]
    VideoGame,
    ///The resolver for [`Genre::Anime`]
    #[allow(dead_code)]
    Anime,
    ///The resolver for [`Genre::Rock`]
    #[allow(dead_code)]
    Rock,
    ///The resolver for [`Genre::Pop`]
    #[allow(dead_code)]
    Pop,
    ///The resolver for [`Genre::Other`]
    #[allow(dead_code)]
    Other,
    ///The resolver for [`Genre::Novelty`]
    #[allow(dead_code)]
    Novelty,
    ///The resolver for [`Genre::HipHop`]
    #[allow(dead_code)]
    HipHop,
    ///The resolver for [`Genre::Electronic`]
    #[allow(dead_code)]
    Electronic,
    ///The resolver for [`Genre::Metal`]
    #[allow(dead_code)]
    Metal,
    ///The resolver for [`Genre::Classical`]
    #[allow(dead_code)]
    Classical,
    ///The resolver for [`Genre::Folk`]
    #[allow(dead_code)]
    Folk,
    ///The resolver for [`Genre::Jazz`]
    #[allow(dead_code)]
    Jazz,
}

const _: () = {
    #[repr(u8)]
    enum ArchivedTag {
        Any,
        Unspecified,
        VideoGame,
        Anime,
        Rock,
        Pop,
        Other,
        Novelty,
        HipHop,
        Electronic,
        Metal,
        Classical,
        Folk,
        Jazz,
    }
    impl Archive for Genre {
        type Archived = ArchivedGenre;
        type Resolver = GenreResolver;

        #[inline]
        unsafe fn resolve(&self, _: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
            match resolver {
                GenreResolver::Any => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Any);
                }
                GenreResolver::Unspecified => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Unspecified);
                }
                GenreResolver::VideoGame => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::VideoGame);
                }
                GenreResolver::Anime => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Anime);
                }
                GenreResolver::Rock => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Rock);
                }
                GenreResolver::Pop => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Pop);
                }
                GenreResolver::Other => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Other);
                }
                GenreResolver::Novelty => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Novelty);
                }
                GenreResolver::HipHop => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::HipHop);
                }
                GenreResolver::Electronic => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Electronic);
                }
                GenreResolver::Metal => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Metal);
                }
                GenreResolver::Classical => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Classical);
                }
                GenreResolver::Folk => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Folk);
                }
                GenreResolver::Jazz => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Jazz);
                }
            }
        }
    }
};

///An archived [`Language`]
#[repr(u8)]
pub enum ArchivedLanguage {
    ///The archived counterpart of [`Language::Any`]
    #[allow(dead_code)]
    Any,
    ///The archived counterpart of [`Language::Other`]
    #[allow(dead_code)]
    Other,
    ///The archived counterpart of [`Language::English`]
    #[allow(dead_code)]
    English,
    ///The archived counterpart of [`Language::Japanese`]
    #[allow(dead_code)]
    Japanese,
    ///The archived counterpart of [`Language::Chinese`]
    #[allow(dead_code)]
    Chinese,
    ///The archived counterpart of [`Language::Instrumental`]
    #[allow(dead_code)]
    Instrumental,
    ///The archived counterpart of [`Language::Korean`]
    #[allow(dead_code)]
    Korean,
    ///The archived counterpart of [`Language::French`]
    #[allow(dead_code)]
    French,
    ///The archived counterpart of [`Language::German`]
    #[allow(dead_code)]
    German,
    ///The archived counterpart of [`Language::Swedish`]
    #[allow(dead_code)]
    Swedish,
    ///The archived counterpart of [`Language::Spanish`]
    #[allow(dead_code)]
    Spanish,
    ///The archived counterpart of [`Language::Italian`]
    #[allow(dead_code)]
    Italian,
    ///The archived counterpart of [`Language::Russian`]
    #[allow(dead_code)]
    Russian,
    ///The archived counterpart of [`Language::Polish`]
    #[allow(dead_code)]
    Polish,
    ///The archived counterpart of [`Language::Unspecified`]
    #[allow(dead_code)]
    Unspecified,
}

///The resolver for an archived [`Language`]
pub enum LanguageResolver {
    ///The resolver for [`Language::Any`]
    #[allow(dead_code)]
    Any,
    ///The resolver for [`Language::Other`]
    #[allow(dead_code)]
    Other,
    ///The resolver for [`Language::English`]
    #[allow(dead_code)]
    English,
    ///The resolver for [`Language::Japanese`]
    #[allow(dead_code)]
    Japanese,
    ///The resolver for [`Language::Chinese`]
    #[allow(dead_code)]
    Chinese,
    ///The resolver for [`Language::Instrumental`]
    #[allow(dead_code)]
    Instrumental,
    ///The resolver for [`Language::Korean`]
    #[allow(dead_code)]
    Korean,
    ///The resolver for [`Language::French`]
    #[allow(dead_code)]
    French,
    ///The resolver for [`Language::German`]
    #[allow(dead_code)]
    German,
    ///The resolver for [`Language::Swedish`]
    #[allow(dead_code)]
    Swedish,
    ///The resolver for [`Language::Spanish`]
    #[allow(dead_code)]
    Spanish,
    ///The resolver for [`Language::Italian`]
    #[allow(dead_code)]
    Italian,
    ///The resolver for [`Language::Russian`]
    #[allow(dead_code)]
    Russian,
    ///The resolver for [`Language::Polish`]
    #[allow(dead_code)]
    Polish,
    ///The resolver for [`Language::Unspecified`]
    #[allow(dead_code)]
    Unspecified,
}

const _: () = {
    #[repr(u8)]
    enum ArchivedTag {
        Any,
        Other,
        English,
        Japanese,
        Chinese,
        Instrumental,
        Korean,
        French,
        German,
        Swedish,
        Spanish,
        Italian,
        Russian,
        Polish,
        Unspecified,
    }

    impl Archive for Language {
        type Archived = ArchivedLanguage;
        type Resolver = LanguageResolver;

        #[inline]
        unsafe fn resolve(&self, _: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
            match resolver {
                LanguageResolver::Any => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Any);
                }
                LanguageResolver::Other => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Other);
                }
                LanguageResolver::English => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::English);
                }
                LanguageResolver::Japanese => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Japanese);
                }
                LanguageResolver::Chinese => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Chinese);
                }
                LanguageResolver::Instrumental => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Instrumental);
                }
                LanguageResolver::Korean => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Korean);
                }
                LanguageResolver::French => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::French);
                }
                LanguageResolver::German => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::German);
                }
                LanguageResolver::Swedish => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Swedish);
                }
                LanguageResolver::Spanish => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Spanish);
                }
                LanguageResolver::Italian => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Italian);
                }
                LanguageResolver::Russian => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Russian);
                }
                LanguageResolver::Polish => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Polish);
                }
                LanguageResolver::Unspecified => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Unspecified);
                }
            }
        }
    }
};

///An archived [`ScoringType`]
#[repr(u8)]
pub enum ArchivedScoringType {
    ///The archived counterpart of [`ScoringType::Score`]
    #[allow(dead_code)]
    Score,
    ///The archived counterpart of [`ScoringType::Accuracy`]
    #[allow(dead_code)]
    Accuracy,
    ///The archived counterpart of [`ScoringType::Combo`]
    #[allow(dead_code)]
    Combo,
    ///The archived counterpart of [`ScoringType::ScoreV2`]
    #[allow(dead_code)]
    ScoreV2,
}

///The resolver for an archived [`ScoringType`]
pub enum ScoringTypeResolver {
    ///The resolver for [`ScoringType::Score`]
    #[allow(dead_code)]
    Score,
    ///The resolver for [`ScoringType::Accuracy`]
    #[allow(dead_code)]
    Accuracy,
    ///The resolver for [`ScoringType::Combo`]
    #[allow(dead_code)]
    Combo,
    ///The resolver for [`ScoringType::ScoreV2`]
    #[allow(dead_code)]
    ScoreV2,
}

const _: () = {
    #[repr(u8)]
    enum ArchivedTag {
        Score,
        Accuracy,
        Combo,
        ScoreV2,
    }

    impl Archive for ScoringType {
        type Archived = ArchivedScoringType;
        type Resolver = ScoringTypeResolver;

        #[inline]
        unsafe fn resolve(&self, _: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
            match resolver {
                ScoringTypeResolver::Score => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Score);
                }
                ScoringTypeResolver::Accuracy => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Accuracy);
                }
                ScoringTypeResolver::Combo => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Combo);
                }
                ScoringTypeResolver::ScoreV2 => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::ScoreV2);
                }
            }
        }
    }
};

///An archived [`Team`]
#[repr(u8)]
pub enum ArchivedTeam {
    ///The archived counterpart of [`Team::None`]
    #[allow(dead_code)]
    None,
    ///The archived counterpart of [`Team::Blue`]
    #[allow(dead_code)]
    Blue,
    ///The archived counterpart of [`Team::Red`]
    #[allow(dead_code)]
    Red,
}

///The resolver for an archived [`Team`]
pub enum TeamResolver {
    ///The resolver for [`Team::None`]
    #[allow(dead_code)]
    None,
    ///The resolver for [`Team::Blue`]
    #[allow(dead_code)]
    Blue,
    ///The resolver for [`Team::Red`]
    #[allow(dead_code)]
    Red,
}

const _: () = {
    #[repr(u8)]
    enum ArchivedTag {
        None,
        Blue,
        Red,
    }
    impl Archive for Team {
        type Archived = ArchivedTeam;
        type Resolver = TeamResolver;

        #[inline]
        unsafe fn resolve(&self, _: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
            match resolver {
                TeamResolver::None => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::None);
                }
                TeamResolver::Blue => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Blue);
                }
                TeamResolver::Red => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::Red);
                }
            }
        }
    }
};

///An archived [`TeamType`]
#[repr(u8)]
pub enum ArchivedTeamType {
    ///The archived counterpart of [`TeamType::HeadToHead`]
    #[allow(dead_code)]
    HeadToHead,
    ///The archived counterpart of [`TeamType::TagCoop`]
    #[allow(dead_code)]
    TagCoop,
    ///The archived counterpart of [`TeamType::TeamVS`]
    #[allow(dead_code)]
    TeamVS,
    ///The archived counterpart of [`TeamType::TagTeamVS`]
    #[allow(dead_code)]
    TagTeamVS,
}

///The resolver for an archived [`TeamType`]
pub enum TeamTypeResolver {
    ///The resolver for [`TeamType::HeadToHead`]
    #[allow(dead_code)]
    HeadToHead,
    ///The resolver for [`TeamType::TagCoop`]
    #[allow(dead_code)]
    TagCoop,
    ///The resolver for [`TeamType::TeamVS`]
    #[allow(dead_code)]
    TeamVS,
    ///The resolver for [`TeamType::TagTeamVS`]
    #[allow(dead_code)]
    TagTeamVS,
}

const _: () = {
    #[repr(u8)]
    enum ArchivedTag {
        HeadToHead,
        TagCoop,
        TeamVS,
        TagTeamVS,
    }
    impl Archive for TeamType {
        type Archived = ArchivedTeamType;
        type Resolver = TeamTypeResolver;

        #[inline]
        unsafe fn resolve(&self, _: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
            match resolver {
                TeamTypeResolver::HeadToHead => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::HeadToHead);
                }
                TeamTypeResolver::TagCoop => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::TagCoop);
                }
                TeamTypeResolver::TeamVS => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::TeamVS);
                }
                TeamTypeResolver::TagTeamVS => {
                    out.cast::<ArchivedTag>().write(ArchivedTag::TagTeamVS);
                }
            }
        }
    }
};

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
