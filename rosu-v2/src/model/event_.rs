use std::fmt;

use serde::{de, Deserialize};
use time::OffsetDateTime;

use crate::{Osu, OsuResult};

use super::{
    beatmap::RankStatus,
    serde_,
    user::{Medal, Username},
    GameMode, Grade,
};

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Events {
    pub events: Vec<Event>,
    #[serde(rename = "cursor_string", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sort: Option<EventSort>,
}

impl Events {
    /// Returns whether there is a next page of events, retrievable via
    /// [`get_next`](Events::get_next).
    #[inline]
    pub const fn has_more(&self) -> bool {
        self.cursor.is_some()
    }

    /// If [`has_more`](Events::has_more) is true, the API can provide the next
    /// set of events and this method will request them. Otherwise, this method
    /// returns `None`.
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<Events>> {
        let cursor = self.cursor.as_deref()?;

        let fut = osu
            .events()
            .cursor(cursor)
            .sort(self.sort.unwrap_or_default());

        Some(fut.await)
    }
}

/// The object has different attributes depending on its type.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Event {
    #[serde(with = "serde_::datetime")]
    pub created_at: OffsetDateTime,
    #[serde(rename = "id")]
    pub event_id: u32,
    #[serde(flatten)]
    pub event_type: EventType,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct EventBeatmap {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct EventBeatmapset {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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
    Medal {
        #[serde(rename = "achievement")]
        medal: Medal,
        user: EventUser,
    },
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum EventSort {
    #[default]
    IdDescending,
    IdAscending,
}

impl EventSort {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::IdDescending => "id_desc",
            Self::IdAscending => "id_asc",
        }
    }
}

impl<'de> de::Deserialize<'de> for EventSort {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct EventSortVisitor;

        impl<'de> de::Visitor<'de> for EventSortVisitor {
            type Value = EventSort;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("id_desc or id_asc")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match v {
                    "id_desc" => Ok(EventSort::IdDescending),
                    "id_asc" => Ok(EventSort::IdAscending),
                    _ => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
                }
            }
        }

        d.deserialize_str(EventSortVisitor)
    }
}

impl serde::Serialize for EventSort {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct EventUser {
    pub username: Username,
    pub url: String,
    /// Only for UsernameChange events
    #[serde(
        default,
        rename = "previousUsername",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_username: Option<Username>,
}
