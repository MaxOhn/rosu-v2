use super::{
    beatmap_::{BeatmapExtended, Beatmapset},
    mods::GameMods,
    serde_,
    user::User,
    GameMode, Grade,
};
use crate::{mods, prelude::ModeAsSeed, request::GetUser, Osu};

use serde::{
    de::{DeserializeSeed, Error as DeError, IgnoredAny},
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
    /// Request the [`UserExtended`](crate::model::user::UserExtended) of the score
    #[inline]
    pub fn get_user<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        self.score.get_user(osu)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct Score {
    pub ranked: Option<bool>,
    pub preserve: Option<bool>,
    pub maximum_statistics: ScoreStatistics,
    pub mods: GameMods,
    pub statistics: ScoreStatistics,
    #[cfg_attr(feature = "serialize", serde(rename = "beatmap_id"))]
    pub map_id: u32,
    pub best_id: Option<u64>,
    pub id: u64,
    #[cfg_attr(feature = "serialize", serde(rename = "rank"))]
    pub grade: Grade,
    #[cfg_attr(feature = "serialize", serde(rename = "type"))]
    pub kind: Box<str>,
    pub user_id: u32,
    #[cfg_attr(feature = "serialize", serde(with = "serde_::adjust_acc"))]
    pub accuracy: f32,
    pub build_id: Option<u32>,
    #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub ended_at: OffsetDateTime,
    pub has_replay: bool,
    pub is_perfect_combo: bool,
    pub legacy_perfect: Option<bool>,
    pub legacy_score_id: Option<u64>,
    #[cfg_attr(feature = "serialize", serde(rename = "legacy_total_score"))]
    pub legacy_score: u32,
    pub max_combo: u32,
    pub passed: bool,
    pub pp: Option<f32>,
    #[cfg_attr(feature = "serialize", serde(rename = "ruleset_id"))]
    pub mode: GameMode,
    #[cfg_attr(feature = "serialize", serde(with = "serde_::option_datetime"))]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub started_at: Option<OffsetDateTime>,
    #[cfg_attr(feature = "serialize", serde(rename = "total_score"))]
    pub score: u32,
    pub replay: bool,
    pub current_user_attributes: UserAttributes,
    #[cfg_attr(feature = "serialize", serde(rename = "beatmap"))]
    pub map: Option<Box<BeatmapExtended>>,
    #[cfg_attr(feature = "serialize", serde(rename = "beatmapset"))]
    pub mapset: Option<Box<Beatmapset>>,
    pub rank_global: Option<u32>,
    pub user: Option<Box<User>>,
    pub weight: Option<ScoreWeight>,
}

impl<'de> Deserialize<'de> for Score {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ScoreRawMods {
            ranked: Option<bool>,
            preserve: Option<bool>,
            #[serde(default)]
            maximum_statistics: ScoreStatistics,
            mods: Box<RawValue>,
            statistics: ScoreStatistics,
            #[serde(rename = "beatmap_id")]
            map_id: Option<u32>, // not available in legacy scores
            best_id: Option<u64>,
            id: u64,
            #[serde(rename = "rank")]
            grade: Grade,
            #[serde(rename = "type")]
            kind: Box<str>,
            user_id: u32,
            #[serde(with = "serde_::adjust_acc")]
            accuracy: f32,
            build_id: Option<u32>,
            #[serde(alias = "created_at", with = "serde_::datetime")]
            ended_at: OffsetDateTime,
            has_replay: Option<bool>,       // not available in legacy scores
            is_perfect_combo: Option<bool>, // not available in legacy scores
            #[serde(alias = "perfect")]
            legacy_perfect: Option<bool>,
            legacy_score_id: Option<u64>,
            #[serde(default, rename = "legacy_total_score")]
            legacy_score: u32, // not available in legacy scores
            max_combo: u32,
            passed: bool,
            pp: Option<f32>,
            #[serde(rename = "mode")]
            _mode: Option<IgnoredAny>, // only available in legacy scores
            #[serde(rename = "ruleset_id", alias = "mode_int")]
            mode: GameMode,
            #[serde(default, with = "serde_::option_datetime")]
            started_at: Option<OffsetDateTime>,
            #[serde(rename = "total_score", alias = "score")]
            score: u32,
            replay: bool,
            current_user_attributes: UserAttributes,
            #[serde(rename = "beatmap")]
            map: Option<Box<BeatmapExtended>>,
            #[serde(rename = "beatmapset")]
            mapset: Option<Box<Beatmapset>>,
            rank_global: Option<u32>,
            user: Option<Box<User>>,
            // TODO: This is just a temporary fix for <https://github.com/ppy/osu-web/issues/10932>.
            // Once the issue is resolved, `Option<ScoreWeight>` can be used again.
            weight: Option<MaybeWeight>,
        }

        #[derive(Deserialize)]
        struct MaybeWeight {
            percentage: f32,
            pp: Option<f32>,
        }

        let score_raw = <ScoreRawMods as serde::Deserialize>::deserialize(d)?;
        let mut d = serde_json::Deserializer::from_str(score_raw.mods.get());

        // Lazer scores don't have `mode` specified; only `ruleset_id`
        // so we use that to determine if the score is legacy.
        // Maybe using `type` would be better? As of now it seems unreliable.
        let is_legacy = score_raw._mode.is_some();

        Ok(Score {
            ranked: score_raw.ranked,
            preserve: score_raw.preserve,
            maximum_statistics: score_raw.maximum_statistics,
            mods: ModeAsSeed::<GameMods>::new(score_raw.mode)
                .deserialize(&mut d)
                .map_err(DeError::custom)?,
            statistics: score_raw.statistics,
            map_id: score_raw
                .map_id
                .or_else(|| score_raw.map.as_ref().map(|map| map.map_id))
                .unwrap_or(0),
            best_id: score_raw.best_id,
            id: score_raw.id,
            grade: score_raw.grade,
            kind: score_raw.kind,
            user_id: score_raw.user_id,
            accuracy: score_raw.accuracy,
            build_id: score_raw.build_id,
            ended_at: score_raw.ended_at,
            has_replay: score_raw.has_replay.unwrap_or(score_raw.replay),
            is_perfect_combo: score_raw
                .is_perfect_combo
                .or(score_raw.legacy_perfect)
                .unwrap_or(false),
            legacy_perfect: score_raw.legacy_perfect,
            legacy_score_id: score_raw
                .legacy_score_id
                .or_else(|| is_legacy.then_some(score_raw.id)),
            legacy_score: if is_legacy {
                score_raw.score
            } else {
                score_raw.legacy_score
            },
            max_combo: score_raw.max_combo,
            passed: score_raw.passed,
            pp: score_raw.pp,
            mode: score_raw.mode,
            started_at: score_raw.started_at,
            score: score_raw.score,
            replay: score_raw.replay,
            current_user_attributes: score_raw.current_user_attributes,
            map: score_raw.map,
            mapset: score_raw.mapset,
            rank_global: score_raw.rank_global,
            user: score_raw.user,
            weight: score_raw.weight.and_then(|weight| {
                Some(ScoreWeight {
                    percentage: weight.percentage,
                    pp: weight.pp?,
                })
            }),
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
    /// Note: Includes tiny droplet (misses) for `GameMode::Catch`.
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
    ///
    /// The accuracy is calculated internally if not provided.
    ///
    /// This method assumes the score to be a pass i.e. the amount of passed
    /// objects is equal to the beatmaps total amount of objects. Otherwise,
    /// it may produce an incorrect grade.
    pub fn grade(&self, accuracy: Option<f32>) -> Grade {
        let passed_objects = self.total_hits();

        match self.mode {
            GameMode::Osu => osu_grade(self, passed_objects, accuracy),
            GameMode::Taiko => taiko_grade(self, passed_objects, accuracy),
            GameMode::Catch => catch_grade(self, accuracy),
            GameMode::Mania => mania_grade(self, passed_objects, accuracy),
        }
    }

    /// Calculate the legacy grade of the score.
    ///
    /// The accuracy is calculated internally if not provided.
    ///
    /// This method assumes the score to be a pass i.e. the amount of passed
    /// objects is equal to the beatmaps total amount of objects. Otherwise,
    /// it may produce an incorrect grade.
    pub fn legacy_grade(&self, accuracy: Option<f32>) -> Grade {
        let passed_objects = self.total_hits();

        match self.mode {
            GameMode::Osu => osu_grade_legacy(self, passed_objects),
            GameMode::Taiko => taiko_grade_legacy(self, passed_objects),
            GameMode::Catch => catch_grade_legacy(self, accuracy),
            GameMode::Mania => mania_grade_legacy(self, passed_objects, accuracy),
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct ScoreStatistics {
    #[serde(
        default,
        alias = "count_miss",
        deserialize_with = "serde_::from_option::deserialize"
    )]
    pub miss: u32,
    #[serde(
        default,
        alias = "count_50",
        deserialize_with = "serde_::from_option::deserialize"
    )]
    pub meh: u32,
    #[serde(
        default,
        alias = "count_100",
        deserialize_with = "serde_::from_option::deserialize"
    )]
    pub ok: u32,
    #[serde(
        default,
        alias = "count_katu",
        deserialize_with = "serde_::from_option::deserialize"
    )]
    pub good: u32,
    #[serde(
        default,
        alias = "count_300",
        deserialize_with = "serde_::from_option::deserialize"
    )]
    pub great: u32,
    #[serde(
        default,
        alias = "count_geki",
        deserialize_with = "serde_::from_option::deserialize"
    )]
    pub perfect: u32,
    #[serde(default)]
    pub large_tick_hit: u32,
    #[serde(default)]
    pub large_tick_miss: u32,
    #[serde(default)]
    pub small_tick_hit: u32,
    #[serde(default)]
    pub small_tick_miss: u32,
    #[serde(default)]
    pub ignore_hit: u32,
    #[serde(default)]
    pub ignore_miss: u32,
    #[serde(default)]
    pub large_bonus: u32,
    #[serde(default)]
    pub small_bonus: u32,
    #[serde(default)]
    pub slider_tail_hit: u32,
    #[serde(default)]
    pub combo_break: u32,
    #[serde(default)]
    pub legacy_combo_increase: u32,
}

