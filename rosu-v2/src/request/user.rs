use crate::{
    model::{
        beatmap::{BeatmapsetExtended, MostPlayedMap, RankStatus},
        event::Event,
        kudosu::KudosuHistory,
        score::Score,
        user::{User, UserExtended, Username},
        user_::Users,
        GameMode,
    },
    request::{
        serialize::{maybe_bool_as_u8, maybe_mode_as_str, maybe_user_id_type},
        Pending, Query, Request,
    },
    routing::Route,
    Osu,
};

use futures::future::TryFutureExt;
use itoa::Buffer;
use serde::Serialize;
use smallstr::SmallString;
use std::fmt;

/// Either a user id as u32 or a username as String.
///
/// Use the `From` implementations to create this enum
///
/// # Example
///
/// ```
/// use rosu_v2::request::UserId;
///
/// let user_id: UserId = 123_456.into();
/// let user_id: UserId = "my username".into();
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UserId {
    /// Represents a user through their user id
    Id(u32),
    /// Represents a user through their username
    Name(Username),
}

impl From<u32> for UserId {
    #[inline]
    fn from(id: u32) -> Self {
        Self::Id(id)
    }
}

impl From<&str> for UserId {
    #[inline]
    fn from(name: &str) -> Self {
        Self::Name(SmallString::from_str(name))
    }
}

impl From<&String> for UserId {
    #[inline]
    fn from(name: &String) -> Self {
        Self::Name(SmallString::from_str(name))
    }
}

impl From<String> for UserId {
    #[inline]
    fn from(name: String) -> Self {
        Self::Name(SmallString::from_string(name))
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{id}"),
            Self::Name(name) => f.write_str(name),
        }
    }
}

/// Get the [`UserExtended`](crate::model::user::UserExtended) of the authenticated user.
///
/// Note that the client has to be initialized with the `identify` scope
/// through the OAuth process in order for this endpoint to not return an error.
///
/// See [`OsuBuilder::with_authorization`](crate::OsuBuilder::with_authorization).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetOwnData<'a> {
    fut: Option<Pending<'a, UserExtended>>,
    osu: &'a Osu,
    mode: Option<GameMode>,
}

impl<'a> GetOwnData<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            mode: None,
        }
    }

    /// Specify the mode for which the user data should be retrieved
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    fn start(&mut self) -> Pending<'a, UserExtended> {
        let req = Request::new(Route::GetOwnData { mode: self.mode });
        let osu = self.osu;
        let fut = osu.request::<UserExtended>(req);

        #[cfg(feature = "cache")]
        let fut = fut.inspect_ok(move |user| osu.update_cache(user.user_id, &user.username));

        Box::pin(fut)
    }
}

poll_req!(GetOwnData => UserExtended);

/// Get a [`UserExtended`](crate::model::user::UserExtended) by their id.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetUser<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, UserExtended>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(rename(serialize = "key"), serialize_with = "maybe_user_id_type")]
    user_id: Option<UserId>,
    #[serde(skip)]
    mode: Option<GameMode>,
}

impl<'a> GetUser<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: impl Into<UserId>) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id.into()),
            mode: None,
        }
    }

    /// Specify the mode for which the user data should be retrieved
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    fn start(&mut self) -> Pending<'a, UserExtended> {
        let query = Query::encode(self);
        let user_id = self.user_id.take().unwrap();

        let route = Route::GetUser {
            user_id,
            mode: self.mode,
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;
        let fut = osu.request::<UserExtended>(req);

        #[cfg(feature = "cache")]
        let fut = fut.inspect_ok(move |user| osu.update_cache(user.user_id, &user.username));

        Box::pin(fut)
    }
}

poll_req!(GetUser => UserExtended);

