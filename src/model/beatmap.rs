use super::GameMode;

use chrono::{DateTime, Utc};

pub struct Beatmap {
    pub bpm: f32,
    pub checksum: Option<String>,
    pub convert: bool,
    pub count_circles: u32,
    pub count_sliders: u32,
    pub count_spinners: u32,
    pub deleted_at: Option<DateTime<Utc>>,
    pub diff_ar: f32,
    pub diff_cs: f32,
    pub diff_hp: f32,
    pub diff_od: f32,
    pub fail_times: Option<FailTimes>,
    pub is_scoreable: bool,
    pub last_updated: DateTime<Utc>,
    pub map_id: u32,
    pub mapset: Mapset,
    pub mapset_id: u32,
    pub max_combo: Option<u32>,
    pub mode: GameMode,
    pub mode_int: u8,
    pub passcount: u32,
    pub playcount: u32,
    pub ranked: i8,
    pub seconds_drain: u32,
    pub seconds_total: u32,
    pub stars: f32,
    pub status: i8,
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

pub struct Beatmapset {
    pub artist: String,
    pub artist_unicode: String,
    pub bpm: f32,
    pub can_be_hyped: bool,
    pub covers: Covers,
    pub creator: String,
    pub discussion_enabled: bool,
    pub discussion_locked: bool,
    pub download_disabled: bool,
    pub favourite_count: u32,
    pub hype_count: u32,
    pub hype_required: u32,
    pub is_scoreable: bool,
    pub last_updated: DateTime<Utc>,
    pub legacy_thread_url: Option<String>,
    pub mapset_id: u32,
    pub more_information: Option<String>,
    pub nominations_count: u32,
    pub nominations_required: u32,
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
    pub video: String,
}

// TODO: Optional fields
pub struct BeatmapsetCompact {
    pub artist: String,
    pub artist_unicode: String,
    pub covers: Covers,
    pub creator: String,
    pub favourite_count: u32,
    pub mapset_id: u32,
    pub playcount: u32,
    pub preview_url: String,
    pub source: String,
    pub status: String,
    pub title: String,
    pub title_unicode: String,
    pub user_id: u32,
    pub video: String,
}

pub struct Covers {
    pub cover: String,
    pub cover_2x: String,
    pub card: String,
    pub card_2x: String,
    pub list: String,
    pub list_2x: String,
    pub slim_cover: String,
    pub slim_cover_2x: String,
}

pub struct FailTimes {
    pub exit: Option<[u32; 100]>,
    pub fail: Option<[u32; 100]>,
}

pub enum Mapset {
    Full(Beatmapset),
    Compact(BeatmapsetCompact),
    None,
}

#[repr(i8)]
pub enum RankStatus {
    Graveyard = -2,
    WIP = -1,
    Pending = 0,
    Ranked = 1,
    Approved = 2,
    Qualified = 3,
    Loved = 4,
}