impl ScoreStatistics {
    /// Count all hitobjects of the score i.e. for `GameMode::Osu` the amount 300s, 100s, 50s, and misses.
    ///
    /// Note: Includes tiny droplet (misses) for `GameMode::Catch`.
    pub fn total_hits(&self, mode: GameMode) -> u32 {
        match mode {
            GameMode::Osu => self.ok + self.meh + self.great + self.miss,
            GameMode::Taiko => self.ok + self.great + self.miss,
            GameMode::Catch => self.miss + self.great + self.large_tick_hit + self.small_tick_hit,
            GameMode::Mania => {
                self.ok + self.meh + self.good + self.miss + self.great + self.perfect
            }
        }
    }

    /// Calculate the accuracy rounded to two decimal points i.e. `0 <= accuracy <= 100`
    pub fn accuracy(&self, mode: GameMode) -> f32 {
        let numerator;
        let denominator;

        match mode {
            GameMode::Osu => {
                numerator = (self.meh * 50 + self.ok * 100 + self.great * 300) as f32;
                denominator = (self.total_hits(mode) * 300) as f32;
            }
            GameMode::Taiko => {
                numerator = 0.5 * self.ok as f32 + self.great as f32;
                denominator = self.total_hits(mode) as f32;
            }
            GameMode::Catch => {
                numerator = (self.large_tick_hit + self.great + self.small_tick_hit) as f32;
                denominator = self.total_hits(mode) as f32;
            }
            GameMode::Mania => {
                numerator = (self.meh * 50
                    + self.ok * 100
                    + self.good * 200
                    + (self.great + self.perfect) * 300) as f32;

                denominator = (self.total_hits(mode) * 300) as f32;
            }
        };

        (10_000.0 * numerator / denominator).round() / 100.0
    }

