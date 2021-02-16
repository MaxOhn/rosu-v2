use super::GameMode;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    created_at: DateTime<Utc>,
    #[serde(rename = "id")]
    event_id: u32,
    #[serde(flatten)]
    event_type: EventType,
}

#[derive(Debug, Deserialize)]
pub struct EventBeatmap {
    title: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct EventBeatmapset {
    title: String,
    url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum EventType {
    /// When a beatmap has been played for a certain amount of times
    BeatmapPlaycount { beatmap: EventBeatmap, count: u32 },
    /// When a beatmapset changes state
    BeatmapsetApprove {
        approval: String, // TODO
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
    Medal {
        // medal: Medal, // TODO
        user: EventUser,
    },
    /// When a user achieves a certain rank on a beatmap
    Rank {
        #[serde(rename = "scoreRank")]
        grade: String, // TODO
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

#[derive(Debug, Deserialize)]
pub struct EventUser {
    username: String,
    url: String,
    /// Only for UsernameChange events
    #[serde(rename = "previousUsername")]
    previous_username: Option<String>,
}
