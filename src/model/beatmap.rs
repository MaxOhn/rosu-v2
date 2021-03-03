use super::GameMode;
use crate::error::OsuError;

use chrono::{DateTime, Utc};
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::{convert::TryFrom, fmt};

#[derive(Clone, Debug, Deserialize)]
pub struct Beatmap {
    pub ar: f32,
    pub bpm: f32,
    pub checksum: Option<String>,
    pub convert: bool,
    pub count_circles: u32,
    pub count_sliders: u32,
    pub count_spinners: u32,
    pub cs: f32,
    pub deleted_at: Option<DateTime<Utc>>,
    pub fail_times: Option<FailTimes>,
    #[serde(rename = "drain")]
    pub hp: f32,
    pub is_scoreable: bool,
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "id")]
    pub map_id: u32,
    #[serde(rename = "beatmapset", default)]
    pub mapset: Option<Mapset>,
    #[serde(rename = "beatmapset_id")]
    pub mapset_id: u32,
    pub max_combo: Option<u32>,
    pub mode: GameMode,
    #[serde(rename = "accuracy")]
    pub od: f32,
    pub passcount: u32,
    pub playcount: u32,
    #[serde(rename = "hit_length")]
    pub seconds_drain: u32,
    #[serde(rename = "total_length")]
    pub seconds_total: u32,
    #[serde(rename = "difficulty_rating")]
    pub stars: f32,
    pub status: RankStatus,
    /// Full URL, i.e. `https://osu.ppy.sh/beatmaps/{map_id}`
    pub url: String,
    pub version: String,
}

impl PartialEq for Beatmap {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map_id == other.map_id && self.last_updated == other.last_updated
    }
}

impl Eq for Beatmap {}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BeatmapCompact {
    pub checksum: Option<String>,
    pub fail_times: Option<FailTimes>,
    #[serde(rename = "id")]
    pub map_id: u32,
    #[serde(rename = "beatmapset", default)]
    pub mapset: Option<Mapset>,
    pub max_combo: Option<u32>,
    pub mode: GameMode,
    #[serde(rename = "total_length")]
    pub seconds_total: u32,
    #[serde(rename = "difficulty_rating")]
    pub stars: f32,
    pub status: RankStatus,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Beatmapset {
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub availability: BeatmapsetAvailability,
    pub bpm: f32,
    pub can_be_hyped: bool,
    pub covers: BeatmapsetCovers,
    pub creator: String,
    #[serde(rename = "user_id")]
    pub creator_id: u32,
    pub discussion_enabled: bool,
    pub discussion_locked: bool,
    pub favourite_count: u32,
    pub hype: Option<BeatmapsetHype>,
    pub is_scoreable: bool,
    pub last_updated: DateTime<Utc>,
    /// Full URL, i.e. `https://osu.ppy.sh/community/forums/topics/{thread_id}`
    pub legacy_thread_url: Option<String>,
    #[serde(rename = "beatmaps")]
    pub maps: Option<Vec<Beatmap>>,
    #[serde(rename = "id")]
    pub mapset_id: u32,
    pub nominations_summary: BeatmapsetNominations,
    pub nsfw: bool,
    #[serde(rename = "play_count")]
    pub playcount: u32,
    /// Full URL, i.e. `b.ppy.sh/preview/{mapset_id}.mp3`
    pub preview_url: String,
    pub ratings: Option<Vec<u32>>,
    pub ranked_date: Option<DateTime<Utc>>,
    pub source: String,
    pub status: RankStatus,
    pub storyboard: bool,
    pub submitted_date: Option<DateTime<Utc>>,
    pub tags: String,
    pub title: String,
    pub title_unicode: Option<String>,
    pub video: bool,
}

impl PartialEq for Beatmapset {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.mapset_id == other.mapset_id && self.last_updated == other.last_updated
    }
}

impl Eq for Beatmapset {}