    /// Turn [`ScoreStatistics`] into [`LegacyScoreStatistics`]
    pub fn as_legacy(&self, mode: GameMode) -> LegacyScoreStatistics {
        let mut count_geki = 0;
        let mut count_katu = 0;
        let count_300 = self.great;
        let count_100;
        let mut count_50 = 0;
        let count_miss = self.miss;

        match mode {
            GameMode::Osu => {
                count_100 = self.ok;
                count_50 = self.meh;
            }
            GameMode::Taiko => count_100 = self.ok,
            GameMode::Catch => {
                count_100 = self.large_tick_hit;
                count_50 = self.small_tick_hit;
                count_katu = self.small_tick_miss;
            }
            GameMode::Mania => {
                count_geki = self.perfect;
                count_katu = self.good;
                count_100 = self.ok;
                count_50 = self.meh;
            }
        }

        LegacyScoreStatistics {
            count_geki,
            count_katu,
            count_300,
            count_100,
            count_50,
            count_miss,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct LegacyScoreStatistics {
    pub count_geki: u32,
    pub count_katu: u32,
    pub count_300: u32,
    pub count_100: u32,
    pub count_50: u32,
    pub count_miss: u32,
}

impl LegacyScoreStatistics {
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

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct UserAttributes {
    pub pin: Option<()>,
}

fn osu_grade(score: &Score, passed_objects: u32, accuracy: Option<f32>) -> Grade {
    if score.statistics.great == passed_objects {
        return if score.mods.contains_any(mods!(HD FL)) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    if accuracy >= 95.0 && score.statistics.miss == 0 {
        if score.mods.contains_any(mods!(HD FL FI)) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if accuracy >= 90.0 {
        Grade::A
    } else if accuracy >= 80.0 {
        Grade::B
    } else if accuracy >= 70.0 {
        Grade::C
    } else {
        Grade::D
    }
}

fn taiko_grade(score: &Score, passed_objects: u32, accuracy: Option<f32>) -> Grade {
    osu_grade(score, passed_objects, accuracy)
}

fn catch_grade(score: &Score, accuracy: Option<f32>) -> Grade {
    catch_grade_legacy(score, accuracy)
}

fn mania_grade(score: &Score, passed_objects: u32, accuracy: Option<f32>) -> Grade {
    mania_grade_legacy(score, passed_objects, accuracy)
}

fn osu_grade_legacy(score: &Score, passed_objects: u32) -> Grade {
    if score.statistics.great == passed_objects {
        return if score.mods.contains_any(mods!(HD FL)) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let stats = &score.statistics;

    let ratio300 = stats.great as f32 / passed_objects as f32;
    let ratio50 = stats.meh as f32 / passed_objects as f32;

    if ratio300 > 0.9 && ratio50 < 0.01 && stats.miss == 0 {
        if score.mods.contains_any(mods!(HD FL)) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if ratio300 > 0.9 || (ratio300 > 0.8 && stats.miss == 0) {
        Grade::A
    } else if ratio300 > 0.8 || (ratio300 > 0.7 && stats.miss == 0) {
        Grade::B
    } else if ratio300 > 0.6 {
        Grade::C
    } else {
        Grade::D
    }
}

fn taiko_grade_legacy(score: &Score, passed_objects: u32) -> Grade {
    if score.statistics.great == passed_objects {
        return if score.mods.contains_any(mods!(HD FL)) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let stats = &score.statistics;
    let ratio300 = stats.great as f32 / passed_objects as f32;

    if ratio300 > 0.9 && stats.miss == 0 {
        if score.mods.contains_any(mods!(HD FL)) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if ratio300 > 0.9 || (ratio300 > 0.8 && stats.miss == 0) {
        Grade::A
    } else if ratio300 > 0.8 || (ratio300 > 0.7 && stats.miss == 0) {
        Grade::B
    } else if ratio300 > 0.6 {
        Grade::C
    } else {
        Grade::D
    }
}

fn catch_grade_legacy(score: &Score, accuracy: Option<f32>) -> Grade {
    let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    if (100.0 - accuracy).abs() < std::f32::EPSILON {
        if score.mods.contains_any(mods!(HD FL)) {
            Grade::XH
        } else {
            Grade::X
        }
    } else if accuracy >= 98.0 {
        if score.mods.contains_any(mods!(HD FL)) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if accuracy >= 94.0 {
        Grade::A
    } else if accuracy >= 90.0 {
        Grade::B
    } else if accuracy >= 85.0 {
        Grade::C
    } else {
        Grade::D
    }
}

fn mania_grade_legacy(score: &Score, passed_objects: u32, accuracy: Option<f32>) -> Grade {
    if score.statistics.perfect == passed_objects {
        return if score.mods.contains_any(mods!(HD FL FI)) {
            Grade::XH
        } else {
            Grade::X
        };
    }

    let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    if accuracy >= 95.0 {
        if score.mods.contains_any(mods!(HD FL FI)) {
            Grade::SH
        } else {
            Grade::S
        }
    } else if accuracy >= 90.0 {
        Grade::A
    } else if accuracy >= 80.0 {
        Grade::B
    } else if accuracy >= 70.0 {
        Grade::C
    } else {
        Grade::D
    }
}
