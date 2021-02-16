use crate::model::{Beatmap, Beatmapset, GameMode, User};

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BeatmapScores {
    scores: Vec<Score>,
    #[serde(alias = "userScore")]
    user_score: Option<BeatmapUserScore>,
}

#[derive(Debug, Deserialize)]
pub struct BeatmapUserScore {
    #[serde(rename = "position")]
    pos: usize,
    score: Score,
}

// TODO: Missing grade?
#[derive(Debug, Deserialize)]
pub struct Score {
    accuracy: f32,
    best_id: u32,
    created_at: DateTime<Utc>,
    max_combo: Option<u32>,
    map: Option<Beatmap>,
    mapset: Option<Beatmapset>,
    // #[serde(rename = "match")]
    // osu_match: _, // TODO
    mode: GameMode,
    mode_int: u32,
    mods: u32,
    perfect: bool,
    pp: Option<f32>,
    rank_country: Option<u32>,
    rank_global: Option<u32>,
    replay: String,
    score: u32,
    #[serde(rename = "id")]
    score_id: u32,
    statistics: ScoreStatistics,
    user: Option<User>,
    user_id: u32,
    weight: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ScoreStatistics {
    count_geki: u32,
    count_300: u32,
    count_katu: u32,
    count_100: u32,
    count_50: u32,
    count_miss: u32,
}
