use crate::model::{Beatmap, Beatmapset, GameMode, GameMods, UserCompact};

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BeatmapScores {
    pub scores: Vec<Score>,
    #[serde(alias = "userScore")]
    pub user_score: Option<BeatmapUserScore>,
}

#[derive(Debug, Deserialize)]
pub struct BeatmapUserScore {
    #[serde(rename = "position")]
    pub pos: usize,
    pub score: Score,
}

// TODO: Missing grade?
#[derive(Debug, Deserialize)]
pub struct Score {
    pub accuracy: f32,
    pub best_id: u32,
    pub created_at: DateTime<Utc>,
    pub max_combo: Option<u32>,
    pub map: Option<Beatmap>,
    pub mapset: Option<Beatmapset>,
    // #[serde(rename = "match")]
    // pub osu_match: _, // TODO
    pub mode: GameMode,
    pub mode_int: u32,
    pub mods: GameMods,
    pub perfect: bool,
    pub pp: Option<f32>,
    pub rank_country: Option<u32>,
    pub rank_global: Option<u32>,
    pub replay: bool,
    pub score: u32,
    #[serde(rename = "id")]
    pub score_id: u32,
    pub statistics: ScoreStatistics,
    pub user: Option<UserCompact>,
    pub user_id: u32,
    pub weight: Option<ScoreWeight>,
}

#[derive(Debug, Deserialize)]
pub struct ScoreStatistics {
    pub count_geki: u32,
    pub count_300: u32,
    pub count_katu: u32,
    pub count_100: u32,
    pub count_50: u32,
    pub count_miss: u32,
}

#[derive(Debug, Deserialize)]
pub struct ScoreWeight {
    percentage: f32,
    pp: f32,
}
