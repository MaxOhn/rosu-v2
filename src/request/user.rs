use crate::{
    error::OsuError,
    model::{
        beatmap::{Beatmapset, MostPlayedMap, RankStatus},
        kudosu::KudosuHistory,
        recent_event::RecentEvent,
        score::Score,
        user::{User, UserCompact},
        GameMode,
    },
    prelude::Username,
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

use smallstr::SmallString;
use std::fmt;

#[cfg(feature = "cache")]
use futures::future::TryFutureExt;

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
#[derive(Debug)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{}", id),
            Self::Name(name) => f.write_str(name),
        }
    }
}

/// Get the [`User`](crate::model::user::User) of the authenticated user.
///
/// Note that the client has to be initialized with the `identify` scope
/// through the OAuth process in order for this endpoint to not return an error.
///
/// See [`OsuBuilder::with_authorization`](crate::OsuBuilder::with_authorization).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetOwnData<'a> {
    fut: Option<Pending<'a, User>>,
    osu: &'a Osu,
    mode: Option<GameMode>,
}

impl<'a> GetOwnData<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            mode: None,
        }
    }

    /// Specify the mode for which the user data should be retrieved
    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    fn start(&mut self) -> Pending<'a, User> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.own_data.inc();

        let req = Request::new(Route::GetOwnData { mode: self.mode });

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetOwnData => User);

/// Get a [`User`](crate::model::user::User) by their id.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetUser<'a> {
    fut: Option<Pending<'a, User>>,
    osu: &'a Osu,
    user_id: Option<UserId>,
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
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    fn start(&mut self) -> Pending<'a, User> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.user.inc();

        let mut query = Query::new();

        let user_id = self.user_id.take().unwrap();

        let kind = match &user_id {
            UserId::Id(_) => "id",
            UserId::Name(_) => "username",
        };

        query.push("key", &kind);

        let route = Route::GetUser {
            user_id,
            mode: self.mode,
        };

        let req = Request::with_query(route, query);

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetUser => User);

/// Get the [`Beatmapset`](crate::model::beatmap::Beatmapset)s of a user by their id.
///
/// If no map type specified, either manually through
/// [`map_type`](crate::request::GetUserBeatmapsets::map_type),
/// or through any of the methods [`loved`](crate::request::GetUserBeatmapsets::loved),
/// [`graveyard`](crate::request::GetUserBeatmapsets::graveyard),
/// [`ranked`](crate::request::GetUserBeatmapsets::ranked),
/// [`pending`](crate::request::GetUserBeatmapsets::pending),
/// it defaults to `ranked`.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetUserBeatmapsets<'a> {
    fut: Option<Pending<'a, Vec<Beatmapset>>>,
    osu: &'a Osu,
    map_type: &'static str,
    limit: Option<usize>,
    offset: Option<usize>,

    #[cfg(not(feature = "cache"))]
    user_id: u32,

    #[cfg(feature = "cache")]
    user_id: Option<UserId>,
}

impl<'a> GetUserBeatmapsets<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: u32) -> Self {
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
    pub(crate) fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id),
            map_type: "ranked",
            limit: None,
            offset: None,
        }
    }

    /// Limit the amount of results in the response
    #[inline]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit.replace(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset.replace(offset);

        self
    }

    /// Only include mapsets with the specified status
    #[inline]
    pub fn status(mut self, map_type: RankStatus) -> Self {
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
    pub fn loved(mut self) -> Self {
        self.map_type = "loved";

        self
    }

    /// Require mapset rank status to be `graveyard`
    #[inline]
    pub fn graveyard(mut self) -> Self {
        self.map_type = "graveyard";

        self
    }

    /// Require mapset rank status to be either `ranked` or `approved`
    #[inline]
    pub fn ranked(mut self) -> Self {
        self.map_type = "ranked";

        self
    }

    /// Require mapset rank status to be `pending`
    #[inline]
    pub fn pending(mut self) -> Self {
        self.map_type = "pending";

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<Beatmapset>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.user_beatmapsets.inc();

        let map_type = self.map_type;
        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", &limit);
        }

        if let Some(offset) = self.offset {
            query.push("offset", &offset);
        }

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::with_query(Route::GetUserBeatmapsets { user_id, map_type }, query);

            Box::pin(self.osu.inner.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    Request::with_query(Route::GetUserBeatmapsets { user_id, map_type }, query)
                })
                .and_then(move |req| osu.request(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetUserBeatmapsets => Vec<Beatmapset>);

/// Get a user's kudosu history by their user id in form of a vec
/// of [`KudosuHistory`](crate::model::kudosu::KudosuHistory).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetUserKudosu<'a> {
    fut: Option<Pending<'a, Vec<KudosuHistory>>>,
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,

    #[cfg(not(feature = "cache"))]
    user_id: u32,

    #[cfg(feature = "cache")]
    user_id: Option<UserId>,
}

