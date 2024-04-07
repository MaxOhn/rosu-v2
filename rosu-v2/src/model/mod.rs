macro_rules! def_enum {
    (@BASE $type:tt { $($variant:ident = $n:literal,)* }) => {
        #[allow(clippy::upper_case_acronyms, missing_docs)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        #[repr(u8)]
        pub enum $type {
            $($variant = $n,)*
        }

        impl<'de> serde::Deserialize<'de> for $type {
            #[inline]
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                d.deserialize_option(super::EnumVisitor::<$type>::new())
            }
        }

        #[cfg(feature = "rkyv")]
        impl Archive for $type {
            type Archived = Self;
            type Resolver = ();

            #[inline]
            unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
                out.write(*self)
            }
        }

        #[cfg(feature = "rkyv")]
        impl<S: rkyv::Fallible> rkyv::Serialize<S> for $type {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<(), S::Error> {
                Ok(())
            }
        }

        #[cfg(feature = "rkyv")]
        impl<D: rkyv::Fallible> rkyv::Deserialize<Self, D> for $type {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<Self, D::Error> {
                Ok(*self)
            }
        }
        impl From<$type> for u8 {
            #[inline]
            fn from(v: $type) -> Self {
                v as u8
            }
        }

        impl std::convert::TryFrom<u8> for $type {
            type Error = crate::error::OsuError;

            #[inline]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($n => Ok(<$type>::$variant),)*
                    _ => Err(crate::error::ParsingError::$type(value).into()),
                }
            }
        }

        #[cfg(feature = "serialize")]
        impl serde::Serialize for $type {
            #[inline]
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_u8(*self as u8)
            }
        }
    };

    (@VISIT $type:tt { $($variant:ident = $n:literal,)* }) => {
        #[inline]
        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            match v {
                $($n => Ok(<$type>::$variant),)*
                _ => {
                    Err(serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &stringify!($($n),*)))
                },
            }
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_any(self)
        }

        #[inline]
        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok($type::default())
        }

        fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut result = None;

            while let Some(key) = map.next_key::<&str>()? {
                match key {
                    "id" | "name" => result = Some(map.next_value()?),
                    _ => {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
            }

            result.ok_or_else(|| serde::de::Error::missing_field("id or name"))
        }
    };

    // Main macro with variants as serde strings
    ($type:tt { $($variant:ident = $n:literal,)* }) => {
        def_enum!(@BASE $type { $($variant = $n,)* });

        impl<'de> serde::de::Visitor<'de> for super::EnumVisitor<$type> {
            type Value = $type;

            #[inline]
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(concat!(stringify!($($n),*),$(", \"",stringify!($variant),"\"",)*))
            }

            #[inline]
            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $(stringify!($variant) => Ok(<$type>::$variant),)*
                    _ => {
                        Err(serde::de::Error::unknown_variant(s, &[stringify!($($variant),*)]))
                    },
                }
            }

            def_enum!(@VISIT $type { $($variant = $n,)* });
        }
    };

    // Main macro with specified serde strings
    ($type:tt { $($variant:ident = $n:literal ($alt:literal),)* }) => {
        def_enum!(@BASE $type { $($variant = $n,)* });

        impl<'de> serde::de::Visitor<'de> for super::EnumVisitor<$type> {
            type Value = $type;

            #[inline]
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                 f.write_str(concat!(stringify!($($n),*),", ",stringify!($($alt),*)))
            }

            #[inline]
            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $($alt => Ok(<$type>::$variant),)*
                    _ => {
                        Err(serde::de::Error::unknown_variant(s, &[stringify!($($alt),*)]))
                    },
                }
            }

            def_enum!(@VISIT $type { $($variant = $n,)* });
        }
    }
}

mod grade;
mod mode;
mod serde_;

#[cfg(feature = "rkyv")]
mod rkyv_impls;

pub(crate) mod beatmap_;
pub(crate) mod comments_;
pub(crate) mod event_;
pub(crate) mod forum_;
pub(crate) mod kudosu_;
pub(crate) mod matches_;
pub(crate) mod news_;
pub(crate) mod ranking_;
pub(crate) mod score_;
pub(crate) mod seasonal_backgrounds_;
pub(crate) mod user_;
pub(crate) mod wiki_;

