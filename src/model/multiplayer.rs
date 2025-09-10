use rosu_mods::{serde::GameModsSeed, GameMode, GameMods};
use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;
use time::OffsetDateTime;

use crate::{
    error::OsuError,
    model::{serde_util, CacheUserFn, ContainedUsers},
    prelude::{Beatmap, Score, User},
    request::{GetRoomEvents, GetRoomLeaderboard},
    Osu, OsuResult,
};

/// The playlist item of a [`Room`].
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
    pub scores: Vec<Score>,
}

impl ContainedUsers for PlaylistItem {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.map.apply_to_users(f);
        self.scores.apply_to_users(f);
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
            #[serde(default)]
            scores: Vec<Score>,
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
            scores: item_raw.scores,
        })
    }
}

/// Statistics of a [`PlaylistItem`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct PlaylistItemStats {
    pub count_active: usize,
    pub count_total: usize,
    #[serde(rename = "ruleset_ids")]
    pub modes: Vec<GameMode>,
}

/// Scores of a [`Room`]'s playlist.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct PlaylistScores {
    #[serde(default)]
    pub room_id: u64,
    #[serde(default)]
    pub playlist_id: u32,
    pub params: PlaylistScoresParams,
    pub scores: Vec<Score>,
    pub total: usize,
    pub user_score: Option<Score>,
    #[serde(rename = "cursor_string")]
    pub cursor: Box<str>,
}

impl PlaylistScores {
    /// Get the next set of [`PlaylistScores`].
    ///
    /// Returns `None` if there are no more events.
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<Self>> {
        if self.scores.is_empty() {
            return None;
        }

        let res = osu
            .playlist_scores(self.room_id, self.playlist_id)
            .limit(self.params.limit)
            .sort(self.params.sort)
            .cursor(self.cursor.as_ref())
            .await;

        Some(res)
    }
}

impl ContainedUsers for PlaylistScores {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.scores.apply_to_users(f);
        self.user_score.apply_to_users(f);
    }
}

/// Request parameters for [`PlaylistScores`].
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct PlaylistScoresParams {
    pub limit: usize,
    pub sort: PlaylistScoresSort,
}

/// The sort order of [`PlaylistScores`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum PlaylistScoresSort {
    #[serde(rename = "score_desc")]
    Descending,
    #[serde(rename = "score_asc")]
    Ascending,
}

/// A multiplayer room.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Room {
    #[serde(rename = "id")]
    pub room_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub category: RoomCategory,
    pub status: RoomStatus,
    #[serde(rename = "type")]
    pub type_group: RoomTypeGroup,
    pub user_id: u32,
    #[serde(with = "serde_util::datetime")]
    pub starts_at: OffsetDateTime,
    #[serde(default, with = "serde_util::option_datetime")]
    pub ends_at: Option<OffsetDateTime>,
    pub max_attempts: Option<usize>,
    pub participant_count: usize,
    pub channel_id: Option<u64>,
    pub active: bool,
    pub has_password: bool,
    pub queue_mode: RoomQueueMode,
    pub auto_skip: bool,
    #[serde(default)]
    pub pinned: bool,
    pub current_playlist_item: Option<PlaylistItem>,
    pub host: User,
    pub recent_participants: Vec<User>,
    pub playlist_item_stats: Option<PlaylistItemStats>,
    pub difficulty_range: Option<RoomDifficultyRange>,
    #[serde(default)]
    pub playlist: Vec<PlaylistItem>,
}

impl Room {
    /// Get the [`RoomEvents`] for this [`Room`].
    pub const fn get_events<'osu>(&self, osu: &'osu Osu) -> GetRoomEvents<'osu> {
        osu.room_events(self.room_id)
    }

    /// Get the [`RoomLeaderboard`] for this [`Room`].
    pub const fn get_leaderboard<'osu>(&self, osu: &'osu Osu) -> GetRoomLeaderboard<'osu> {
        osu.room_leaderboard(self.room_id)
    }
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RoomDifficultyRange {
    pub min: f32,
    pub max: f32,
}

/// An event of a [`Room`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RoomEvent {
    #[serde(with = "serde_util::datetime")]
    pub created_at: OffsetDateTime,
    #[serde(rename = "event_type")]
    pub kind: RoomEventKind,
    #[serde(rename = "id")]
    pub room_event_id: u64,
    pub playlist_item_id: Option<u32>,
    pub user_id: Option<u32>,
}

/// The type of a [`RoomEvent`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[serde(rename_all = "snake_case")]
pub enum RoomEventKind {
    GameStarted,
    GameAborted,
    GameCompleted,
    HostChanged,
    PlayerJoined,
    PlayerKicked,
    PlayerLeft,
    RoomCreated,
    RoomDisbanded,
    Unknown,
}

/// Events of a [`Room`].
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RoomEvents {
    pub events: Vec<RoomEvent>,
    pub users: Vec<User>,
    pub first_event_id: u64,
    pub last_event_id: u64,
    pub playlist_items: Vec<PlaylistItem>,
    pub current_playlist_item_id: u32,
}

impl RoomEvents {
    /// Get the previous set of [`RoomEvents`].
    ///
    /// Returns `None` if there are no previous events.
    pub async fn get_previous(&self, osu: &Osu) -> Option<OsuResult<Self>> {
        if self.events.is_empty() {
            return None;
        }

        Some(osu.room_events(self.first_event_id).await)
    }

    /// Get the next set of [`RoomEvents`].
    ///
    /// Returns `None` if there are no more events.
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<Self>> {
        if self.events.is_empty() {
            return None;
        }

        Some(osu.room_events(self.last_event_id).await)
    }
}

impl ContainedUsers for RoomEvents {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.users.apply_to_users(f);
        self.playlist_items.apply_to_users(f);
    }
}

/// The leaderboard of a [`Room`].
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RoomLeaderboard {
    pub leaderboard: Vec<RoomLeaderboardItem>,
    /// `None` if not authenticated via `OAuth2`.
    pub user_score: Option<RoomLeaderboardItem>,
}

impl ContainedUsers for RoomLeaderboard {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.leaderboard.apply_to_users(f);
        self.user_score.apply_to_users(f);
    }
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

/// An entry within a [`RoomLeaderboard`].
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct RoomLeaderboardItem {
    #[serde(with = "serde_util::adjust_acc")]
    pub accuracy: f32,
    pub attempts: usize,
    pub completed: usize,
    pub pp: f32,
    pub room_id: u64,
    #[serde(rename = "total_score")]
    pub score: u64,
    pub user_id: u32,
    pub user: User,
}

impl ContainedUsers for RoomLeaderboardItem {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.user.apply_to_users(f);
    }
}

/// The status of a [`Room`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[non_exhaustive]
pub enum RoomStatus {
    Idle,
    Playing,
}

/// The type of a [`Room`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[non_exhaustive]
pub enum RoomTypeGroup {
    Playlists,
    HeadToHead,
    TeamVersus,
    TagCoop,
    TagTeamVersus,
}
