use super::{
    beatmap::Beatmap, mods::GameMods, score_::ScoreStatistics, serde_, user_::User,
    Cursor, GameMode,
};
use crate::{
    prelude::{GameModsIntermode, ModeAsSeed},
    Osu, OsuResult,
};

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use serde::{
    de::{
        DeserializeSeed, Deserializer, Error as DeError, Error, IgnoredAny, MapAccess, SeqAccess,
        Unexpected, Visitor,
    },
    Deserialize,
};
use serde_json::value::RawValue;
use std::{collections::HashMap, fmt, slice::Iter, vec::Drain};
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
#[cfg_attr(feature = "serialize", serde(tag = "type"))]
pub enum MatchEvent {
    /// The match was created
    #[cfg_attr(feature = "serialize", serde(rename(serialize = "match-created")))]
    Create {
        #[cfg_attr(feature = "serialize", serde(rename(serialize = "id")))]
        event_id: u64,
        #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        timestamp: OffsetDateTime,
        user_id: Option<u32>,
    },
    /// The match was closed
    #[cfg_attr(feature = "serialize", serde(rename(serialize = "match-disbanded")))]
    Disbanded {
        #[cfg_attr(feature = "serialize", serde(rename(serialize = "id")))]
        event_id: u64,
        #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        timestamp: OffsetDateTime,
    },
    /// A map is / was being played
    #[cfg_attr(feature = "serialize", serde(rename(serialize = "other")))]
    Game {
        #[cfg_attr(feature = "serialize", serde(rename(serialize = "id")))]
        event_id: u64,
        /// Boxed to optimize [`MatchEvent`](crate::model::matches::MatchEvent)'s
        /// size in memory.
        game: Box<MatchGame>,
        #[cfg_attr(feature = "serialize", serde(default))]
        match_name: String,
        #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        timestamp: OffsetDateTime,
    },
    /// The match host changed
    #[cfg_attr(feature = "serialize", serde(rename(serialize = "host-changed")))]
    HostChanged {
        #[cfg_attr(feature = "serialize", serde(rename(serialize = "id")))]
        event_id: u64,
        #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        timestamp: OffsetDateTime,
        user_id: u32,
    },
    /// A player joined the match
    #[cfg_attr(feature = "serialize", serde(rename(serialize = "player-joined")))]
    Joined {
        #[cfg_attr(feature = "serialize", serde(rename(serialize = "id")))]
        event_id: u64,
        #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        timestamp: OffsetDateTime,
        user_id: u32,
    },
    /// A player was kicked from the match
    #[cfg_attr(feature = "serialize", serde(rename(serialize = "player-kicked")))]
    Kicked {
        #[cfg_attr(feature = "serialize", serde(rename(serialize = "id")))]
        event_id: u64,
        #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        timestamp: OffsetDateTime,
        user_id: u32,
    },
    /// A player left the match
    #[cfg_attr(feature = "serialize", serde(rename(serialize = "player-left")))]
    Left {
        #[cfg_attr(feature = "serialize", serde(rename(serialize = "id")))]
        event_id: u64,
        #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        timestamp: OffsetDateTime,
        user_id: u32,
    },
}

impl MatchEvent {
    /// Return the id of the event
    pub fn event_id(&self) -> u64 {
        match self {
            Self::Create { event_id, .. } => *event_id,
            Self::Disbanded { event_id, .. } => *event_id,
            Self::Game { event_id, .. } => *event_id,
            Self::HostChanged { event_id, .. } => *event_id,
            Self::Joined { event_id, .. } => *event_id,
            Self::Kicked { event_id, .. } => *event_id,
            Self::Left { event_id, .. } => *event_id,
        }
    }

    /// Return the timestamp of the event
    pub fn timestamp(&self) -> OffsetDateTime {
        match self {
            Self::Create { timestamp, .. } => *timestamp,
            Self::Disbanded { timestamp, .. } => *timestamp,
            Self::Game { timestamp, .. } => *timestamp,
            Self::HostChanged { timestamp, .. } => *timestamp,
            Self::Joined { timestamp, .. } => *timestamp,
            Self::Kicked { timestamp, .. } => *timestamp,
            Self::Left { timestamp, .. } => *timestamp,
        }
    }

