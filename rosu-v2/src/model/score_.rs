use super::{
    beatmap::{Beatmap, BeatmapsetCompact},
    mods::GameMods,
    serde_,
    user_::UserCompact,
    GameMode, Grade,
};
use crate::{mods, prelude::ModeAsSeed, request::GetUser, Osu};

use serde::{
    de::{DeserializeSeed, Error as DeError},
    Deserialize, Deserializer,
};

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde_json::value::RawValue;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
pub(crate) struct BeatmapScores {
    pub(crate) scores: Vec<Score>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapUserScore {
    /// The position of the score within the requested beatmap ranking
    #[serde(rename = "position")]
    pub pos: usize,
    /// The details of the score
    pub score: Score,
}

impl BeatmapUserScore {
    /// Request the [`User`](crate::model::user::User) of the score
    #[inline]
    pub fn get_user<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        self.score.get_user(osu)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct Score {
    #[cfg_attr(feature = "serialize", serde(with = "serde_::adjust_acc"))]
    pub accuracy: f32,
    #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub ended_at: OffsetDateTime,
    pub passed: bool,
    #[cfg_attr(feature = "serialize", serde(rename = "rank"))]
    pub grade: Grade,
    #[cfg_attr(feature = "serialize", serde(rename = "beatmap_id"))]
    pub map_id: u32,
    pub max_combo: u32,
    #[cfg_attr(
        feature = "serialize",
        serde(rename = "beatmap", skip_serializing_if = "Option::is_none")
    )]
    pub map: Option<Beatmap>,
    #[cfg_attr(
        feature = "serialize",
        serde(rename = "beatmapset", skip_serializing_if = "Option::is_none")
    )]
    pub mapset: Option<BeatmapsetCompact>,
    pub mode: GameMode,
    pub mods: GameMods,
    pub perfect: bool,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub pp: Option<f32>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub rank_country: Option<u32>,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub rank_global: Option<u32>,
    pub replay: Option<bool>,
    pub score: u32,
    #[cfg_attr(feature = "serialize", serde(rename = "best_id"))]
    pub score_id: Option<u64>,
    pub statistics: ScoreStatistics,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub user: Option<UserCompact>,
    pub user_id: u32,
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub weight: Option<ScoreWeight>,
}

impl<'de> Deserialize<'de> for Score {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct ScoreRawMods {
            #[serde(with = "serde_::adjust_acc")]
            accuracy: f32,
            #[serde(with = "serde_::datetime")]
            ended_at: OffsetDateTime,
            passed: bool,
            #[serde(rename = "rank")]
            grade: Grade,
            #[serde(rename = "beatmap_id")]
            map_id: u32,
            max_combo: u32,
            #[serde(rename = "beatmap")]
            map: Option<Beatmap>,
            #[serde(rename = "beatmapset")]
            mapset: Option<BeatmapsetCompact>,
            #[serde(alias = "ruleset_id")]
            mode: GameMode,
            mods: Box<RawValue>,
            #[serde(alias = "legacy_perfect")]
            perfect: bool,
            pp: Option<f32>,
            rank_country: Option<u32>,
            rank_global: Option<u32>,
            replay: Option<bool>,
            #[serde(alias = "total_score")]
            score: u32,
            #[serde(rename = "best_id")]
            score_id: Option<u64>,
            statistics: ScoreStatistics,
            user: Option<UserCompact>,
            user_id: u32,
            weight: Option<ScoreWeight>,
        }

        let score_raw = <ScoreRawMods as serde::Deserialize>::deserialize(d)?;
        let mut d = serde_json::Deserializer::from_str(score_raw.mods.get());

        Ok(Score {
            mods: ModeAsSeed::<GameMods>::new(score_raw.mode)
                .deserialize(&mut d)
                .map_err(DeError::custom)?,
            accuracy: score_raw.accuracy,
            ended_at: score_raw.ended_at,
            passed: score_raw.passed,
            grade: score_raw.grade,
            map_id: score_raw.map_id,
            max_combo: score_raw.max_combo,
            map: score_raw.map,
            mapset: score_raw.mapset,
            mode: score_raw.mode,
            perfect: score_raw.perfect,
            pp: score_raw.pp,
            rank_country: score_raw.rank_country,
            rank_global: score_raw.rank_global,
            replay: score_raw.replay,
            score: score_raw.score,
            score_id: score_raw.score_id,
            statistics: score_raw.statistics,
            user: score_raw.user,
            user_id: score_raw.user_id,
            weight: score_raw.weight,
        })
    }
}

