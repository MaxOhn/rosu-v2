use super::{BeatmapCompact, GameMode, GameMods, ScoreStatistics, UserCompact};
use crate::{Osu, OsuResult};

use chrono::{DateTime, Utc};
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Unexpected, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum MatchEvent {
    #[serde(rename(serialize = "match-created"))]
    Create {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    #[serde(rename(serialize = "match-disbanded"))]
    Disbanded {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
    },
    #[serde(rename(serialize = "other"))]
    Game {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        game: MatchGame,
        timestamp: DateTime<Utc>,
    },
    #[serde(rename(serialize = "host-changed"))]
    HostChanged {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    #[serde(rename(serialize = "player-joined"))]
    Joined {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    #[serde(rename(serialize = "player-left"))]
    Left {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
}

impl MatchEvent {
    pub fn event_id(&self) -> u64 {
        match self {
            Self::Create { event_id, .. } => *event_id,
            Self::Disbanded { event_id, .. } => *event_id,
            Self::Game { event_id, .. } => *event_id,
            Self::HostChanged { event_id, .. } => *event_id,
            Self::Joined { event_id, .. } => *event_id,
            Self::Left { event_id, .. } => *event_id,
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Self::Create { timestamp, .. } => *timestamp,
            Self::Disbanded { timestamp, .. } => *timestamp,
            Self::Game { timestamp, .. } => *timestamp,
            Self::HostChanged { timestamp, .. } => *timestamp,
            Self::Joined { timestamp, .. } => *timestamp,
            Self::Left { timestamp, .. } => *timestamp,
        }
    }

    pub fn user_id(&self) -> Option<u32> {
        match self {
            Self::Create { user_id, .. } => Some(*user_id),
            Self::Disbanded { .. } => None,
            Self::Game { .. } => None,
            Self::HostChanged { user_id, .. } => Some(*user_id),
            Self::Joined { user_id, .. } => Some(*user_id),
            Self::Left { user_id, .. } => Some(*user_id),
        }
    }
}

enum MatchEventType {
    Create,
    Disbanded,
    Game,
    HostChanged,
    Joined,
    Left,
}

struct MatchEventTypeVisitor;

impl<'de> Visitor<'de> for MatchEventTypeVisitor {
    type Value = MatchEventType;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#""match-created", "player-joined", "player-left", "match-disbanded", "host-changed", or "other""#
        )
    }

    fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
        let event = match s {
            "player-joined" => MatchEventType::Joined,
            "player-left" => MatchEventType::Left,
            "other" => MatchEventType::Game,
            "host-changed" => MatchEventType::HostChanged,
            "match-created" => MatchEventType::Create,
            "match-disbanded" => MatchEventType::Disbanded,
            _ => {
                return Err(E::invalid_value(
                    Unexpected::Str(s),
                    &"match-created, player-joined, player-left, match-disbanded, host-changed, or other",
                ))
            }
        };

        Ok(event)
    }
}

impl<'de> Deserialize<'de> for MatchEventType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_str(MatchEventTypeVisitor)
    }
}

#[derive(Deserialize)]
struct Detail {
    #[serde(rename = "type")]
    kind: MatchEventType,
}

struct MatchEventVisitor;

impl<'de> Visitor<'de> for MatchEventVisitor {
    type Value = MatchEvent;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MatchEvent enum")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut id = None;
        let mut timestamp = None;
        let mut user_id = None;
        let mut kind = None;
        let mut game = None;

