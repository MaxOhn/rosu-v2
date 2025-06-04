use rosu_mods::{serde::GameModsSeed, GameMode, GameMods};
use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;
use time::OffsetDateTime;

use crate::{
    error::OsuError,
    model::{serde_util, CacheUserFn, ContainedUsers},
    prelude::{Beatmap, User},
};

/// The playlist item of a [`Room`].
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PlaylistItem {
    #[cfg_attr(feature = "serialize", serde(rename = "beatmap"))]
    pub map: Beatmap,
    #[cfg_attr(feature = "serialize", serde(rename = "beatmap_id"))]
    pub map_id: u32,
    #[cfg_attr(feature = "serialize", serde(with = "serde_util::option_datetime"))]
    pub created_at: Option<OffsetDateTime>,
    #[cfg_attr(feature = "serialize", serde(rename = "id"))]
    pub playlist_item_id: u32,
    pub owner_id: u32,
    pub room_id: u64,
    #[cfg_attr(feature = "serialize", serde(rename = "ruleset_id"))]
    pub mode: GameMode,
    pub freestyle: bool,
    pub expired: bool,
    #[cfg_attr(feature = "serialize", serde(with = "serde_util::option_datetime"))]
    pub played_at: Option<OffsetDateTime>,
    pub playlist_order: Option<u32>,
    pub allowed_mods: GameMods,
    pub required_mods: GameMods,
}

impl ContainedUsers for PlaylistItem {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.map.apply_to_users(f);
    }
}

impl<'de> Deserialize<'de> for PlaylistItem {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
        struct PlaylistItemRaw {
            #[serde(rename = "beatmap")]
            map: Beatmap,
            #[serde(rename = "beatmap_id")]
            map_id: u32,
            #[serde(with = "serde_util::option_datetime")]
            created_at: Option<OffsetDateTime>,
            #[serde(rename = "id")]
            playlist_item_id: u32,
            owner_id: u32,
            room_id: u64,
            #[serde(rename = "ruleset_id")]
            mode: GameMode,
            freestyle: bool,
            expired: bool,
            #[serde(with = "serde_util::option_datetime")]
            played_at: Option<OffsetDateTime>,
            playlist_order: Option<u32>,
            allowed_mods: Box<RawValue>,
            required_mods: Box<RawValue>,
        }

        let item_raw = <PlaylistItemRaw as serde::Deserialize>::deserialize(d)?;

        let mods_seed = GameModsSeed::Mode {
            mode: item_raw.mode,
            deny_unknown_fields: false,
        };

        Ok(Self {
            map: item_raw.map,
            map_id: item_raw.map_id,
            created_at: item_raw.created_at,
            playlist_item_id: item_raw.playlist_item_id,
            owner_id: item_raw.owner_id,
            room_id: item_raw.room_id,
            mode: item_raw.mode,
            freestyle: item_raw.freestyle,
            expired: item_raw.expired,
            played_at: item_raw.played_at,
            playlist_order: item_raw.playlist_order,
            allowed_mods: mods_seed
                .deserialize(&*item_raw.allowed_mods)
                .map_err(|e| OsuError::invalid_mods(&item_raw.allowed_mods, &e))?,
            required_mods: mods_seed
                .deserialize(&*item_raw.required_mods)
                .map_err(|e| OsuError::invalid_mods(&item_raw.required_mods, &e))?,
        })
    }
}

/// Statistics of a [`PlaylistItem`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct PlaylistItemStats {
    pub count_active: usize,
    pub count_total: usize,
    #[serde(rename = "ruleset_ids")]
    pub modes: Vec<GameMode>,
}

/// A multiplayer room.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Room {
    #[serde(rename = "id")]
    pub room_id: u64,
    pub name: String,
    pub category: RoomCategory,
    pub status: RoomStatus,
    #[serde(rename = "type")]
    pub type_group: RoomTypeGroup,
    pub user_id: u32,
    #[serde(with = "serde_util::datetime")]
    pub starts_at: OffsetDateTime,
    #[serde(with = "serde_util::option_datetime")]
    pub ends_at: Option<OffsetDateTime>,
    pub max_attempts: Option<usize>,
    pub participant_count: usize,
    pub channel_id: Option<u64>,
    pub active: bool,
    pub has_password: bool,
    pub queue_mode: RoomQueueMode,
    pub auto_skip: bool,
    pub current_playlist_item: Option<PlaylistItem>,
    pub host: User,
    pub recent_participants: Vec<User>,
    pub playlist_item_stats: PlaylistItemStats,
    pub difficulty_range: RoomDifficultyRange,
}

impl ContainedUsers for Room {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.current_playlist_item.apply_to_users(f);
        self.host.apply_to_users(f);
        self.recent_participants.apply_to_users(f);
    }
}

/// The category of a [`Room`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RoomCategory {
    Normal,
    Spotlight,
    FeaturedArtist,
    DailyChallenge,
}

/// The difficulty range of a [`Room`].
#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct RoomDifficultyRange {
    pub min: f32,
    pub max: f32,
}

/// The queue mode of a [`Room`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RoomQueueMode {
    AllPlayers,
    AllPlayersRoundRobin,
    HostOnly,
}

/// The status of a [`Room`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[non_exhaustive]
pub enum RoomStatus {
    Idle,
    Playing,
}

/// The type of a [`Room`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[non_exhaustive]
pub enum RoomTypeGroup {
    Playlists,
    HeadToHead,
    TeamVersus,
    TagCoop,
    TagTeamVersus,
}
