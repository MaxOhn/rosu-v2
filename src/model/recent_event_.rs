use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    beatmap::RankStatus,
    user_::{Medal, Username},
    GameMode, Grade,
};

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// The object has different attributes depending on its type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct RecentEvent {
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "id")]
    pub event_id: u32,
    #[serde(flatten)]
    pub event_type: EventType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct EventBeatmap {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct EventBeatmapset {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum EventType {
    /// When a beatmap has been played for a certain amount of times
    BeatmapPlaycount { beatmap: EventBeatmap, count: u32 },
    /// When a beatmapset changes state
    BeatmapsetApprove {
        approval: RankStatus,
        beatmapset: EventBeatmapset,
        /// Beatmapset owner
        user: EventUser,
    },
    /// When a beatmapset is deleted
    BeatmapsetDelete { beatmapset: EventBeatmapset },
    /// When a beatmapset in graveyard is updated
    BeatmapsetRevive {
        beatmapset: EventBeatmapset,
        /// Beatmapset owner
        user: EventUser,
    },
    /// When a beatmapset is updated
    BeatmapsetUpdate {
        beatmapset: EventBeatmapset,
        /// Beatmapset owner
        user: EventUser,
    },
    /// When a new beatmapset is uploaded
    BeatmapsetUpload {
        beatmapset: EventBeatmapset,
        /// Beatmapset owner
        user: EventUser,
    },
    /// When a user obtained a medal
    #[serde(rename = "achievement")]
    Medal { medal: Medal, user: EventUser },
    /// When a user achieves a certain rank on a beatmap
    Rank {
        #[serde(rename = "scoreRank")]
        grade: Grade,
        rank: u32,
        mode: GameMode,
        beatmap: EventBeatmap,
        user: EventUser,
    },
    /// When a user loses first place to another user
    RankLost {
        mode: GameMode,
        beatmap: EventBeatmap,
        user: EventUser,
    },
    /// When a user supports osu! for the second time and onwards
    #[serde(rename = "userSupportAgain")]
    SupportAgain { user: EventUser },
    /// When a user becomes a supporter for the first time
    #[serde(rename = "userSupportFirst")]
    SupportFirst { user: EventUser },
    /// When a user is gifted a supporter tag by another user
    #[serde(rename = "userSupportGift")]
    SupportGift {
        /// Recipient user
        user: EventUser,
    },
    /// When a user changes their username
    UsernameChange {
        /// Includes previous_username
        user: EventUser,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct EventUser {
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::UsernameWrapper))]
    pub username: Username,
    pub url: String,
    /// Only for UsernameChange events
    #[serde(
        default,
        rename = "previousUsername",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::UsernameMap))]
    pub previous_username: Option<Username>,
}
