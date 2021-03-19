use super::{
    beatmap::BeatmapCompact, deflate_acc, score::ScoreStatistics, user::UserCompact, GameMode,
    GameMods,
};
use crate::{Osu, OsuResult};

use chrono::{DateTime, Utc};
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Unexpected, Visitor},
    Deserialize, Serialize,
};
use std::{fmt, slice::Iter, vec::Drain};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum MatchEvent {
    /// The match was created
    #[serde(rename(serialize = "match-created"))]
    Create {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    /// The match was closed
    #[serde(rename(serialize = "match-disbanded"))]
    Disbanded {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
    },
    /// A map is / was being played
    #[serde(rename(serialize = "other"))]
    Game {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        /// Boxed to optimize [`MatchEvent`](crate::model::matches::MatchEvent)'s
        /// size in memory.
        game: Box<MatchGame>,
        timestamp: DateTime<Utc>,
    },
    /// The match host changed
    #[serde(rename(serialize = "host-changed"))]
    HostChanged {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    /// A player joined the match
    #[serde(rename(serialize = "player-joined"))]
    Joined {
        #[serde(rename(serialize = "id"))]
        event_id: u64,
        timestamp: DateTime<Utc>,
        user_id: u32,
    },
    /// A player left the match
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
                return Err(E::unknown_variant(
                    s,
                    &[
                        "match-created",
                        "player-joined",
                        "player-left",
                        "match-disbanded",
                        "host-changed",
                        "other",
                    ],
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
                "user_id" => user_id = map.next_value()?,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    pub mode: GameMode,
    pub scoring_type: ScoringType,
    pub team_type: TeamType,
    pub mods: GameMods,
    /// Optional in case the map was deleted
    #[serde(rename = "beatmap")]
    pub map: Option<BeatmapCompact>,
    pub scores: Vec<MatchScore>,
}

macro_rules! mvp_fold {
    ($self:ident => $field:ident) => {
        $self
            .scores
            .iter()
            .fold(None, |mvp, next| match mvp {
                Some(($field, _)) if $field < next.$field => Some((next.$field, next.user_id)),
                None => Some((next.$field, next.user_id)),
                Some(_) => mvp,
            })
            .map(|(_, user_id)| user_id)
    };
}

impl MatchGame {
    /// Get the user id of the user that performed the best this game.
    pub fn mvp_user_id(&self) -> Option<u32> {
        match self.scoring_type {
            ScoringType::Score | ScoringType::ScoreV2 => mvp_fold!(self => score),
            ScoringType::Accuracy => mvp_fold!(self => accuracy),
            // ! BUG: Winning condition is the combo at the end, not max combo
            ScoringType::Combo => mvp_fold!(self => max_combo),
        }
    }
}

/// Iterates over `&MatchGame`s.
#[derive(Clone, Debug)]
pub struct MatchGameIter<'m> {
    iter: Iter<'m, MatchEvent>,
}

impl<'m> MatchGameIter<'m> {
    #[inline]
    fn new(iter: Iter<'m, MatchEvent>) -> Self {
        Self { iter }
    }
}

impl<'m> Iterator for MatchGameIter<'m> {
    type Item = &'m MatchGame;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let MatchEvent::Game { game, .. } = self.iter.next()? {
                return Some(game);
            }
        }
    }
}

/// Iterates over `MatchGame`s by draining the events of a match.
#[derive(Debug)]
pub struct MatchGameDrain<'m> {
    drain: Drain<'m, MatchEvent>,
}

impl<'m> MatchGameDrain<'m> {
    #[inline]
    fn new(drain: Drain<'m, MatchEvent>) -> Self {
        Self { drain }
    }
}