        while let Some(key) = map.next_key()? {
            match key {
                "id" => {
                    id.replace(map.next_value()?);
                }
                "timestamp" => {
                    timestamp.replace(map.next_value()?);
                }
                "detail" => {
                    let detail: Detail = map.next_value()?;
                    kind.replace(detail.kind);
                }
                "user_id" => {
                    user_id = map.next_value()?;
                }
                "type" => {
                    kind.replace(map.next_value()?);
                }
                "game" => {
                    game.replace(map.next_value()?);
                }
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        let event_id = id.ok_or_else(|| Error::missing_field("id"))?;
        let timestamp = timestamp.ok_or_else(|| Error::missing_field("timestamp"))?;
        let kind = kind.ok_or_else(|| Error::missing_field("detail or type"))?;

        let event = match kind {
            MatchEventType::Joined => MatchEvent::Joined {
                event_id,
                timestamp,
                user_id: user_id.ok_or_else(|| Error::missing_field("user_id"))?,
            },
            MatchEventType::Left => MatchEvent::Left {
                event_id,
                timestamp,
                user_id: user_id.ok_or_else(|| Error::missing_field("user_id"))?,
            },
            MatchEventType::Game => MatchEvent::Game {
                event_id,
                game: game.ok_or_else(|| Error::missing_field("game"))?,
                timestamp,
            },
            MatchEventType::HostChanged => MatchEvent::HostChanged {
                event_id,
                timestamp,
                user_id: user_id.ok_or_else(|| Error::missing_field("user_id"))?,
            },
            MatchEventType::Create => MatchEvent::Create {
                event_id,
                timestamp,
                user_id: user_id.ok_or_else(|| Error::missing_field("user_id"))?,
            },
            MatchEventType::Disbanded => MatchEvent::Disbanded {
                event_id,
                timestamp,
            },
        };

        Ok(event)
    }
}

impl<'de> Deserialize<'de> for MatchEvent {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(MatchEventVisitor)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MatchGame {
    #[serde(rename = "id")]
    pub game_id: u64,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub mode: GameMode,
    pub scoring_type: ScoringType,
    pub team_type: TeamType,
    pub mods: GameMods,
    #[serde(rename = "beatmap")]
    pub map: BeatmapCompact,
    pub scores: Vec<MatchScore>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MatchInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(rename = "id")]
    pub match_id: u32,
    pub name: String,
    pub start_time: DateTime<Utc>,
}

impl PartialEq for MatchInfo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.match_id == other.match_id && self.end_time == other.end_time
    }
}

impl Eq for MatchInfo {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MatchList {
    pub(crate) cursor: Option<MatchListCursor>,
    pub matches: Vec<MatchInfo>,
    pub params: MatchListParams,
}

impl MatchList {
    #[inline]
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }

    /// If `has_more()` is true, the API can provide the next set of matches and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<MatchList>> {
        Some(osu.osu_matches().cursor(self.cursor?).await)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct MatchListCursor {
    pub(crate) match_id: u32,
}

// TODO
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MatchListParams {
    limit: u32,
    sort: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchScore {
    pub accuracy: f32,
    pub max_combo: u32,
    pub mods: GameMods,
    pub pass: bool,
    pub perfect: bool,
    pub score: u32,
    pub slot: u8,
    pub statistics: ScoreStatistics,
    pub team: Team,
    pub user_id: u32,
}

struct MatchScoreVisitor;

impl<'de> Visitor<'de> for MatchScoreVisitor {
    type Value = MatchScore;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MatchScore struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut user_id = None;
        let mut accuracy = None;
        let mut mods = None;
        let mut score = None;
        let mut max_combo = None;
        let mut perfect = None;
        let mut statistics = None;
        let mut info = None;

        while let Some(key) = map.next_key()? {
            match key {
                "accuracy" => {
                    accuracy.replace(map.next_value::<f32>()? * 100.0);
                }
                "match" => {
                    info.replace(map.next_value()?);
                }
                "max_combo" => {
                    max_combo.replace(map.next_value()?);
                }
                "mods" => {
                    mods.replace(map.next_value()?);
                }
                "perfect" => {
                    perfect.replace(map.next_value::<Bool>()?.0);
                }
                "score" => {
                    score.replace(map.next_value()?);
                }
                "statistics" => {
                    statistics.replace(map.next_value()?);
                }
                "user_id" => {
                    user_id.replace(map.next_value()?);
                }
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        let accuracy = accuracy.ok_or_else(|| Error::missing_field("accuracy"))?;
        let info: MatchScoreInfo = info.ok_or_else(|| Error::missing_field("match"))?;
        let max_combo = max_combo.ok_or_else(|| Error::missing_field("max_combo"))?;
        let mods = mods.ok_or_else(|| Error::missing_field("mods"))?;
        let perfect = perfect.ok_or_else(|| Error::missing_field("perfect"))?;
        let score = score.ok_or_else(|| Error::missing_field("score"))?;
        let statistics = statistics.ok_or_else(|| Error::missing_field("statistics"))?;
        let user_id = user_id.ok_or_else(|| Error::missing_field("user_id"))?;

        Ok(MatchScore {
            accuracy,
            max_combo,
            mods,
            pass: info.pass,
            perfect,
            score,
            slot: info.slot,
            statistics,
            team: info.team,
            user_id,
        })
    }
}

impl<'de> Deserialize<'de> for MatchScore {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_struct(
            "MatchScore",
            &[
                "accuracy",
                "match",
                "max_combo",
                "mods",
                "perfect",
                "score",
                "statistics",
                "user_id",
            ],
            MatchScoreVisitor,
        )
    }
}

impl Serialize for MatchScore {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut score = s.serialize_struct("MatchScore", 8)?;

        score.serialize_field("accuracy", &(self.accuracy / 100.0))?;
        score.serialize_field::<MatchScoreInfo>(
            "match",
            &MatchScoreInfo {
                slot: self.slot,
                team: self.team,
                pass: self.pass,
            },
        )?;
        score.serialize_field("max_combo", &self.max_combo)?;
        score.serialize_field("mods", &self.mods)?;
        score.serialize_field("perfect", &self.perfect)?;
        score.serialize_field("score", &self.score)?;
        score.serialize_field("statistics", &self.statistics)?;
        score.serialize_field("user_id", &self.user_id)?;

        score.end()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct MatchScoreInfo {
    pub slot: u8,
    pub team: Team,
    #[serde(deserialize_with = "to_bool")]
    pub pass: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OsuMatch {
    pub current_game_id: Option<()>,
    pub events: Vec<MatchEvent>,
    pub first_event_id: u64,
    #[serde(rename = "match")]
    pub info: MatchInfo,
    pub latest_event_id: u64,
    pub users: Vec<UserCompact>,
}

impl PartialEq for OsuMatch {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info && self.latest_event_id == other.latest_event_id
    }
}

impl Eq for OsuMatch {}

macro_rules! def_enum {
    ($type:tt { $($variant:ident = $n:literal ($alt:literal),)* }) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum $type {
            $($variant = $n,)*
        }

        impl<'de> Visitor<'de> for MyVisitor<$type> {
            type Value = $type;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", concat!($(stringify!($n), ",\"", $alt, "\""),*))
            }

            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                match v {
                    $($n => Ok(<$type>::$variant),)*
                    _ => {
                        Err(Error::invalid_value(Unexpected::Unsigned(v), &stringify!($($n),*)))
                    },
                }
            }

            fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
                match s {
                    $($alt => Ok(<$type>::$variant),)*
                    _ => {
                        Err(Error::invalid_value(Unexpected::Str(s), &stringify!($($alt),*)))
                    },
                }
            }
        }

        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                d.deserialize_any(MyVisitor::<$type>::new())
            }
        }

        impl Serialize for $type {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_u8(*self as u8)
            }
        }
    }
}

