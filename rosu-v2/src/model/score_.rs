use std::fmt::{Formatter, Result as FmtResult};

use super::{
    beatmap_::{Beatmap, Beatmapset},
    mods::GameMods,
    serde_,
    user::User,
    GameMode, Grade,
};
use crate::{mods, prelude::ModeAsSeed, request::GetUser, Osu};

use serde::{
    de::{
        value::{MapAccessDeserializer, MapDeserializer},
        DeserializeSeed, Error as DeError, MapAccess, Visitor,
    },
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
    pub maximum_statistics: MaximumStatistics,
    pub mods: GameMods,
    pub statistics: ScoreStatistics,
    pub map_id: u32,
    pub best_id: Option<u64>,
    pub id: u64,
    pub grade: Grade,
    pub kind: Box<str>,
    pub user_id: u32,
    pub accuracy: f32,
    pub build_id: Option<()>,
    pub ended_at: OffsetDateTime,
    pub has_replay: bool,
    pub legacy_perfect: Option<bool>,
    pub legacy_score_id: u64,
    pub legacy_score: u32,
    pub max_combo: u32,
    pub passed: bool,
    pub pp: Option<f32>,
    pub mode: GameMode,
    pub started_at: Option<()>,
    pub score: u32,
    pub replay: bool,
    pub current_user_attributes: UserAttributes,
    pub map: Option<Box<Beatmap>>,
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
            maximum_statistics: Box<RawValue>,
            mods: Box<RawValue>,
            statistics: Box<RawValue>,
            #[serde(rename = "beatmap_id")]
            map_id: u32,
            best_id: Option<u64>,
            id: u64,
            #[serde(rename = "rank")]
            grade: Grade,
            #[serde(rename = "type")]
            kind: Box<str>,
            user_id: u32,
            #[serde(with = "serde_::adjust_acc")]
            accuracy: f32,
            build_id: Option<()>,
            #[serde(with = "serde_::datetime")]
            ended_at: OffsetDateTime,
            has_replay: bool,
            legacy_perfect: Option<bool>,
            legacy_score_id: u64,
            #[serde(rename = "legacy_total_score")]
            legacy_score: u32,
            max_combo: u32,
            passed: bool,
            pp: Option<f32>,
            #[serde(rename = "ruleset_id")]
            mode: GameMode,
            started_at: Option<()>,
            #[serde(rename = "total_score")]
            score: u32,
            replay: bool,
            current_user_attributes: UserAttributes,
            #[serde(rename = "beatmap")]
            map: Option<Box<Beatmap>>,
            #[serde(rename = "beatmapset")]
            mapset: Option<Box<Beatmapset>>,
            rank_global: Option<u32>,
            user: Option<Box<User>>,
            weight: Option<ScoreWeight>,
        }

        let score_raw = <ScoreRawMods as serde::Deserialize>::deserialize(d)?;

        Ok(Score {
            maximum_statistics: ModeAsSeed::<MaximumStatistics>::new(score_raw.mode)
                .deserialize(&mut serde_json::Deserializer::from_str(
                    score_raw.maximum_statistics.get(),
                ))
                .map_err(DeError::custom)?,
            mods: ModeAsSeed::<GameMods>::new(score_raw.mode)
                .deserialize(&mut serde_json::Deserializer::from_str(
                    score_raw.mods.get(),
                ))
                .map_err(DeError::custom)?,
            statistics: ModeAsSeed::<ScoreStatistics>::new(score_raw.mode)
                .deserialize(&mut serde_json::Deserializer::from_str(
                    score_raw.statistics.get(),
                ))
                .map_err(DeError::custom)?,
            map_id: score_raw.map_id,
            best_id: score_raw.best_id,
            id: score_raw.id,
            grade: score_raw.grade,
            kind: score_raw.kind,
            user_id: score_raw.user_id,
            accuracy: score_raw.accuracy,
            build_id: score_raw.build_id,
            ended_at: score_raw.ended_at,
            has_replay: score_raw.has_replay,
            legacy_perfect: score_raw.legacy_perfect,
            legacy_score_id: score_raw.legacy_score_id,
            legacy_score: score_raw.legacy_score,
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
    /// Note: Includes tiny droplet (misses) for `GameMode::Catch`.
    #[inline]
    pub fn total_hits(&self) -> u32 {
        self.statistics.total_hits()
    }

    /// Calculate the accuracy i.e. `0 <= accuracy <= 100`
    #[inline]
    pub fn accuracy(&self) -> f32 {
        self.statistics.accuracy()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScoreStatistics {
    Osu(OsuStatistics),
    Taiko(TaikoStatistics),
    Catch(CatchStatistics),
    Mania(ManiaStatistics),
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OsuStatistics {
    #[serde(default)]
    pub ok: u32,
    #[serde(default)]
    pub meh: u32,
    #[serde(default)]
    pub miss: u32,
    #[serde(default)]
    pub great: u32,
    #[serde(default)]
    pub ignore_hit: u32,
    #[serde(default)]
    pub ignore_miss: u32,
    #[serde(default)]
    pub small_bonus: u32,
    #[serde(default)]
    pub large_tick_hit: u32,
    #[serde(default)]
    pub large_tick_miss: u32,
}

impl OsuStatistics {
    /// Count all hitobjects of the score.
    pub fn total_hits(&self) -> u32 {
        self.ok + self.meh + self.great + self.miss
    }

    /// Calculate the accuracy rounded to two decimal points i.e. `0 <= accuracy <= 100`
    pub fn accuracy(&self) -> f32 {
        let numerator = (self.meh * 50 + self.ok * 100 + self.great * 300) as f32;
        let denominator = (self.total_hits() * 300) as f32;

        (10_000.0 * numerator / denominator).round() / 100.0
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TaikoStatistics {
    #[serde(default)]
    pub ok: u32,
    #[serde(default)]
    pub miss: u32,
    #[serde(default)]
    pub great: u32,
    #[serde(default)]
    pub ignore_hit: u32,
    #[serde(default)]
    pub large_bonus: u32,
    #[serde(default)]
    pub small_bonus: u32,
}

impl TaikoStatistics {
    /// Count all hitobjects of the score.
    pub fn total_hits(&self) -> u32 {
        self.ok + self.great + self.miss
    }

    /// Calculate the accuracy rounded to two decimal points i.e. `0 <= accuracy <= 100`
    pub fn accuracy(&self) -> f32 {
        let numerator = 0.5 * self.ok as f32 + self.great as f32;
        let denominator = self.total_hits() as f32;

        (10_000.0 * numerator / denominator).round() / 100.0
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CatchStatistics {
    #[serde(default)]
    pub miss: u32,
    #[serde(default)]
    pub great: u32,
    #[serde(default)]
    pub large_tick_hit: u32,
    #[serde(default)]
    pub small_tick_hit: u32,
    #[serde(default)]
    pub small_tick_miss: u32,
}

impl CatchStatistics {
    /// Count all hitobjects of the score.
    ///
    /// Note: Includes tiny droplet (misses).
    pub fn total_hits(&self) -> u32 {
        self.miss + self.great + self.large_tick_hit + self.small_tick_hit
    }

    /// Calculate the accuracy rounded to two decimal points i.e. `0 <= accuracy <= 100`
    pub fn accuracy(&self) -> f32 {
        // TODO: test
        let numerator = (self.large_tick_hit + self.great + self.small_tick_hit) as f32;
        let denominator = self.total_hits() as f32;

        (10_000.0 * numerator / denominator).round() / 100.0
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ManiaStatistics {
    #[serde(default)]
    pub ok: u32,
    #[serde(default)]
    pub meh: u32,
    #[serde(default)]
    pub good: u32,
    #[serde(default)]
    pub miss: u32,
    #[serde(default)]
    pub great: u32,
    #[serde(default)]
    pub perfect: u32,
    #[serde(default)]
    pub ignore_hit: u32,
    #[serde(default)]
    pub combo_break: u32,
    #[serde(default)]
    pub ignore_miss: u32,
}

impl ManiaStatistics {
    /// Count all hitobjects of the score.
    pub fn total_hits(&self) -> u32 {
        self.ok + self.meh + self.good + self.miss + self.great + self.perfect
    }

    /// Calculate the accuracy rounded to two decimal points i.e. `0 <= accuracy <= 100`
    pub fn accuracy(&self) -> f32 {
        let numerator =
            (self.meh * 50 + self.ok * 100 + self.good * 200 + (self.great + self.perfect) * 300)
                as f32;

        let denominator = (self.total_hits() * 300) as f32;

        (10_000.0 * numerator / denominator).round() / 100.0
    }
}

impl<'de> Visitor<'de> for ModeAsSeed<ScoreStatistics> {
    type Value = ScoreStatistics;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a statistics object")
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        let mut d = MapAccessDeserializer::new(map);

        let stats = match self.mode {
            GameMode::Osu => ScoreStatistics::Osu(Deserialize::deserialize(d)?),
            GameMode::Taiko => ScoreStatistics::Taiko(Deserialize::deserialize(d)?),
            GameMode::Catch => ScoreStatistics::Catch(Deserialize::deserialize(d)?),
            GameMode::Mania => ScoreStatistics::Mania(Deserialize::deserialize(d)?),
        };

        Ok(stats)
    }
}

impl<'de> DeserializeSeed<'de> for ModeAsSeed<ScoreStatistics> {
    type Value = ScoreStatistics;

    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_map(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaximumStatistics {
    Osu(OsuMaximumStatistics),
    Taiko(TaikoMaximumStatistics),
    Catch(CatchMaximumStatistics),
    Mania(ManiaMaximumStatistics),
}

#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OsuMaximumStatistics {
    #[serde(default)]
    pub great: u32,
    #[serde(default)]
    pub miss: u32,
    pub legacy_combo_increase: Option<u32>,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TaikoMaximumStatistics {
    #[serde(default)]
    pub great: u32,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CatchMaximumStatistics {
    #[serde(default)]
    pub great: u32,
    #[serde(default)]
    pub large_tick_hit: u32,
    #[serde(default)]
    pub small_tick_hit: u32,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ManiaMaximumStatistics {
    #[serde(default)]
    pub perfect: u32,
    pub legacy_combo_increase: Option<u32>,
}

impl<'de> Visitor<'de> for ModeAsSeed<MaximumStatistics> {
    type Value = MaximumStatistics;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a maximum statistics object")
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        let mut d = MapAccessDeserializer::new(map);

        let stats = match self.mode {
            GameMode::Osu => MaximumStatistics::Osu(Deserialize::deserialize(d)?),
            GameMode::Taiko => MaximumStatistics::Taiko(Deserialize::deserialize(d)?),
            GameMode::Catch => MaximumStatistics::Catch(Deserialize::deserialize(d)?),
            GameMode::Mania => MaximumStatistics::Mania(Deserialize::deserialize(d)?),
        };

        Ok(stats)
    }
}

impl<'de> DeserializeSeed<'de> for ModeAsSeed<MaximumStatistics> {
    type Value = MaximumStatistics;

    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_map(self)
    }
}

impl ScoreStatistics {
    /// Count all hitobjects of the score i.e. for `GameMode::Osu` the amount 300s, 100s, 50s, and misses.
    ///
    /// Note: Includes tiny droplet (misses) for `GameMode::Catch`.
    pub fn total_hits(&self) -> u32 {
        match self {
            ScoreStatistics::Osu(stats) => stats.total_hits(),
            ScoreStatistics::Taiko(stats) => stats.total_hits(),
            ScoreStatistics::Catch(stats) => stats.total_hits(),
            ScoreStatistics::Mania(stats) => stats.total_hits(),
        }
    }

    /// Calculate the accuracy rounded to two decimal points i.e. `0 <= accuracy <= 100`
    pub fn accuracy(&self) -> f32 {
        match self {
            ScoreStatistics::Osu(stats) => stats.accuracy(),
            ScoreStatistics::Taiko(stats) => stats.accuracy(),
            ScoreStatistics::Catch(stats) => stats.accuracy(),
            ScoreStatistics::Mania(stats) => stats.accuracy(),
        }
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
pub struct UserAttributes {
    pub pin: Option<()>,
}

fn osu_grade(score: &Score, passed_objects: u32) -> Grade {
    todo!()
    // if score.statistics.count_300 == passed_objects {
    //     return if score.mods.contains_any(mods!(HD FL)) {
    //         Grade::XH
    //     } else {
    //         Grade::X
    //     };
    // }

    // let stats = &score.statistics;

    // let ratio300 = stats.count_300 as f32 / passed_objects as f32;
    // let ratio50 = stats.count_50 as f32 / passed_objects as f32;

    // if ratio300 > 0.9 && ratio50 < 0.01 && stats.count_miss == 0 {
    //     if score.mods.contains_any(mods!(HD FL)) {
    //         Grade::SH
    //     } else {
    //         Grade::S
    //     }
    // } else if ratio300 > 0.9 || (ratio300 > 0.8 && stats.count_miss == 0) {
    //     Grade::A
    // } else if ratio300 > 0.8 || (ratio300 > 0.7 && stats.count_miss == 0) {
    //     Grade::B
    // } else if ratio300 > 0.6 {
    //     Grade::C
    // } else {
    //     Grade::D
    // }
}

fn mania_grade(score: &Score, passed_objects: u32, accuracy: Option<f32>) -> Grade {
    todo!()
    // if score.statistics.count_geki == passed_objects {
    //     return if score.mods.contains_any(mods!(HD FL FI)) {
    //         Grade::XH
    //     } else {
    //         Grade::X
    //     };
    // }

    // let accuracy = accuracy.unwrap_or_else(|| score.accuracy());

    // if accuracy > 95.0 {
    //     if score.mods.contains_any(mods!(HD FL FI)) {
    //         Grade::SH
    //     } else {
    //         Grade::S
    //     }
    // } else if accuracy > 90.0 {
    //     Grade::A
    // } else if accuracy > 80.0 {
    //     Grade::B
    // } else if accuracy > 70.0 {
    //     Grade::C
    // } else {
    //     Grade::D
    // }
}

fn taiko_grade(score: &Score, passed_objects: u32) -> Grade {
    todo!()
    // if score.statistics.count_300 == passed_objects {
    //     return if score.mods.contains_any(mods!(HD FL)) {
    //         Grade::XH
    //     } else {
    //         Grade::X
    //     };
    // }

    // let stats = &score.statistics;
    // let ratio300 = stats.count_300 as f32 / passed_objects as f32;

    // if ratio300 > 0.9 && stats.count_miss == 0 {
    //     if score.mods.contains_any(mods!(HD FL)) {
    //         Grade::SH
    //     } else {
    //         Grade::S
    //     }
    // } else if ratio300 > 0.9 || (ratio300 > 0.8 && stats.count_miss == 0) {
    //     Grade::A
    // } else if ratio300 > 0.8 || (ratio300 > 0.7 && stats.count_miss == 0) {
    //     Grade::B
    // } else if ratio300 > 0.6 {
    //     Grade::C
    // } else {
    //     Grade::D
    // }
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