    /// Return the user id of the user associated with the event
    pub fn user_id(&self) -> Option<u32> {
        match self {
            Self::Create { user_id, .. } => *user_id,
            Self::Disbanded { .. } => None,
            Self::Game { .. } => None,
            Self::HostChanged { user_id, .. } => Some(*user_id),
            Self::Joined { user_id, .. } => Some(*user_id),
            Self::Kicked { user_id, .. } => Some(*user_id),
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
    Kicked,
    Left,
}

struct MatchEventTypeVisitor;

impl<'de> Visitor<'de> for MatchEventTypeVisitor {
    type Value = MatchEventType;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
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
            "player-kicked" => MatchEventType::Kicked,
            _ => {
                return Err(E::unknown_variant(
                    s,
                    &[
                        "match-created",
                        "player-joined",
                        "player-left",
                        "player-kicked",
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
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_str(MatchEventTypeVisitor)
    }
}

#[derive(Deserialize)]
struct Detail {
    #[serde(rename = "type")]
    kind: MatchEventType,
    #[serde(default, rename = "text")]
    match_name: String,
}

struct MatchEventVisitor;

impl<'de> Visitor<'de> for MatchEventVisitor {
    type Value = MatchEvent;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MatchEvent enum")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        #[derive(Deserialize)]
        struct DateTimeWrapper(#[serde(with = "serde_::datetime")] OffsetDateTime);

        let mut id = None;
        let mut timestamp: Option<DateTimeWrapper> = None;
        let mut user_id = None;
        let mut kind = None;
        let mut match_name = None;
        let mut game = None;

        while let Some(key) = map.next_key()? {
            match key {
                "id" => id = Some(map.next_value()?),
                "timestamp" => timestamp = Some(map.next_value()?),
                "detail" => {
                    let detail: Detail = map.next_value()?;
                    kind = Some(detail.kind);

                    if !detail.match_name.is_empty() {
                        match_name.replace(detail.match_name);
                    }
                }
                "user_id" => user_id = map.next_value()?,
                "game" => game = Some(map.next_value()?),
                "type" => kind = Some(map.next_value()?),
                "match_name" => match_name = Some(map.next_value()?),
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        let event_id = id.ok_or_else(|| Error::missing_field("id"))?;
        let DateTimeWrapper(timestamp) =
            timestamp.ok_or_else(|| Error::missing_field("timestamp"))?;
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
                match_name: match_name
                    .ok_or_else(|| Error::missing_field("detail or match_name"))?,
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
                user_id,
            },
            MatchEventType::Disbanded => MatchEvent::Disbanded {
                event_id,
                timestamp,
            },
            MatchEventType::Kicked => MatchEvent::Kicked {
                event_id,
                timestamp,
                user_id: user_id.ok_or_else(|| Error::missing_field("user_id"))?,
            },
        };

        Ok(event)
    }
}

impl<'de> Deserialize<'de> for MatchEvent {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(MatchEventVisitor)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct MatchGame {
    #[cfg_attr(feature = "serialize", serde(rename = "id"))]
    pub game_id: u64,
    #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub start_time: OffsetDateTime,
    #[cfg_attr(
        feature = "serialize",
        serde(
            skip_serializing_if = "Option::is_none",
            with = "serde_::option_datetime"
        )
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub end_time: Option<OffsetDateTime>,
    pub mode: GameMode,
    pub scoring_type: ScoringType,
    pub team_type: TeamType,
    pub mods: GameMods,
    /// [`Beatmap`](crate::model::beatmap::Beatmap) of the game;
    /// `None` if the map was deleted
    #[cfg_attr(feature = "serialize", serde(rename = "beatmap"))]
    pub map: Option<Beatmap>,
    pub scores: Vec<MatchScore>,
}

impl<'de> Deserialize<'de> for MatchGame {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct MatchGameRawMods {
            #[serde(rename = "id")]
            game_id: u64,
            #[serde(with = "serde_::datetime")]
            start_time: OffsetDateTime,
            #[serde(with = "serde_::option_datetime")]
            end_time: Option<OffsetDateTime>,
            mode: GameMode,
            scoring_type: ScoringType,
            team_type: TeamType,
            mods: Box<RawValue>,
            #[serde(rename = "beatmap")]
            map: Option<Beatmap>,
            scores: Vec<MatchScore>,
        }

        let game_raw = <MatchGameRawMods as serde::Deserialize>::deserialize(d)?;
        let mut d = serde_json::Deserializer::from_str(game_raw.mods.get());

        Ok(MatchGame {
            mods: ModeAsSeed::<GameMods>::new(game_raw.mode)
                .deserialize(&mut d)
                .map_err(DeError::custom)?,
            game_id: game_raw.game_id,
            start_time: game_raw.start_time,
            end_time: game_raw.end_time,
            mode: game_raw.mode,
            scoring_type: game_raw.scoring_type,
            team_type: game_raw.team_type,
            map: game_raw.map,
            scores: game_raw.scores,
        })
    }
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
    #[inline]
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
    fn new(iter: Iter<'m, MatchEvent>) -> Self {
        Self { iter }
    }
}

impl<'m> Iterator for MatchGameIter<'m> {
    type Item = &'m MatchGame;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let MatchEvent::Game { game, .. } = self.iter.next()? {
                return Some(game);
            }
        }
    }
}

impl<'m> DoubleEndedIterator for MatchGameIter<'m> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let MatchEvent::Game { game, .. } = self.iter.next_back()? {
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
    fn new(drain: Drain<'m, MatchEvent>) -> Self {
        Self { drain }
    }
}

impl<'m> Iterator for MatchGameDrain<'m> {
    type Item = MatchGame;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let MatchEvent::Game { game, .. } = self.drain.next()? {
                return Some(*game);
            }
        }
    }
}