struct MyVisitor<T> {
    phantom: PhantomData<T>,
}

impl<T> MyVisitor<T> {
    fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

// TODO: Test all values
def_enum!(ScoringType {
    Score = 0 ("score"),
    Accuracy = 1 ("accuracy"),
    Combo = 2 ("combo"),
    ScoreV2 = 3 ("scorev2"),
});

def_enum!(Team {
    None = 0 ("none"),
    Blue = 1 ("blue"),
    Red = 2 ("red"),
});

def_enum!(TeamType {
    HeadToHead = 0 ("head-to-head"),
    TagCoop = 1 ("tagcoop"),
    TeamVS = 2 ("team-vs"),
    TagTeamVS = 3 ("tagteamvs"),
});

struct Bool(bool);
struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = Bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a bool, a stringified bool, or 0 or 1 in either number, string or char format",
        )
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(Bool(false)),
            1 => Ok(Bool(true)),
            _ => Err(Error::invalid_value(Unexpected::Unsigned(v), &"0 or 1")),
        }
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        if let Ok(b) = v.parse() {
            return Ok(Bool(b));
        }

        v.parse::<u64>()
            .map(|n| match n {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(Error::invalid_value(Unexpected::Unsigned(n), &"0 or 1")),
            })
            .map_err(|_| {
                Error::invalid_value(Unexpected::Str(v), &r#""true", "false", "0", or "1""#)
            })?
            .map(Bool)
    }

    fn visit_char<E: Error>(self, v: char) -> Result<Self::Value, E> {
        match v {
            '0' => Ok(Bool(false)),
            '1' => Ok(Bool(true)),
            _ => Err(Error::invalid_value(Unexpected::Char(v), &"'0' or '1'")),
        }
    }

    #[inline]
    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Bool(v))
    }
}

pub(crate) fn to_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    Ok(d.deserialize_any(BoolVisitor)?.0)
}

impl<'de> Deserialize<'de> for Bool {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(BoolVisitor)
    }
}