/// Get the [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended)s of a user by their id.
///
/// If no map type specified, either manually through
/// [`status`](crate::request::GetUserBeatmapsets::status),
/// or through any of the methods [`loved`](crate::request::GetUserBeatmapsets::loved),
/// [`graveyard`](crate::request::GetUserBeatmapsets::graveyard),
/// [`ranked`](crate::request::GetUserBeatmapsets::ranked),
/// [`pending`](crate::request::GetUserBeatmapsets::pending),
/// it defaults to `ranked`.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetUserBeatmapsets<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Vec<BeatmapsetExtended>>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    map_type: &'static str,
    limit: Option<usize>,
    offset: Option<usize>,

    #[cfg(not(feature = "cache"))]
    #[serde(skip)]
    user_id: u32,

    #[cfg(feature = "cache")]
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetUserBeatmapsets<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            map_type: "ranked_and_approved",
            limit: None,
            offset: None,
        }
    }

    #[cfg(feature = "cache")]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            map_type: "ranked",
            limit: None,
            offset: None,
        }
    }

    /// Limit the amount of results in the response
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub const fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);

        self
    }

    /// Only include mapsets with the specified status
    #[inline]
    pub const fn status(mut self, map_type: RankStatus) -> Self {
        self.map_type = match map_type {
            RankStatus::Approved | RankStatus::Ranked => "ranked",
            RankStatus::Graveyard => "graveyard",
            RankStatus::Pending | RankStatus::WIP | RankStatus::Qualified => "pending",
            RankStatus::Loved => "loved",
        };

        self
    }

    /// Require mapset rank status to be `loved`
    #[inline]
    pub const fn loved(mut self) -> Self {
        self.map_type = "loved";

        self
    }

    /// Require mapset rank status to be `graveyard`
    #[inline]
    pub const fn graveyard(mut self) -> Self {
        self.map_type = "graveyard";

        self
    }

    /// Require mapset rank status to be either `ranked` or `approved`
    #[inline]
    pub const fn ranked(mut self) -> Self {
        self.map_type = "ranked";

        self
    }

    /// Require mapset rank status to be `pending`
    #[inline]
    pub const fn pending(mut self) -> Self {
        self.map_type = "pending";

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<BeatmapsetExtended>> {
        let query = Query::encode(self);
        let map_type = self.map_type;
        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::with_query(Route::GetUserBeatmapsets { user_id, map_type }, query);

            Box::pin(osu.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let user_id = std::mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = osu
                .cache_user(user_id)
                .map_ok(move |user_id| {
                    Request::with_query(Route::GetUserBeatmapsets { user_id, map_type }, query)
                })
                .and_then(move |req| osu.request(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetUserBeatmapsets => Vec<BeatmapsetExtended>);

/// Get a user's kudosu history by their user id in form of a vec
/// of [`KudosuHistory`](crate::model::kudosu::KudosuHistory).
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetUserKudosu<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Vec<KudosuHistory>>>,
    #[serde(skip)]
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,

    #[cfg(not(feature = "cache"))]
    #[serde(skip)]
    user_id: u32,

    #[cfg(feature = "cache")]
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetUserKudosu<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            limit: None,
            offset: None,
        }
    }

    #[cfg(feature = "cache")]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            limit: None,
            offset: None,
        }
    }

    /// Limit the amount of results in the response
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub const fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<KudosuHistory>> {
        let query = Query::encode(self);
        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::with_query(Route::GetUserKudosu { user_id }, query);

            Box::pin(osu.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let user_id = std::mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = osu
                .cache_user(user_id)
                .map_ok(move |user_id| Request::with_query(Route::GetUserKudosu { user_id }, query))
                .and_then(move |req| osu.request(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetUserKudosu => Vec<KudosuHistory>);

/// Get the most played beatmaps of a user by their id in form
/// of a vec of [`MostPlayedMap`](crate::model::beatmap::MostPlayedMap).
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetUserMostPlayed<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Vec<MostPlayedMap>>>,
    #[serde(skip)]
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,

    #[cfg(not(feature = "cache"))]
    #[serde(skip)]
    user_id: u32,

    #[cfg(feature = "cache")]
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetUserMostPlayed<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            limit: None,
            offset: None,
        }
    }

    #[cfg(feature = "cache")]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            limit: None,
            offset: None,
        }
    }

    /// The API provides at most 51 results per requests.
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub const fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<MostPlayedMap>> {
        let query = Query::encode(self);
        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetUserBeatmapsets {
                user_id: self.user_id,
                map_type: "most_played",
            };

            let req = Request::with_query(route, query);

            Box::pin(osu.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let user_id = std::mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = osu
                .cache_user(user_id)
                .map_ok(move |user_id| {
                    let route = Route::GetUserBeatmapsets {
                        user_id,
                        map_type: "most_played",
                    };

                    Request::with_query(route, query)
                })
                .and_then(move |req| osu.request::<Vec<MostPlayedMap>>(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetUserMostPlayed => Vec<MostPlayedMap>);

/// Get a vec of [`Event`](crate::model::event::Event) of a user by their id.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetRecentActivity<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Vec<Event>>>,
    #[serde(skip)]
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,

    #[cfg(not(feature = "cache"))]
    #[serde(skip)]
    user_id: u32,

    #[cfg(feature = "cache")]
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetRecentActivity<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            limit: None,
            offset: None,
        }
    }

    #[cfg(feature = "cache")]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            limit: None,
            offset: None,
        }
    }

    /// Limit the amount of results in the response
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub const fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<Event>> {
        let query = Query::encode(self);
        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::with_query(Route::GetRecentActivity { user_id }, query);

            Box::pin(osu.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let user_id = std::mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = osu
                .cache_user(user_id)
                .map_ok(move |user_id| {
                    Request::with_query(Route::GetRecentActivity { user_id }, query)
                })
                .and_then(move |req| osu.request(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetRecentActivity => Vec<Event>);

#[derive(Copy, Clone, Debug)]
pub(crate) enum ScoreType {
    Best,
    First,
    Pinned,
    Recent,
}

impl ScoreType {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Best => "best",
            Self::First => "firsts",
            Self::Pinned => "pinned",
            Self::Recent => "recent",
        }
    }
}

/// Get a vec of [`Score`](crate::model::score::Score) of a user by the user's id.
///
/// If no score type is specified by either
/// [`best`](crate::request::GetUserScores::best),
/// [`firsts`](crate::request::GetUserScores::firsts),
/// or [`recent`](crate::request::GetUserScores::recent), it defaults to `best`.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetUserScores<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Vec<Score>>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    score_type: ScoreType,
    limit: Option<usize>,
    offset: Option<usize>,
    #[serde(serialize_with = "maybe_bool_as_u8")]
    include_fails: Option<bool>,
    #[serde(serialize_with = "maybe_mode_as_str")]
    mode: Option<GameMode>,
    #[serde(skip)]
    legacy_scores: bool,

    #[cfg(not(feature = "cache"))]
    #[serde(skip)]
    user_id: u32,

    #[cfg(feature = "cache")]
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetUserScores<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            score_type: ScoreType::Best,
            limit: None,
            offset: None,
            include_fails: None,
            mode: None,
            legacy_scores: false,
        }
    }

    #[cfg(feature = "cache")]
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            score_type: ScoreType::Best,
            limit: None,
            offset: None,
            include_fails: None,
            mode: None,
            legacy_scores: false,
        }
    }

    /// The API provides at most 100 results per requests.
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub const fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);

        self
    }

    /// Specify the mode of the scores
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Specify whether failed scores can be included.
    ///
    /// Only relevant for [`recent`](GetUserScores::recent)
    #[inline]
    pub const fn include_fails(mut self, include_fails: bool) -> Self {
        self.include_fails = Some(include_fails);

        self
    }

    /// Get top scores of a user
    #[inline]
    pub const fn best(mut self) -> Self {
        self.score_type = ScoreType::Best;

        self
    }

    /// Get global #1 scores of a user.
    #[inline]
    pub const fn firsts(mut self) -> Self {
        self.score_type = ScoreType::First;

        self
    }

    /// Get the pinned scores of a user.
    #[inline]
    pub const fn pinned(mut self) -> Self {
        self.score_type = ScoreType::Pinned;

        self
    }

    /// Get recent scores of a user.
    #[inline]
    pub const fn recent(mut self) -> Self {
        self.score_type = ScoreType::Recent;

        self
    }

    /// Specify whether the scores should contain legacy data or not.
    ///
    /// Legacy data consists of a different grade calculation, less
    /// populated statistics, legacy mods, and a different score kind.
    #[inline]
    pub const fn legacy_scores(mut self, legacy_scores: bool) -> Self {
        self.legacy_scores = legacy_scores;

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<Score>> {
        let query = Query::encode(self);
        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetUserScores {
                user_id: self.user_id,
                score_type: self.score_type,
            };

            let mut req = Request::with_query(route, query);

            if self.legacy_scores {
                req.api_version(0);
            }

            Box::pin(osu.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let score_type = self.score_type;
            let legacy_scores = self.legacy_scores;
            let user_id = std::mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = osu
                .cache_user(user_id)
                .map_ok(move |user_id| {
                    let route = Route::GetUserScores {
                        user_id,
                        score_type,
                    };

                    let mut req = Request::with_query(route, query);

                    if legacy_scores {
                        req.api_version(0);
                    }

                    req
                })
                .and_then(move |req| osu.request::<Vec<Score>>(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetUserScores => Vec<Score>);

/// Get a vec of [`User`](crate::model::user::User) by their ids.
#[allow(dead_code)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetUsers<'a> {
    fut: Option<Pending<'a, Vec<User>>>,
    osu: &'a Osu,
    query: Option<String>,
}

impl<'a> GetUsers<'a> {
    #[inline]
    pub(crate) fn new<I>(osu: &'a Osu, user_ids: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        let mut query = String::new();
        let mut buf = Buffer::new();

        let mut iter = user_ids.into_iter().take(50);

        if let Some(user_id) = iter.next() {
            query.push_str("ids[]=");
            query.push_str(buf.format(user_id));

            for user_id in iter {
                query.push_str("&ids[]=");
                query.push_str(buf.format(user_id));
            }
        }

        Self {
            fut: None,
            osu,
            query: Some(query),
        }
    }

    fn start(&mut self) -> Pending<'a, Vec<User>> {
        let query = self.query.take().unwrap();
        let req = Request::with_query(Route::GetUsers, query);
        let osu = self.osu;

        let fut = osu.request::<Users>(req);

        let fut = fut.map_ok(|users| {
            #[cfg(feature = "cache")]
            for user in users.users.iter() {
                osu.update_cache(user.user_id, &user.username);
            }

            users.users
        });

        Box::pin(fut)
    }
}

poll_req!(GetUsers => Vec<User>);