impl<'m> DoubleEndedIterator for MatchGameDrain<'m> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let MatchEvent::Game { game, .. } = self.drain.next_back()? {
                return Some(*game);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct MatchInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub end_time: Option<OffsetDateTime>,
    #[serde(rename = "id")]
    pub match_id: u32,
    pub name: String,
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub start_time: OffsetDateTime,
}

impl PartialEq for MatchInfo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.match_id == other.match_id && self.end_time == other.end_time
    }
}

impl Eq for MatchInfo {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
// TODO
// #[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct MatchList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cursor: Option<Cursor>,
    pub matches: Vec<MatchInfo>,
    pub params: MatchListParams,
}

impl MatchList {
    /// Returns whether there is a next page of match results,
    /// retrievable via [`get_next`](MatchList::get_next).
    #[inline]
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }

    /// If [`has_more`](MatchList::has_more) is true, the API can provide the next set of matches and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<MatchList>> {
        Some(osu.osu_matches().cursor(self.cursor.clone()?).await)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct MatchListParams {
    pub limit: u32,
    pub sort: String,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct MatchScore {
    /// Accuracy between `0.0` and `100.0`
    #[cfg_attr(feature = "serialize", serde(with = "serde_::adjust_acc"))]
    pub accuracy: f32,
    pub max_combo: u32,
    pub mods: GameModsIntermode,
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

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a MatchScore struct")
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
                "accuracy" => accuracy = Some(map.next_value::<f32>()? * 100.0),
                "match" => {
                    let info: MatchScoreInfo = map.next_value()?;

                    pass = Some(info.pass);
                    slot = Some(info.slot);
                    team = Some(info.team);
                }
                "max_combo" => max_combo = Some(map.next_value()?),
                "mods" => mods = Some(map.next_value()?),
                "pass" => pass = Some(map.next_value()?),
                "perfect" => perfect = Some(map.next_value::<Bool>()?.0),
                "score" => score = Some(map.next_value()?),
                "slot" => slot = Some(map.next_value()?),
                "statistics" => statistics = Some(map.next_value()?),
                "team" => team = Some(map.next_value()?),
                "user_id" => user_id = Some(map.next_value()?),
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
    #[inline]
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

struct MatchUsers(HashMap<u32, User>);

struct MatchUsersVisitor;

impl<'de> Visitor<'de> for MatchUsersVisitor {
    type Value = MatchUsers;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a sequence containing UserCompact")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut users = HashMap::with_capacity(seq.size_hint().unwrap_or_default());

        while let Some(next) = seq.next_element::<User>()? {
            users.insert(next.user_id, next);
        }

        Ok(MatchUsers(users))
    }
}

impl<'de> Deserialize<'de> for MatchUsers {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_seq(MatchUsersVisitor)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct OsuMatch {
    #[cfg_attr(feature = "serialize", serde(skip_serializing_if = "Option::is_none"))]
    pub current_game_id: Option<u64>,
    #[cfg_attr(
        feature = "serialize",
        serde(
            skip_serializing_if = "Option::is_none",
            with = "serde_::option_datetime"
        )
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub end_time: Option<OffsetDateTime>,
    pub events: Vec<MatchEvent>,
    pub first_event_id: u64,
    pub latest_event_id: u64,
    pub match_id: u32,
    pub name: String,
    #[cfg_attr(feature = "serialize", serde(with = "serde_::datetime"))]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub start_time: OffsetDateTime,
    /// Maps user ids to users
    #[cfg_attr(feature = "serialize", serde(serialize_with = "serialize_match_users"))]
    pub users: HashMap<u32, User>,
}

#[cfg(feature = "serialize")]
fn serialize_match_users<S: serde::ser::Serializer>(
    users: &HashMap<u32, User>,
    s: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeSeq;

    let mut seq = s.serialize_seq(Some(users.len()))?;

    for user in users.values() {
        seq.serialize_element(user)?;
    }

    seq.end()
}

impl OsuMatch {
    /// Iterate over references of the match's [`MatchGame`]s.
    #[inline]
    pub fn games(&self) -> MatchGameIter<'_> {
        MatchGameIter::new(self.events.iter())
    }

    /// Return an iterator over all [`MatchGame`]s of the match.
    ///
    /// **Note:** The games are drained from the match's events meaning the
    /// `events` field will be empty after this method is called.
    ///
    /// # Example
    ///
    /// ```
    /// use rosu_v2::model::matches::{OsuMatch, MatchEvent, MatchGame};
    /// # use rosu_v2::prelude::{GameMode, GameMods, RankStatus, ScoringType, TeamType};
    /// #
    /// # let date = time::OffsetDateTime::now_utc();
    ///
    /// let mut osu_match = OsuMatch {
    ///     events: vec![
    ///         # /*
    ///         MatchEvent::Create { ... },
    ///         # */
    ///         # MatchEvent::Create {
    ///         #     event_id: 0,
    ///         #     timestamp: date,
    ///         #     user_id: Some(0),
    ///         # },
    ///         MatchEvent::Game {
    ///             # /*
    ///             game: Box::new(MatchGame { game_id: 14, ... }),
    ///             # */
    ///             # game: Box::new(MatchGame {
    ///                 # game_id: 14,
    ///                 # start_time:date,
    ///                 # end_time: None,
    ///                 # mode: GameMode::Osu,
    ///                 # scoring_type: ScoringType::Score,
    ///                 # team_type: TeamType::HeadToHead,
    ///                 # mods: GameMods::new(),
    ///                 # map: None,
    ///                 # scores: vec![],
    ///             # }),
    ///             # /*
    ///             ...
    ///             # */
    ///             # event_id: 0,
    ///             # match_name: String::new(),
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
    ///                 # mode: GameMode::Osu,
    ///                 # scoring_type: ScoringType::Score,
    ///                 # team_type: TeamType::HeadToHead,
    ///                 # mods: GameMods::new(),
    ///                 # map: None,
    ///                 # scores: vec![],
    ///             # }),
    ///             # /*
    ///             ...
    ///             # */
    ///             # event_id: 0,
    ///             # match_name: String::new(),
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
    ///     # users: std::collections::HashMap::new(),
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
    pub fn drain_games(&mut self) -> MatchGameDrain<'_> {
        MatchGameDrain::new(self.events.drain(..))
    }

    /// Get the [`OsuMatch`] containing only data from some event id onwards.
    ///
    /// If the latest *game* event is an in-progress game, the result will contain
    /// all events starting from this game event, inclusively.
    /// Otherwise it will contain all events after the currently last event.
    ///
    /// Convenient to display a "live" update of the match, e.g. the way an mp link
    /// pulls the next result every 10 seconds.
    pub async fn get_next(&self, osu: &Osu) -> OsuResult<OsuMatch> {
        let mut last_id = self.latest_event_id;

        for event in self.events.iter().rev() {
            if let MatchEvent::Game { event_id, game, .. } = event {
                if game.end_time.is_none() {
                    last_id = event_id - 1;
                }

                break;
            }
        }

        osu.osu_match(self.match_id).after(last_id).limit(100).await
    }

    /// The API sends only up to 100 events per request.
    /// This method checks whether there are events before the currently first event.
    #[inline]
    pub fn has_previous(&self) -> bool {
        self.events
            .first()
            .map_or(false, |event| self.first_event_id != event.event_id())
    }

    /// Get the [`OsuMatch`] containing only data before some event id.
    ///
    /// This method returns `None` either if the `events` field is empty or
    /// if the first event is already contained.
    pub async fn get_previous(&self, osu: &Osu) -> Option<OsuResult<OsuMatch>> {
        let first_id = self
            .events
            .first()
            .map(MatchEvent::event_id)
            .filter(|&first_id| first_id != self.first_event_id)?;

        let previous = osu
            .osu_match(self.match_id)
            .before(first_id)
            .limit(100)
            .await;

        Some(previous)
    }
}

