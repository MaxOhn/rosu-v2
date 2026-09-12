use rosu_mods::GameMode;
use serde::Deserialize;
use time::OffsetDateTime;

/// The type of a [`MatchmakingPool`].
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MatchmakingPoolType {
    /// Quick play.
    QuickPlay,
    /// Ranked play.
    RankedPlay,
}

/// The result of a match in a [`MatchmakingPool`].
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "snake_case")]
pub enum MatchmakingResult {
    Win,
    Loss,
    Draw,
}

/// A matchmaking pool users can play in.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MatchmakingPool {
    /// Whether the pool is currently active.
    pub active: bool,
    /// Unique identifier of the pool.
    #[serde(rename = "id")]
    pub pool_id: u32,
    /// Display name of the pool.
    pub name: String,
    /// The game mode of the pool.
    #[serde(rename = "ruleset_id")]
    pub mode: GameMode,
    /// The type of the pool.
    #[serde(rename = "type")]
    pub kind: MatchmakingPoolType,
    /// The variant of the pool (`0` for no variant).
    pub variant_id: u32,
}

/// A single elo history entry of a user in a [`MatchmakingPool`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MatchmakingUserEloHistory {
    /// The elo of the user after the match.
    pub elo_after: i32,
    /// Unique identifier of the entry.
    #[serde(rename = "id")]
    pub entry_id: u32,
    /// When the entry was created.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "super::serde_util::option_datetime"
    )]
    pub created_at: Option<OffsetDateTime>,
    /// The result of the match.
    pub result: MatchmakingResult,
}

/// The ranked play (matchmaking) stats of a user in a single pool.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MatchmakingUserStats {
    /// The number of first placements.
    pub first_placements: u32,
    /// Whether the rating of the user is still provisional.
    pub is_rating_provisional: bool,
    /// The number of plays.
    pub plays: u32,
    /// Unique identifier of the pool the stats belong to.
    pub pool_id: u32,
    /// The rank of the user in the pool (`1` being the highest).
    pub rank: u32,
    /// The percentile rank of the user in the pool (`0.0` to `1.0`).
    pub rank_percent: f64,
    /// The current rating of the user in the pool.
    pub rating: i32,
    /// The total points of the user in the pool.
    pub total_points: i32,
    /// Unique identifier of the user.
    pub user_id: u32,

    /// The pool the stats belong to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<MatchmakingPool>,
    /// Recent elo history entries of the user in the pool.
    ///
    /// `None` if the pool is not active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recent_history: Option<Vec<MatchmakingUserEloHistory>>,
}
