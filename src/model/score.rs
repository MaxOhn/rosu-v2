use super::{
    beatmap::{Beatmap, BeatmapsetCompact},
    deflate_acc, inflate_acc,
    user::UserCompact,
    GameMode, GameMods, Grade,
};
use crate::{request::GetUser, Osu};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(crate) struct BeatmapScores {
    pub(crate) scores: Vec<Score>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapUserScore {
    /// Position of the score in the map's global leaderboard
    #[serde(rename = "position")]
    pub pos: usize,
    pub score: Score,
}

impl BeatmapUserScore {
    #[inline]
    pub fn get_user<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        self.score.get_user(osu)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Score {
    #[serde(deserialize_with = "inflate_acc", serialize_with = "deflate_acc")]
    pub accuracy: f32,
    pub created_at: DateTime<Utc>,
    #[serde(rename = "rank")]
    pub grade: Grade,
    pub max_combo: u32,
    #[serde(default, rename = "beatmap", skip_serializing_if = "Option::is_none")]
    pub map: Option<Beatmap>,
    #[serde(
        default,
        rename = "beatmapset",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapset: Option<BeatmapsetCompact>,
    pub mode: GameMode,
    pub mods: GameMods,
    pub perfect: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pp: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_country: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_global: Option<u32>,
    pub replay: bool,
    pub score: u32,
    #[serde(rename = "id")]
    pub score_id: u64,
    pub statistics: ScoreStatistics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserCompact>,
    pub user_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<ScoreWeight>,
}

impl Score {
    #[inline]
    pub fn get_user<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        osu.user(self.user_id)
    }

    /// Count all hitobjects of the score i.e. for `GameMode::STD` the amount 300s, 100s, 50s, and misses.
    ///
    /// Note: Includes tiny droplet (misses) for `GameMode::CTB`
    #[inline]
    pub fn total_hits(&self) -> u32 {
        self.statistics.total_hits(self.mode)
    }

    /// Calculate the accuracy i.e. `0 <= accuracy <= 100`
    #[inline]
    pub fn accuracy(&self) -> f32 {
        self.statistics.accuracy(self.mode)
    }

    /// Calculate the grade of the score.
    /// Should only be used in case the score was modified and the internal `grade` field is no longer correct.
    ///
    /// The accuracy is only required for non-`GameMode::STD` scores and is
    /// calculated internally if not provided.
    ///
    /// This method assumes the score to be a pass i.e. the amount of passed
    /// objects is equal to the beatmaps total amount of objects. Otherwise,
    /// it may produce an incorrect grade.
    pub fn grade(&self, accuracy: Option<f32>) -> Grade {
        let passed_objects = self.total_hits();

        match self.mode {
            GameMode::STD => osu_grade(self, passed_objects),
            GameMode::TKO => taiko_grade(self, passed_objects, accuracy),
            GameMode::CTB => ctb_grade(self, accuracy),
            GameMode::MNA => mania_grade(self, passed_objects, accuracy),
        }
    }
}

impl PartialEq for Score {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id && self.created_at == other.created_at
    }
}

impl Eq for Score {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScoreStatistics {
    pub count_geki: u32,
    pub count_300: u32,
    pub count_katu: u32,
    pub count_100: u32,
    pub count_50: u32,
    pub count_miss: u32,
}

impl ScoreStatistics {
    /// Count all hitobjects of the score i.e. for `GameMode::STD` the amount 300s, 100s, 50s, and misses.
    ///
    /// Note: Includes tiny droplet (misses) for `GameMode::CTB`
    pub fn total_hits(&self, mode: GameMode) -> u32 {
        let mut amount = self.count_300 + self.count_100 + self.count_miss;

        if mode != GameMode::TKO {
            amount += self.count_50;

            if mode != GameMode::STD {
                amount += self.count_katu;
                amount += (mode != GameMode::CTB) as u32 * self.count_geki;
            }
        }

        amount
    }

    /// Calculate the accuracy rounded to two decimal points i.e. `0 <= accuracy <= 100`
    pub fn accuracy(&self, mode: GameMode) -> f32 {
        let amount_objects = self.total_hits(mode) as f32;

        let (numerator, denumerator) = match mode {
            GameMode::TKO => (
                0.5 * self.count_100 as f32 + self.count_300 as f32,
                amount_objects,
            ),
            GameMode::CTB => (
                (self.count_300 + self.count_100 + self.count_50) as f32,
                amount_objects,
            ),
            GameMode::STD | GameMode::MNA => {
                let mut n =
                    (self.count_50 * 50 + self.count_100 * 100 + self.count_300 * 300) as f32;

                n += ((mode == GameMode::MNA) as u32
                    * (self.count_katu * 200 + self.count_geki * 300)) as f32;

                (n, amount_objects * 300.0)
            }
        };

        (10_000.0 * numerator / denumerator).round() / 100.0
    }
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScoreWeight {
    /// Percentage of the score's pp that will be added to the user's total pp between 0 and 100
    pub percentage: f32,
    /// PP **after** taking the percentage of the score's raw pp
    pub pp: f32,
}

const HDFL: GameMods =
    GameMods::from_bits_truncate(GameMods::Hidden.bits() + GameMods::Flashlight.bits());
const HDFLFI: GameMods = GameMods::from_bits_truncate(HDFL.bits() + GameMods::FadeIn.bits());

fn osu_grade(score: &Score, passed_objects: u32) -> Grade {
    if score.statistics.count_300 == passed_objects {
        return if score.mods.intersects(HDFL) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let stats = &score.statistics;

    let ratio300 = stats.count_300 as f32 / passed_objects as f32;
    let ratio50 = stats.count_50 as f32 / passed_objects as f32;

    if ratio300 > 0.9 && ratio50 < 0.01 && stats.count_miss == 0 {
        if score.mods.intersects(HDFL) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if ratio300 > 0.9 || (ratio300 > 0.8 && stats.count_miss == 0) {
        Grade::A
    } else if ratio300 > 0.8 || (ratio300 > 0.7 && stats.count_miss == 0) {
        Grade::B
    } else if ratio300 > 0.6 {
        Grade::C
    } else {
        Grade::D
    }
}

fn mania_grade(score: &Score, passed_objects: u32, accuracy: Option<f32>) -> Grade {
    if score.statistics.count_geki == passed_objects {
        return if score.mods.intersects(HDFLFI) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    if accuracy > 95.0 {
        if score.mods.intersects(HDFLFI) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if accuracy > 90.0 {
        Grade::A
    } else if accuracy > 80.0 {
        Grade::B
    } else if accuracy > 70.0 {
        Grade::C
    } else {
        Grade::D
    }
}

fn taiko_grade(score: &Score, passed_objects: u32, accuracy: Option<f32>) -> Grade {
    if score.statistics.count_300 == passed_objects {
        return if score.mods.intersects(HDFL) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    if accuracy > 95.0 {
        if score.mods.intersects(HDFL) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if accuracy > 90.0 {
        Grade::A
    } else if accuracy > 80.0 {
        Grade::B
    } else {
        Grade::C
    }
}

fn ctb_grade(score: &Score, accuracy: Option<f32>) -> Grade {
    let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    if (100.0 - accuracy).abs() <= std::f32::EPSILON {
        if score.mods.intersects(HDFL) {
            Grade::XH
        } else {
            Grade::X
        }
    } else if accuracy > 98.0 {
        if score.mods.intersects(HDFL) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if accuracy > 94.0 {
        Grade::A
    } else if accuracy > 90.0 {
        Grade::B
    } else if accuracy > 85.0 {
        Grade::C
    } else {
        Grade::D
    }
}