struct OsuMatchVisitor;

impl<'de> Visitor<'de> for OsuMatchVisitor {
    type Value = OsuMatch;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an OsuMatch struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        #[derive(Deserialize)]
        struct DateTimeWrapper(#[serde(with = "serde_::datetime")] OffsetDateTime);

        #[derive(Deserialize)]
        struct OptionDateTimeWrapper(
            #[serde(with = "serde_::option_datetime")] Option<OffsetDateTime>,
        );

        let mut current_game_id = None;
        let mut end_time: Option<OptionDateTimeWrapper> = None;
        let mut events = None;
        let mut first_event_id = None;
        let mut latest_event_id = None;
        let mut match_id = None;
        let mut name = None;
        let mut start_time: Option<DateTimeWrapper> = None;
        let mut users = None;

        while let Some(key) = map.next_key()? {
            match key {
                "current_game_id" => current_game_id = map.next_value()?,
                "end_time" => end_time = map.next_value()?,
                "events" => {
                    let value: Vec<MatchEvent> = map.next_value()?;

                    let name_opt = value.iter().rev().find_map(|event| match event {
                        MatchEvent::Game { match_name, .. } => Some(match_name.to_owned()),
                        _ => None,
                    });

                    if let Some(match_name) = name_opt {
                        name = Some(match_name);
                    }

                    events = Some(value);
                }
                "first_event_id" => first_event_id = Some(map.next_value()?),
                "latest_event_id" => latest_event_id = Some(map.next_value()?),
                "match" => {
                    let info: MatchInfo = map.next_value()?;

                    end_time = Some(OptionDateTimeWrapper(info.end_time));
                    match_id.replace(info.match_id);
                    name.replace(info.name);
                    start_time = Some(DateTimeWrapper(info.start_time));
                }
                "match_id" => match_id = Some(map.next_value()?),
                "name" => name = Some(map.next_value()?),
                "start_time" => start_time = Some(map.next_value()?),
                "users" => {
                    let MatchUsers(user_map) = map.next_value()?;

                    users = Some(user_map);
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
        let DateTimeWrapper(start_time) =
            start_time.ok_or_else(|| Error::missing_field("match or start_time"))?;
        let users = users.ok_or_else(|| Error::missing_field("users"))?;

        Ok(OsuMatch {
            current_game_id,
            events,
            first_event_id,
            latest_event_id,
            users,
            end_time: end_time.and_then(|wrapper| wrapper.0),
            match_id,
            name,
            start_time,
        })
    }
}

impl<'de> Deserialize<'de> for OsuMatch {
    #[inline]
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

def_enum!(ScoringType {
    Score = 0 ("score"),
    Accuracy = 1 ("accuracy"),
    Combo = 2 ("combo"),
    ScoreV2 = 3 ("scorev2"),
});

impl Default for ScoringType {
    #[inline]
    fn default() -> Self {
        Self::Score
    }
}

def_enum!(Team {
    None = 0 ("none"),
    Blue = 1 ("blue"),
    Red = 2 ("red"),
});

impl Default for Team {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

def_enum!(TeamType {
    HeadToHead = 0 ("head-to-head"),
    TagCoop = 1 ("tag-coop"),
    TeamVS = 2 ("team-vs"),
    TagTeamVS = 3 ("tag-team-vs"),
});

impl Default for TeamType {
    #[inline]
    fn default() -> Self {
        Self::HeadToHead
    }
}

struct Bool(bool);
struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = Bool;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a bool, a stringified bool, or 0 or 1 in either number, string or char format")
    }

    #[inline]
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

    #[inline]
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
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(BoolVisitor)
    }
}
