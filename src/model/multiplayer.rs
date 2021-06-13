use super::Cursor;

use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiplayerScore {
    accuracy: f32,
    // #[serde(rename = "rank")]
    // pub grade: Grade,
    #[serde(rename = "beatmap_id")]
    pub map_id: u32,
    pub max_combo: u32, // TODO: Option?
    // pub mods: u32, // TODO
    pub passed: bool,
    #[serde(rename = "playlist_item_id")]
    pub playlist_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    pub room_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scores_around: Option<ScoresAround>,
    #[serde(rename = "id")]
    pub score_id: u32,
    // pub statistics: u32, // TODO
    pub total_score: u32,
    pub user_id: u32,
    // pub user: u32, // TODO
}

impl PartialEq for MultiplayerScore {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.score_id == other.score_id
    }
}

impl Eq for MultiplayerScore {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiplayerScores {
    pub(crate) cursor: Option<Cursor>,
    // params: u32, // TODO
    pub scores: Vec<MultiplayerScore>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_score: Option<MultiplayerScore>,
}

impl Deref for MultiplayerScores {
    type Target = [MultiplayerScore];

    fn deref(&self) -> &Self::Target {
        &self.scores
    }
}

impl DerefMut for MultiplayerScores {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scores
    }
}

impl IntoIterator for MultiplayerScores {
    type Item = MultiplayerScore;
    type IntoIter = IntoIter<MultiplayerScore>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.scores.into_iter()
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ScoresAround {
    #[serde(rename = "higher")]
    Higher,
    #[serde(rename = "lower")]
    Lower,
}