impl Score {
    #[inline]
    pub fn get_user<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        osu.user(self.user_id)
    }

    /// Count all hitobjects of the score i.e. for `GameMode::Osu` the amount 300s, 100s, 50s, and misses.
    ///
    /// Note: Includes tiny droplet (misses) for `GameMode::Catch`
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
    /// The accuracy is only required for `GameMode::Mania` and `GameMode::Catch` scores and is
    /// calculated internally if not provided.
    ///
    /// This method assumes the score to be a pass i.e. the amount of passed
    /// objects is equal to the beatmaps total amount of objects. Otherwise,
    /// it may produce an incorrect grade.
    pub fn grade(&self, accuracy: Option<f32>) -> Grade {
        let passed_objects = self.total_hits();

        match self.mode {
            GameMode::Osu => osu_grade(self, passed_objects),
            GameMode::Taiko => taiko_grade(self, passed_objects),
            GameMode::Catch => ctb_grade(self, accuracy),
            GameMode::Mania => mania_grade(self, passed_objects, accuracy),
        }
    }
}

impl PartialEq for Score {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
            && (self.ended_at.unix_timestamp() - other.ended_at.unix_timestamp()).abs() <= 2
            && self.score == other.score
    }
}

impl Eq for Score {}

#[derive(Deserialize)]
pub(crate) struct Scores {
    pub(crate) scores: Vec<Score>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(as = "Self")
)]
pub struct ScoreStatistics {
    #[serde(alias = "perfect", default)]
    pub count_geki: u32,
    #[serde(alias = "great", default)]
    pub count_300: u32,
    #[serde(alias = "good", alias = "small_tick_miss", default)]
    pub count_katu: u32,
    #[serde(alias = "ok", alias = "large_tick_hit", default)]
    pub count_100: u32,
    #[serde(alias = "meh", alias = "small_tick_hit", default)]
    pub count_50: u32,
    #[serde(alias = "miss", default)]
    pub count_miss: u32,
}

impl ScoreStatistics {
    /// Count all hitobjects of the score i.e. for `GameMode::Osu` the amount 300s, 100s, 50s, and misses.
    ///
    /// Note: Includes tiny droplet (misses) for `GameMode::Catch`
    pub fn total_hits(&self, mode: GameMode) -> u32 {
        let mut amount = self.count_300 + self.count_100 + self.count_miss;

        if mode != GameMode::Taiko {
            amount += self.count_50;

            if mode != GameMode::Osu {
                amount += self.count_katu;
                amount += (mode != GameMode::Catch) as u32 * self.count_geki;
            }
        }

        amount
    }

    /// Calculate the accuracy rounded to two decimal points i.e. `0 <= accuracy <= 100`
    pub fn accuracy(&self, mode: GameMode) -> f32 {
        let amount_objects = self.total_hits(mode) as f32;

        let (numerator, denumerator) = match mode {
            GameMode::Taiko => (
                0.5 * self.count_100 as f32 + self.count_300 as f32,
                amount_objects,
            ),
            GameMode::Catch => (
                (self.count_300 + self.count_100 + self.count_50) as f32,
                amount_objects,
            ),
            GameMode::Osu | GameMode::Mania => {
                let mut n =
                    (self.count_50 * 50 + self.count_100 * 100 + self.count_300 * 300) as f32;

                n += ((mode == GameMode::Mania) as u32
                    * (self.count_katu * 200 + self.count_geki * 300)) as f32;

                (n, amount_objects * 300.0)
            }
        };

        (10_000.0 * numerator / denumerator).round() / 100.0
    }
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(as = "Self")
)]
pub struct ScoreWeight {
    /// Percentage of the score's pp that will be added to the user's total pp between 0 and 100
    pub percentage: f32,
    /// PP **after** taking the percentage of the score's raw pp
    pub pp: f32,
}

fn osu_grade(score: &Score, passed_objects: u32) -> Grade {
    if score.statistics.count_300 == passed_objects {
        return if score.mods.contains_any(mods!(HD FL)) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let stats = &score.statistics;

    let ratio300 = stats.count_300 as f32 / passed_objects as f32;
    let ratio50 = stats.count_50 as f32 / passed_objects as f32;

    if ratio300 > 0.9 && ratio50 < 0.01 && stats.count_miss == 0 {
        if score.mods.contains_any(mods!(HD FL)) {
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
        return if score.mods.contains_any(mods!(HD FL FI)) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    if accuracy > 95.0 {
        if score.mods.contains_any(mods!(HD FL FI)) {
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

fn taiko_grade(score: &Score, passed_objects: u32) -> Grade {
    if score.statistics.count_300 == passed_objects {
        return if score.mods.contains_any(mods!(HD FL)) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let stats = &score.statistics;
    let ratio300 = stats.count_300 as f32 / passed_objects as f32;

    if ratio300 > 0.9 && stats.count_miss == 0 {
        if score.mods.contains_any(mods!(HD FL)) {
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

fn ctb_grade(score: &Score, accuracy: Option<f32>) -> Grade {
    let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    if (100.0 - accuracy).abs() <= std::f32::EPSILON {
        if score.mods.contains_any(mods!(HD FL)) {
            Grade::XH
        } else {
            Grade::X
        }
    } else if accuracy > 98.0 {
        if score.mods.contains_any(mods!(HD FL)) {
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
