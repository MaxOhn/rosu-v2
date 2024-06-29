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
        BeatmapsetHype, BeatmapsetNominations, BeatmapsetNominationsRequiredMeta, BeatmapsetPost,
        BeatmapsetReviewsConfig, BeatmapsetSearchResult, BeatmapsetSearchSort, BeatmapsetVote,
        FailTimes, GameModeAttributes, Genre, Language, MostPlayedMap, RankStatus,
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

pub use grade::Grade;
pub use mode::GameMode;

use std::marker::PhantomData;

struct EnumVisitor<T>(PhantomData<T>);

impl<T> EnumVisitor<T> {
    const fn new() -> Self {
        Self(PhantomData)
    }
}