#[derive(Clone, Debug, Deserialize)]
pub struct BeatmapsetAvailability {
    download_disabled: bool,
    more_information: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct BeatmapsetCompact {
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub covers: BeatmapsetCovers,
    pub creator: String,
    #[serde(rename = "user_id")]
    pub creator_id: u32,
    pub favourite_count: u32,
    pub hype: Option<BeatmapsetHype>,
    #[serde(rename = "id")]
    pub mapset_id: u32,
    pub nsfw: bool,
    #[serde(rename = "play_count")]
    pub playcount: u32,
    /// Full URL, i.e. `b.ppy.sh/preview/{mapset_id}.mp3`
    pub preview_url: String,
    pub source: String,
    pub status: RankStatus,
    pub title: String,
    pub title_unicode: Option<String>,
    pub video: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct BeatmapsetCovers {
    pub cover: String,
    #[serde(rename = "cover@2x")]
    pub cover_2x: String,
    pub card: String,
    #[serde(rename = "card@2x")]
    pub card_2x: String,
    pub list: String,
    #[serde(rename = "list@2x")]
    pub list_2x: String,
    #[serde(rename = "slimcover")]
    pub slim_cover: String,
    #[serde(rename = "slimcover@2x")]
    pub slim_cover_2x: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct BeatmapsetHype {
    current: u32,
    required: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct BeatmapsetNominations {
    current: u32,
    required: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FailTimes {
    pub exit: Option<Vec<u32>>, // TODO: Make this [u32; 100], serde currently only goes up to 32
    pub fail: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Mapset {
    Full(Beatmapset),
    Compact(BeatmapsetCompact),
}

#[derive(Clone, Debug, Deserialize)]
pub struct MostPlayedMap {
    pub count: usize,
    #[serde(rename = "beatmap")]
    pub map: BeatmapCompact,
    #[serde(rename = "beatmap_id")]
    pub map_id: u32,
    #[serde(rename = "beatmapset")]
    pub mapset: BeatmapsetCompact,
}

impl PartialEq for MostPlayedMap {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map_id == other.map_id
    }
}

impl Eq for MostPlayedMap {}

macro_rules! impl_get {
    ($func:ident -> $ret:ident) => {
        #[inline]
        pub fn $func(&self) -> $ret {
            match self {
                Self::Full(set) => set.$func,
                Self::Compact(set) => set.$func,
            }
        }
    };

    ($func:ident -> &$ret:ident) => {
        #[inline]
        pub fn $func(&self) -> &$ret {
            match self {
                Self::Full(set) => &set.$func,
                Self::Compact(set) => &set.$func,
            }
        }
    };
}

impl Mapset {
    #[inline]
    pub fn artist_unicode(&self) -> Option<&str> {
        match self {
            Self::Full(set) => set.artist_unicode.as_deref(),
            Self::Compact(set) => set.artist_unicode.as_deref(),
        }
    }

    #[inline]
    pub fn title_unicode(&self) -> Option<&str> {
        match self {
            Self::Full(set) => set.title_unicode.as_deref(),
            Self::Compact(set) => set.title_unicode.as_deref(),
        }
    }

    impl_get!(artist -> &str);
    impl_get!(covers -> &BeatmapsetCovers);
    impl_get!(creator -> &str);
    impl_get!(favourite_count -> u32);
    impl_get!(mapset_id -> u32);
    impl_get!(playcount -> u32);
    impl_get!(preview_url -> &str);
    impl_get!(source -> &str);
    impl_get!(status -> RankStatus);
    impl_get!(title -> &str);
    impl_get!(creator_id -> u32);
    impl_get!(video -> bool);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RankStatus {
    Graveyard = -2,
    WIP = -1,
    Pending = 0,
    Ranked = 1,
    Approved = 2,
    Qualified = 3,
    Loved = 4,
}

impl TryFrom<i8> for RankStatus {
    type Error = OsuError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let status = match value {
            -2 => Self::Graveyard,
            -1 => Self::WIP,
            0 => Self::Pending,
            1 => Self::Ranked,
            2 => Self::Approved,
            3 => Self::Qualified,
            4 => Self::Loved,
            _ => {
                return Err(OsuError::ParsingValue {
                    value: "RankStatus",
                })
            }
        };

        Ok(status)
    }
}

struct RankStatusVisitor;

impl<'de> Visitor<'de> for RankStatusVisitor {
    type Value = RankStatus;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an i8 or a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let status = match v {
            "4" | "loved" => RankStatus::Loved,
            "3" | "qualified" => RankStatus::Qualified,
            "2" | "approved" => RankStatus::Approved,
            "1" | "ranked" => RankStatus::Ranked,
            "0" | "pending" => RankStatus::Pending,
            "-1" | "wip" => RankStatus::WIP,
            "-2" | "graveyard" => RankStatus::Graveyard,
            _ => {
                return Err(Error::invalid_value(
                    Unexpected::Str(v),
                    &r#"
            "4", "loved",
            "3", qualified",
            "2", "approved",
            "1", "ranked",
            "0", "pending",
            "-1", "wip",
            "-2", or "graveyard"
            "#,
                ))
            }
        };

        Ok(status)
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        RankStatus::try_from(v as i8)
            .map_err(|_| Error::invalid_value(Unexpected::Signed(v), &"value between -2 and 4"))
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        RankStatus::try_from(v as i8)
            .map_err(|_| Error::invalid_value(Unexpected::Unsigned(v), &"value between -2 and 4"))
    }
}

impl<'de> Deserialize<'de> for RankStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(RankStatusVisitor)
    }
}