/// Beatmap(set) related types
pub mod beatmap {
    pub use super::beatmap_::{
        Beatmap, BeatmapDifficultyAttributes, BeatmapExtended, Beatmapset, BeatmapsetAvailability,
        BeatmapsetCommentEdit, BeatmapsetCommentId, BeatmapsetCommentKudosuGain,
        BeatmapsetCommentNominate, BeatmapsetCommentOwnerChange, BeatmapsetCovers,
        BeatmapsetDiscussion, BeatmapsetEvent, BeatmapsetEvents, BeatmapsetExtended,
        BeatmapsetHype, BeatmapsetNominations, BeatmapsetPost, BeatmapsetReviewsConfig,
        BeatmapsetSearchResult, BeatmapsetSearchSort, BeatmapsetVote, FailTimes,
        GameModeAttributes, Genre, Language, MostPlayedMap, RankStatus,
    };
}

/// Comment related types
pub mod comments {
    pub use super::comments_::{Comment, CommentBundle, CommentSort, CommentableMeta};
}

/// Forum post related types
pub mod forum {
    pub use super::forum_::{ForumPost, ForumPosts, ForumPostsSearch, ForumTopic};
}

/// User kudosu related types
pub mod kudosu {
    pub use super::kudosu_::{KudosuAction, KudosuGiver, KudosuHistory, KudosuPost};
}

/// Multiplayer match related types
pub mod matches {
    pub use super::matches_::{
        MatchEvent, MatchGame, MatchGameDrain, MatchGameIter, MatchInfo, MatchList,
        MatchListParams, MatchScore, OsuMatch, ScoringType, Team, TeamType,
    };
}

/// Game mods related types
pub mod mods;

/// News related types
pub mod news {
    pub use super::news_::{News, NewsPost, NewsSearch, NewsSidebar};
}

/// Ranking related types
pub mod ranking {
    pub use super::ranking_::{
        ChartRankings, CountryRanking, CountryRankings, Rankings, Spotlight,
    };
}

/// User event related types
pub mod event {
    pub use super::event_::{
        Event, EventBeatmap, EventBeatmapset, EventSort, EventType, EventUser, Events,
    };
}

/// Score related types
pub mod score {
    pub use super::score_::{
        BeatmapUserScore, LegacyScoreStatistics, Score, ScoreStatistics, ScoreWeight,
        UserAttributes,
    };
}

/// Seasonal background related types
pub mod seasonal_backgrounds {
    pub use super::seasonal_backgrounds_::{SeasonalBackground, SeasonalBackgrounds};
}

/// User related types
pub mod user {
    pub use super::user_::{
        AccountHistory, Badge, CountryCode, GradeCounts, Group, HistoryType, Medal, MedalCompact,
        MonthlyCount, Playstyle, ProfileBanner, ProfilePage, User, UserCover, UserExtended,
        UserHighestRank, UserKudosu, UserLevel, UserPage, UserStatistics, UserStatisticsModes,
        Username,
    };
}

/// Wiki related types
pub mod wiki {
    pub use super::wiki_::WikiPage;
}

/// Archived types and Resolvers for all types
#[cfg(feature = "rkyv")]
pub mod rkyv {
    pub use super::beatmap_::{
        ArchivedBeatmap, ArchivedBeatmapExtended, ArchivedBeatmapset,
        ArchivedBeatmapsetAvailability, ArchivedBeatmapsetCommentEdit, ArchivedBeatmapsetCommentId,
        ArchivedBeatmapsetCommentKudosuGain, ArchivedBeatmapsetCommentNominate,
        ArchivedBeatmapsetCommentOwnerChange, ArchivedBeatmapsetCovers,
        ArchivedBeatmapsetDiscussion, ArchivedBeatmapsetEvent, ArchivedBeatmapsetEvents,
        ArchivedBeatmapsetExtended, ArchivedBeatmapsetPost, ArchivedBeatmapsetSearchResult,
        ArchivedFailTimes, ArchivedMostPlayedMap, ArchivedRankStatus,
        BeatmapDifficultyAttributesResolver, BeatmapExtendedResolver, BeatmapResolver,
        BeatmapsetAvailabilityResolver, BeatmapsetCommentEditResolver, BeatmapsetCommentIdResolver,
        BeatmapsetCommentKudosuGainResolver, BeatmapsetCommentNominateResolver,
        BeatmapsetCommentOwnerChangeResolver, BeatmapsetCoversResolver,
        BeatmapsetDiscussionResolver, BeatmapsetEventResolver, BeatmapsetEventsResolver,
        BeatmapsetHypeResolver, BeatmapsetNominationsResolver, BeatmapsetPostResolver,
        BeatmapsetResolver, BeatmapsetReviewsConfigResolver, BeatmapsetSearchResultResolver,
        BeatmapsetVoteResolver, FailTimesResolver, GameModeAttributesResolver,
        MostPlayedMapResolver, RankStatusResolver,
    };

