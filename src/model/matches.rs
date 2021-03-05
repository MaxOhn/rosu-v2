use super::{
    deflate_acc, inflate_acc, BeatmapCompact, GameMode, GameMods, ScoreStatistics, UserCompact,
};

use chrono::{DateTime, Utc};
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Unexpected, Visitor},
    ser::{SerializeMap, Serializer},
    Deserialize, Serialize,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq)]
pub enum MatchEvent {
    Create {
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    Disbanded {
        event_id: u64,
        timestamp: DateTime<Utc>,
    },
    Game {
        event_id: u64,
        game: MatchGame,
        timestamp: DateTime<Utc>,
    },
    HostChanged {
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    Joined {
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    Left {
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
            r#"0, 1, 2, 3, 4, 5, "match-created", "player-joined", "player-left", "match-disbanded", "host-changed", or "other""#
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

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        let event = match v {
            0 => MatchEventType::Create,
            1 => MatchEventType::Disbanded,
            2 => MatchEventType::Game,
            3 => MatchEventType::HostChanged,
            4 => MatchEventType::Joined,
            5 => MatchEventType::Left,
            _ => {
                return Err(E::invalid_value(
                    Unexpected::Unsigned(v),
                    &"0, 1, 2, 3, 4, or 5",
                ))
            }
        };

        Ok(event)
    }
}

impl<'de> Deserialize<'de> for MatchEventType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(MatchEventTypeVisitor)
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
        write!(f, "{}", stringify!($($n),*))
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
                "user_id" => {
                    user_id = map.next_value()?;
                }
                "detail" => {
                    let detail: Detail = map.next_value()?;
                    kind.replace(detail.kind);
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
            MatchEventType::Joined => {
                let user_id = user_id.ok_or_else(|| Error::missing_field("user_id"))?;

                MatchEvent::Joined {
                    event_id,
                    timestamp,
                    user_id,
                }
            }
            MatchEventType::Left => {
                let user_id = user_id.ok_or_else(|| Error::missing_field("user_id"))?;

                MatchEvent::Left {
                    event_id,
                    timestamp,
                    user_id,
                }
            }
            MatchEventType::Game => {
                let game = game.ok_or_else(|| Error::missing_field("game"))?;

                MatchEvent::Game {
                    event_id,
                    game,
                    timestamp,
                }
            }
            MatchEventType::HostChanged => {
                let user_id = user_id.ok_or_else(|| Error::missing_field("user_id"))?;

                MatchEvent::HostChanged {
                    event_id,
                    timestamp,
                    user_id,
                }
            }
            MatchEventType::Create => {
                let user_id = user_id.ok_or_else(|| Error::missing_field("user_id"))?;

                MatchEvent::Create {
                    event_id,
                    timestamp,
                    user_id,
                }
            }
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

impl Serialize for MatchEvent {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(Some(3))?;

        map.serialize_entry("id", &self.event_id())?;
        map.serialize_entry("timestamp", &self.timestamp())?;

        match self {
            Self::Create { user_id, .. } => {
                map.serialize_entry("type", &0)?;
                map.serialize_entry("user_id", user_id)?;
            }
            Self::Disbanded { .. } => {
                map.serialize_entry("type", &1)?;
            }
            Self::Game { game, .. } => {
                map.serialize_entry("type", &2)?;
                map.serialize_entry("game", game)?;
            }
            Self::HostChanged { user_id, .. } => {
                map.serialize_entry("type", &3)?;
                map.serialize_entry("user_id", user_id)?;
            }
            Self::Joined { user_id, .. } => {
                map.serialize_entry("type", &4)?;
                map.serialize_entry("user_id", user_id)?;
            }
            Self::Left { user_id, .. } => {
                map.serialize_entry("type", &5)?;
                map.serialize_entry("user_id", user_id)?;
            }
        }

        map.end()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MatchGame {
    #[serde(rename = "id")]
    game_id: u64,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    mode: GameMode,
    scoring_type: ScoringType,
    team_type: TeamType,
    mods: GameMods,
    #[serde(rename = "beatmap")]
    map: BeatmapCompact,
    scores: Vec<MatchScore>,
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

    // TODO: Use cursor
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct MatchListCursor {
    pub(crate) match_id: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MatchListParams {
    limit: u32,
    sort: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MatchScore {
    user_id: u32,
    #[serde(deserialize_with = "inflate_acc", serialize_with = "deflate_acc")]
    accuracy: f32,
    mods: GameMods,
    score: u32,
    max_combo: u32,
    #[serde(deserialize_with = "to_bool")]
    perfect: bool,
    statistics: ScoreStatistics,
    #[serde(rename = "match")]
    match_meta: MatchScoreInfo,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MatchScoreInfo {
    slot: u32,
    team: Team,
    #[serde(deserialize_with = "to_bool")]
    pass: bool,
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

struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a bool, a stringified bool, or 0 or 1 in either number, string or char format",
        )
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::invalid_value(Unexpected::Unsigned(v), &"0 or 1")),
        }
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        if let Ok(b) = v.parse() {
            return Ok(b);
        }

        v.parse()
            .map(|n| match n {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(Error::invalid_value(
                    Unexpected::Unsigned(n as u64),
                    &"0 or 1",
                )),
            })
            .map_err(|_| {
                Error::invalid_value(Unexpected::Str(v), &r#""true", "false", "0", or "1""#)
            })?
    }

    fn visit_char<E: Error>(self, v: char) -> Result<Self::Value, E> {
        match v {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(Error::invalid_value(Unexpected::Char(v), &"'0' or '1'")),
        }
    }

    #[inline]
    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(v)
    }
}

pub(crate) fn to_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    Ok(d.deserialize_any(BoolVisitor)?)
}