impl<'a> GetUserKudosu<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: u32) -> Self {
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
    pub(crate) fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id),
            limit: None,
            offset: None,
        }
    }

    /// Limit the amount of results in the response
    #[inline]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit.replace(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset.replace(offset);

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<KudosuHistory>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.user_kudosu.inc();

        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", &limit);
        }

        if let Some(offset) = self.offset {
            query.push("offset", &offset);
        }

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::with_query(Route::GetUserKudosu { user_id }, query);

            Box::pin(self.osu.inner.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
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
pub struct GetUserMostPlayed<'a> {
    fut: Option<Pending<'a, Vec<MostPlayedMap>>>,
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,

    #[cfg(not(feature = "cache"))]
    user_id: u32,

    #[cfg(feature = "cache")]
    user_id: Option<UserId>,
}

impl<'a> GetUserMostPlayed<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: u32) -> Self {
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
    pub(crate) fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id),
            limit: None,
            offset: None,
        }
    }

    /// The API provides at most 51 results per requests.
    #[inline]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit.replace(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset.replace(offset);

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<MostPlayedMap>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.most_played.inc();

        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", &limit);
        }

        if let Some(offset) = self.offset {
            query.push("offset", &offset);
        }

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetUserBeatmapsets {
                user_id: self.user_id,
                map_type: "most_played",
            };

            let req = Request::with_query(route, query);

            Box::pin(self.osu.inner.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    let route = Route::GetUserBeatmapsets {
                        user_id,
                        map_type: "most_played",
                    };

                    Request::with_query(route, query)
                })
                .and_then(move |req| osu.request(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetUserMostPlayed => Vec<MostPlayedMap>);

/// Get a vec of [`RecentEvent`](crate::model::recent_event::RecentEvent) of a user by their id.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetRecentEvents<'a> {
    fut: Option<Pending<'a, Vec<RecentEvent>>>,
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,

    #[cfg(not(feature = "cache"))]
    user_id: u32,

    #[cfg(feature = "cache")]
    user_id: Option<UserId>,
}

impl<'a> GetRecentEvents<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: u32) -> Self {
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
    pub(crate) fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id),
            limit: None,
            offset: None,
        }
    }

    /// Limit the amount of results in the response
    #[inline]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit.replace(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset.replace(offset);

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<RecentEvent>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.recent_events.inc();

        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", &limit);
        }

        if let Some(offset) = self.offset {
            query.push("offset", &offset);
        }

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::with_query(Route::GetRecentEvents { user_id }, query);

            Box::pin(self.osu.inner.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    Request::with_query(Route::GetRecentEvents { user_id }, query)
                })
                .and_then(move |req| osu.request(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetRecentEvents => Vec<RecentEvent>);

#[derive(Copy, Clone, Debug)]
pub(crate) enum ScoreType {
    Best,
    First,
    Recent,
}

impl fmt::Display for ScoreType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self {
            Self::Best => "best",
            Self::First => "firsts",
            Self::Recent => "recent",
        };

        f.write_str(kind)
    }
}