impl<'m> Iterator for MatchGameDrain<'m> {
    type Item = MatchGame;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let MatchEvent::Game { game, .. } = self.drain.next()? {
                return Some(*game);
            }
        }
    }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MatchListParams {
    pub limit: u32,
    pub sort: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MatchScore {
    #[serde(serialize_with = "deflate_acc")]
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
        write!(f, "a MatchScore struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut accuracy = None;
        let mut max_combo = None;
        let mut mods = None;
        let mut pass = None;
        let mut perfect = None;
        let mut score = None;
        let mut slot = None;
        let mut statistics = None;
        let mut team = None;
        let mut user_id = None;

        while let Some(key) = map.next_key()? {
            match key {
                "accuracy" => {
                    accuracy.replace(map.next_value::<f32>()? * 100.0);
                }
                "match" => {
                    let info: MatchScoreInfo = map.next_value()?;

                    pass.replace(info.pass);
                    slot.replace(info.slot);
                    team.replace(info.team);
                }
                "max_combo" => {
                    max_combo.replace(map.next_value()?);
                }
                "mods" => {
                    mods.replace(map.next_value()?);
                }
                "pass" => {
                    pass.replace(map.next_value()?);
                }
                "perfect" => {
                    perfect.replace(map.next_value::<Bool>()?.0);
                }
                "score" => {
                    score.replace(map.next_value()?);
                }
                "slot" => {
                    slot.replace(map.next_value()?);
                }
                "statistics" => {
                    statistics.replace(map.next_value()?);
                }
                "team" => {
                    team.replace(map.next_value()?);
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
        let max_combo = max_combo.ok_or_else(|| Error::missing_field("max_combo"))?;
        let mods = mods.ok_or_else(|| Error::missing_field("mods"))?;
        let pass = pass.ok_or_else(|| Error::missing_field("match or pass"))?;
        let perfect = perfect.ok_or_else(|| Error::missing_field("perfect"))?;
        let score = score.ok_or_else(|| Error::missing_field("score"))?;
        let slot = slot.ok_or_else(|| Error::missing_field("match or slot"))?;
        let statistics = statistics.ok_or_else(|| Error::missing_field("statistics"))?;
        let team = team.ok_or_else(|| Error::missing_field("match or team"))?;
        let user_id = user_id.ok_or_else(|| Error::missing_field("user_id"))?;

        Ok(MatchScore {
            accuracy,
            max_combo,
            mods,
            pass,
            perfect,
            score,
            slot,
            statistics,
            team,
            user_id,
        })
    }
}

impl<'de> Deserialize<'de> for MatchScore {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(MatchScoreVisitor)
    }
}

#[derive(Debug, Deserialize)]
struct MatchScoreInfo {
    slot: u8,
    team: Team,
    #[serde(deserialize_with = "to_bool")]
    pass: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct OsuMatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_game_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    pub events: Vec<MatchEvent>,
    pub first_event_id: u64,
    pub latest_event_id: u64,
    pub match_id: u32,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub users: Vec<UserCompact>,
}

impl OsuMatch {
    /// Iterate over references of the match's [`MatchGame`]s.
    #[inline]
    pub fn games(&self) -> MatchGameIter {
        MatchGameIter::new(self.events.iter())
    }

    /// Return a vec containing all [`MatchGame`]s of the match.
    ///
    /// ## Note
    ///
    /// The games are drained from the match's events meaning the
    /// `events` field will be empty after this method is called.
    ///
    /// # Example
    ///
    /// ```
    /// use rosu_v2::model::matches::{OsuMatch, MatchEvent, MatchGame};
    /// # use rosu_v2::prelude::{BeatmapCompact, GameMode, GameMods, RankStatus, ScoringType, TeamType};
    /// # use chrono::{DateTime, Utc};
    /// #
    /// # let date = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")
    /// #     .unwrap()
    /// #     .with_timezone(&Utc);
    /// #
    /// # let map = BeatmapCompact {
    /// #     checksum: None,
    /// #     fail_times: None,
    /// #     map_id: 0,
    /// #     mapset: None,
    /// #     max_combo: None,
    /// #     mode: GameMode::STD,
    /// #     seconds_total: 0,
    /// #     stars: 0.0,
    /// #     status: RankStatus::Ranked,
    /// #     version: String::new(),
    /// # };
    ///
    /// let mut osu_match = OsuMatch {
    ///     events: vec![
    ///         # /*
    ///         MatchEvent::Create { ... },
    ///         # */
    ///         # MatchEvent::Create {
    ///         #     event_id: 0,
    ///         #     timestamp: date,
    ///         #     user_id: 0,
    ///         # },
    ///         MatchEvent::Game {
    ///             # /*
    ///             game: Box::new(MatchGame { game_id: 14, ... }),
    ///             # */
    ///             # game: Box::new(MatchGame {
    ///                 # game_id: 14,
    ///                 # start_time:date,
    ///                 # end_time: None,
    ///                 # mode: GameMode::STD,
    ///                 # scoring_type: ScoringType::Score,
    ///                 # team_type: TeamType::HeadToHead,
    ///                 # mods: GameMods::NoMod,
    ///                 # map: map.clone(),
    ///                 # scores: vec![],
    ///             # }),
    ///             # /*
    ///             ...
    ///             # */
    ///             # event_id: 0,
    ///             # timestamp: date,
    ///         },
    ///         # /*
    ///         MatchEvent::Joined { ... },
    ///         # */
    ///         # MatchEvent::Joined {
    ///             # event_id: 0,
    ///             # timestamp: date,
    ///             # user_id: 0,
    ///         # },
    ///         MatchEvent::Game {
    ///             # /*
    ///             game: Box::new(MatchGame { game_id: 52, ... }),
    ///             # */
    ///             # game: Box::new(MatchGame {
    ///                 # game_id: 52,
    ///                 # start_time: date,
    ///                 # end_time: None,
    ///                 # mode: GameMode::STD,
    ///                 # scoring_type: ScoringType::Score,
    ///                 # team_type: TeamType::HeadToHead,
    ///                 # mods: GameMods::NoMod,
    ///                 # map,
    ///                 # scores: vec![],
    ///             # }),
    ///             # /*
    ///             ...
    ///             # */
    ///             # event_id: 0,
    ///             # timestamp: date,
    ///         },
    ///     ],
    ///     # /*
    ///     ...
    ///     # */
    ///     # current_game_id: None,
    ///     # end_time: None,
    ///     # first_event_id: 0,
    ///     # latest_event_id: 0,
    ///     # match_id: 0,
    ///     # name: String::new(),
    ///     # start_time: date,
    ///     # users: vec![],
    /// };
    ///
    /// assert_eq!(osu_match.events.len(), 4);
    ///
    /// {
    ///     // Borrows osu_match mutably, this smaller scope lifts that borrow
    ///     let mut iter = osu_match.drain_games();
    ///
    ///     assert!(matches!(iter.next(), Some(MatchGame { game_id: 14, .. })));
    ///     assert!(matches!(iter.next(), Some(MatchGame { game_id: 52, .. })));
    ///     assert!(matches!(iter.next(), None));
    /// }
    ///
    /// // Events were drained, hence empty
    /// assert!(osu_match.events.is_empty());
    /// ```
    #[inline]
    pub fn drain_games(&mut self) -> MatchGameDrain {
        MatchGameDrain::new(self.events.drain(..))
    }

    /// Get the [`OsuMatch`] containing only data __after__ the currently last event.
    ///
    /// If the currently last event is an in-progress game, then it will be included
    /// in the next [`OsuMatch`]. In all other cases, only new events will be contained.
    ///
    /// Convenient to display a "live" update of the match, e.g. the way an mp link
    /// pulls the next result every 10 seconds.
    pub async fn get_next(&self, osu: &Osu) -> OsuResult<OsuMatch> {
        let sub = matches!(self.events.last(), Some(MatchEvent::Game { game, .. }) if game.end_time.is_none());
        let last_id = self.latest_event_id - sub as u64;

        osu.osu_match(self.match_id).after(last_id).limit(100).await
    }
}

struct OsuMatchVisitor;

impl<'de> Visitor<'de> for OsuMatchVisitor {
    type Value = OsuMatch;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an OsuMatch struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut current_game_id = None;
        let mut end_time = None;
        let mut events = None;
        let mut first_event_id = None;
        let mut latest_event_id = None;
        let mut match_id = None;
        let mut name = None;
        let mut start_time = None;
        let mut users = None;

        while let Some(key) = map.next_key()? {
            match key {
                "current_game_id" => current_game_id = map.next_value()?,
                "end_time" => end_time = map.next_value()?,
                "events" => {
                    events.replace(map.next_value()?);
                }
                "first_event_id" => {
                    first_event_id.replace(map.next_value()?);
                }
                "latest_event_id" => {
                    latest_event_id.replace(map.next_value()?);
                }
                "match" => {
                    let info: MatchInfo = map.next_value()?;

                    end_time = info.end_time;
                    match_id.replace(info.match_id);
                    name.replace(info.name);
                    start_time.replace(info.start_time);
                }
                "match_id" => {
                    match_id.replace(map.next_value()?);
                }
                "name" => {
                    name.replace(map.next_value()?);
                }
                "start_time" => {
                    start_time.replace(map.next_value()?);
                }
                "users" => {
                    users.replace(map.next_value()?);
                }
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        let events = events.ok_or_else(|| Error::missing_field("events"))?;
        let first_event_id =
            first_event_id.ok_or_else(|| Error::missing_field("first_event_id"))?;
        let latest_event_id =
            latest_event_id.ok_or_else(|| Error::missing_field("latest_event_id"))?;
        let match_id = match_id.ok_or_else(|| Error::missing_field("match or match_id"))?;
        let name = name.ok_or_else(|| Error::missing_field("match or name"))?;
        let start_time = start_time.ok_or_else(|| Error::missing_field("match or start_time"))?;
        let users = users.ok_or_else(|| Error::missing_field("users"))?;

        Ok(OsuMatch {
            current_game_id,
            events,
            first_event_id,
            latest_event_id,
            users,
            end_time,
            match_id,
            name,
            start_time,
        })
    }
}

impl<'de> Deserialize<'de> for OsuMatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(OsuMatchVisitor)
    }
}

impl PartialEq for OsuMatch {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.match_id == other.match_id && self.latest_event_id == other.latest_event_id
    }
}

impl Eq for OsuMatch {}

// TODO: Test all values
def_enum!(u8 ScoringType {
    Score = 0 ("score"),
    Accuracy = 1 ("accuracy"),
    Combo = 2 ("combo"),
    ScoreV2 = 3 ("scorev2"),
});

def_enum!(u8 Team {
    None = 0 ("none"),
    Blue = 1 ("blue"),
    Red = 2 ("red"),
});

def_enum!(u8 TeamType {
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