    pub use super::comments_::{
        ArchivedComment, ArchivedCommentBundle, ArchivedCommentableMeta, CommentBundleResolver,
        CommentResolver, CommentSortResolver, CommentableMetaResolver,
    };

    pub use super::event_::{
        ArchivedEvent, ArchivedEventBeatmap, ArchivedEventBeatmapset, ArchivedEventSort,
        ArchivedEventType, ArchivedEventUser, ArchivedEvents, EventBeatmapResolver,
        EventBeatmapsetResolver, EventResolver, EventSortResolver, EventTypeResolver,
        EventUserResolver, EventsResolver,
    };

    pub use super::forum_::{
        ArchivedForumPost, ArchivedForumPosts, ArchivedForumPostsSearch, ArchivedForumTopic,
        ForumPostResolver, ForumPostsResolver, ForumPostsSearchResolver, ForumTopicResolver,
    };

    pub use super::grade::GradeResolver;

    pub use super::kudosu_::{
        ArchivedKudosuGiver, ArchivedKudosuHistory, ArchivedKudosuPost, KudosuActionResolver,
        KudosuGiverResolver, KudosuHistoryResolver, KudosuPostResolver,
    };

    pub use super::mode::GameModeResolver;

    pub use super::news_::{
        ArchivedNews, ArchivedNewsPost, ArchivedNewsSearch, ArchivedNewsSidebar, NewsPostResolver,
        NewsResolver, NewsSearchResolver, NewsSidebarResolver,
    };

    pub use super::ranking_::{
        ArchivedChartRankings, ArchivedCountryRanking, ArchivedCountryRankings, ArchivedRankings,
        ArchivedSpotlight, ChartRankingsResolver, CountryRankingResolver, CountryRankingsResolver,
        RankingsResolver, SpotlightResolver,
    };

    pub use super::score_::{
        ArchivedBeatmapUserScore, ArchivedLegacyScoreStatistics, ArchivedScore,
        ArchivedScoreStatistics, BeatmapUserScoreResolver, LegacyScoreStatisticsResolver,
        ScoreResolver, ScoreStatisticsResolver, ScoreWeightResolver,
    };

    pub use super::seasonal_backgrounds_::{
        ArchivedSeasonalBackground, ArchivedSeasonalBackgrounds, SeasonalBackgroundResolver,
        SeasonalBackgroundsResolver,
    };

    pub use super::user_::{
        AccountHistoryResolver, ArchivedAccountHistory, ArchivedBadge, ArchivedGroup,
        ArchivedMedal, ArchivedMedalCompact, ArchivedMonthlyCount, ArchivedProfileBanner,
        ArchivedUser, ArchivedUserCover, ArchivedUserExtended, ArchivedUserHighestRank,
        ArchivedUserPage, ArchivedUserStatistics, ArchivedUserStatisticsModes, BadgeResolver,
        GradeCountsResolver, GroupResolver, HistoryTypeResolver, MedalCompactResolver,
        MedalResolver, MonthlyCountResolver, PlaystyleResolver, ProfileBannerResolver,
        ProfilePageResolver, UserCoverResolver, UserExtendedResolver, UserHighestRankResolver,
        UserKudosuResolver, UserLevelResolver, UserPageResolver, UserResolver,
        UserStatisticsModesResolver, UserStatisticsResolver,
    };

    pub use super::wiki_::{ArchivedWikiPage, WikiPageResolver};
}

pub use grade::Grade;
pub use mode::GameMode;

use std::marker::PhantomData;

struct EnumVisitor<T>(PhantomData<T>);

impl<T> EnumVisitor<T> {
    const fn new() -> Self {
        Self(PhantomData)
    }
}