/// Get a vec of [`Score`](crate::model::score::Score) of a user by the user's id.
///
/// If no score type is specified by either
/// [`best`](crate::request::GetUserScores::best),
/// [`firsts`](crate::request::GetUserScores::firsts),
/// or [`recent`](crate::request::GetUserScores::recent), it defaults to `best`.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetUserScores<'a> {
    fut: Option<Pending<'a, Vec<Score>>>,
    osu: &'a Osu,
    score_type: ScoreType,
    limit: Option<usize>,
    offset: Option<usize>,
    include_fails: Option<bool>,
    mode: Option<GameMode>,

    #[cfg(not(feature = "cache"))]
    user_id: u32,

    #[cfg(feature = "cache")]
    user_id: Option<UserId>,
}

impl<'a> GetUserScores<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            score_type: ScoreType::Best,
            limit: None,
            offset: None,
            include_fails: None,
            mode: None,
        }
    }

    #[cfg(feature = "cache")]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id),
            score_type: ScoreType::Best,
            limit: None,
            offset: None,
            include_fails: None,
            mode: None,
        }
    }

    /// The API provides at most 100 results per requests.
    #[inline]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit.replace(limit);

        self
    }

    /// Set an offset for the requested elements
    /// e.g. skip the first `offset` amount in the list
    #[inline]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset.replace(offset);

        self
    }

    /// Specify the mode of the scores
    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    /// Specify whether failed scores can be included.
    ///
    /// Only relevant for [`recent`](GetUserScores::recent)
    #[inline]
    pub fn include_fails(mut self, include_fails: bool) -> Self {
        self.include_fails.replace(include_fails);

        self
    }

    /// Get top scores of a user
    #[inline]
    pub fn best(mut self) -> Self {
        self.score_type = ScoreType::Best;

        self
    }

    /// Get global #1 scores of a user.
    #[inline]
    pub fn firsts(mut self) -> Self {
        self.score_type = ScoreType::First;

        self
    }

    /// Get recent scores of a user.
    #[inline]
    pub fn recent(mut self) -> Self {
        self.score_type = ScoreType::Recent;

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<Score>> {
        #[cfg(feature = "metrics")]
        match self.score_type {
            ScoreType::Best => self.osu.metrics.user_top_scores.inc(),
            ScoreType::First => self.osu.metrics.user_first_scores.inc(),
            ScoreType::Recent => self.osu.metrics.user_recent_scores.inc(),
        }

        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", &limit);
        }

        if let Some(offset) = self.offset {
            query.push("offset", &offset);
        }

        if let Some(mode) = self.mode {
            query.push("mode", &mode.to_string());
        }

        if let Some(include_fails) = self.include_fails {
            query.push("include_fails", &(include_fails as u8));
        }

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetUserScores {
                user_id: self.user_id,
                score_type: self.score_type,
            };

            let req = Request::with_query(route, query);

            Box::pin(self.osu.inner.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let score_type = self.score_type;
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    let route = Route::GetUserScores {
                        user_id,
                        score_type,
                    };

                    Request::with_query(route, query)
                })
                .and_then(move |req| osu.request(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetUserScores => Vec<Score>);

/// Get a vec of [`UserCompact`](crate::model::user::UserCompact) by their ids.
#[allow(dead_code)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetUsers<'a> {
    fut: Option<Pending<'a, Vec<UserCompact>>>,
    osu: &'a Osu,
    form: Option<Query>,
}

impl<'a> GetUsers<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_ids: &[u32]) -> Self {
        let mut query = Query::new();

        for user_id in user_ids.iter().take(50) {
            query.push("id[]", user_id);
        }

        Self {
            fut: None,
            osu,
            form: Some(query),
        }
    }

    fn start(&mut self) -> Pending<'a, Vec<UserCompact>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.users.inc();

        Box::pin(async { Err(OsuError::UnavailableEndpoint) })

        // let query = self.query.take().unwrap();
        // let req = Request::from((query, Route::GetUsers));

        // Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetUsers => Vec<UserCompact>);
