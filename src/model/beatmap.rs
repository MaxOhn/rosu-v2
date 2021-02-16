use super::GameMode;
use crate::error::{OsuError, ValueEnum};

use chrono::{DateTime, Utc};
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::{convert::TryFrom, fmt};

#[derive(Debug, Deserialize)]
pub struct Beatmap {
    pub bpm: f32,
    pub checksum: Option<String>,
    pub convert: bool,
    pub count_circles: u32,
    pub count_sliders: u32,
    pub count_spinners: u32,
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(rename = "ar")]
    pub diff_ar: f32,
    #[serde(rename = "cs")]
    pub diff_cs: f32,
    #[serde(rename = "drain")]
    pub diff_hp: f32,
    #[serde(rename = "accuracy")]
    pub diff_od: f32,
    pub fail_times: Option<FailTimes>,
    pub is_scoreable: bool,
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "id")]
    pub map_id: u32,
    #[serde(rename = "beatmapset", default)]
    pub mapset: Mapset,
    #[serde(rename = "beatmapset_id")]
    pub mapset_id: u32,
    pub max_combo: Option<u32>,
    pub mode: GameMode,
    pub mode_int: u8,
    pub passcount: u32,
    pub playcount: u32,
    pub ranked: i8,
    #[serde(rename = "hit_length")]
    pub seconds_drain: u32,
    #[serde(rename = "total_length")]
    pub seconds_total: u32,
    #[serde(rename = "difficulty_rating")]
    pub stars: f32,
    pub status: String,
    pub url: String,
    pub version: String,
}

pub struct BeatmapCompact {
    pub checksum: Option<String>,
    pub fail_times: Option<FailTimes>,
    pub map_id: u32,
    pub mapset: Mapset,
    pub max_combo: Option<u32>,
    pub mode: GameMode,
    pub stars: f32,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Beatmapset {
    pub artist: String,
    pub artist_unicode: String,
    pub availability: BeatmapsetAvailability,
    pub beatmaps: Option<Vec<Beatmap>>,
    pub bpm: f32,
    pub can_be_hyped: bool,
    pub covers: Covers,
    pub creator: String,
    pub discussion_enabled: bool,
    pub discussion_locked: bool,
    pub favourite_count: u32,
    pub hype: BeatmapsetHype,
    pub is_scoreable: bool,
    pub last_updated: DateTime<Utc>,
    pub legacy_thread_url: Option<String>,
    #[serde(rename = "id")]
    pub mapset_id: u32,
    pub nominations: BeatmapsetNominations,
    #[serde(rename = "play_count")]
    pub playcount: u32,
    pub preview_url: String,
    pub ranked: RankStatus,
    pub source: String,
    pub status: String,
    pub storyboard: bool,
    pub submitted_date: Option<DateTime<Utc>>,
    pub tags: String,
    pub title: String,
    pub title_unicode: String,
    pub user_id: u32,
    pub video: bool,
}

#[derive(Debug, Deserialize)]
pub struct BeatmapsetAvailability {
    download_disabled: bool,
    more_information: Option<String>,
}

// TODO: Optional fields
#[derive(Debug, Deserialize)]
pub struct BeatmapsetCompact {
    pub artist: String,
    pub artist_unicode: String,
    pub covers: Covers,
    pub creator: String,
    pub favourite_count: u32,
    #[serde(rename = "id")]
    pub mapset_id: u32,
    #[serde(rename = "play_count")]
    pub playcount: u32,
    pub preview_url: String,
    pub source: String,
    pub status: String,
    pub title: String,
    pub title_unicode: String,
    pub user_id: u32,
    pub video: bool,
}

#[derive(Debug, Deserialize)]
pub struct BeatmapsetHype {
    current: u32,
    required: u32,
}

#[derive(Debug, Deserialize)]
pub struct BeatmapsetNominations {
    current: u32,
    required: u32,
}

#[derive(Debug, Deserialize)]
pub struct Covers {
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

#[derive(Debug, Deserialize)]
pub struct FailTimes {
    pub exit: Option<Vec<u32>>, // TODO: Make this [u32; 100]
    pub fail: Option<Vec<u32>>,
}

// TODO: Probably needs proper deserializing
#[derive(Debug, Deserialize)]
pub enum Mapset {
    Full(Beatmapset),
    Compact(BeatmapsetCompact),
    None,
}

impl Default for Mapset {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
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
                    value: ValueEnum::RankStatus,
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
