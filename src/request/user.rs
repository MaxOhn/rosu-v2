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
    request::{Pending, Query, Request},
    routing::Route,
    Osu, OsuResult,
};

use std::fmt;

#[cfg(feature = "cache")]
use futures::future::TryFutureExt;

#[derive(Debug)]
pub enum UserId {
    Id(u32),
    Name(String),
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
        Self::Name(name.to_owned())
    }
}

impl From<&String> for UserId {
    #[inline]
    fn from(name: &String) -> Self {
        Self::Name(name.to_owned())
    }
}

impl From<String> for UserId {
    #[inline]
    fn from(name: String) -> Self {
        Self::Name(name)
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

/// Get the beatmapsets of a user by their id.
///
/// The map type **must** be specified before awaiting, either manually through
/// [`map_type`](crate::request::GetUserBeatmapsets::map_type),
/// or through any of the methods [`loved`](crate::request::GetUserBeatmapsets::loved),
/// [`favourite`](crate::request::GetUserBeatmapsets::favourite),
/// [`graveyard`](crate::request::GetUserBeatmapsets::graveyard),
/// [`ranked_and_approved`](crate::request::GetUserBeatmapsets::ranked_and_approved),
/// [`unranked`](crate::request::GetUserBeatmapsets::unranked).
pub struct GetUserBeatmapsets<'a> {
    fut: Option<Pending<'a, Vec<Beatmapset>>>,
    osu: &'a Osu,
    map_type: Option<&'static str>,
    limit: Option<u32>,
    offset: Option<u32>,

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
            map_type: None,
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
            map_type: None,
            limit: None,
            offset: None,
        }
    }

    #[inline]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset.replace(offset);

        self
    }

    pub fn map_type(mut self, map_type: RankStatus) -> Self {
        let map_type = match map_type {
            RankStatus::Approved | RankStatus::Ranked => "ranked_and_approved",
            RankStatus::Graveyard => "graveyard",
            RankStatus::Pending | RankStatus::WIP | RankStatus::Qualified => "unranked",
            RankStatus::Loved => "loved",
        };

        self.map_type.replace(map_type);

        self
    }

    #[inline]
    pub fn loved(mut self) -> Self {
        self.map_type.replace("loved");

        self
    }

    #[inline]
    pub fn favourite(mut self) -> Self {
        self.map_type.replace("favourite");

        self
    }

    #[inline]
    pub fn graveyard(mut self) -> Self {
        self.map_type.replace("graveyard");

        self
    }

    #[inline]
    pub fn ranked_and_approved(mut self) -> Self {
        self.map_type.replace("ranked_and_approved");

        self
    }

    #[inline]
    pub fn unranked(mut self) -> Self {
        self.map_type.replace("unranked");

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let map_type = self
            .map_type
            .ok_or(OsuError::MissingParameter { param: "map type" })?;

        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            query.push("offset", offset.to_string());
        }

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::from((query, Route::GetUserBeatmapsets { user_id, map_type }));
            self.fut.replace(Box::pin(self.osu.inner.request(req)));
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    Request::from((query, Route::GetUserBeatmapsets { user_id, map_type }))
                })
                .and_then(move |req| osu.request(req));

            self.fut.replace(Box::pin(fut));
        }

        Ok(())
    }
}

poll_req!(GetUserBeatmapsets<'_> => OsuResult<Vec<Beatmapset>>);

/// Get the recent events of a user by their id.
pub struct GetRecentEvents<'a> {
    fut: Option<Pending<'a, Vec<RecentEvent>>>,
    osu: &'a Osu,
    limit: Option<u32>,
    offset: Option<u32>,

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

    #[inline]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset.replace(offset);

        self
    }

    fn start(&mut self) {
        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            query.push("offset", offset.to_string());
        }

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::from((query, Route::GetRecentEvents { user_id }));
            self.fut.replace(Box::pin(self.osu.inner.request(req)));
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| Request::from((query, Route::GetRecentEvents { user_id })))
                .and_then(move |req| osu.request(req));

            self.fut.replace(Box::pin(fut));
        }
    }
}

poll_req!(GetRecentEvents<'_> => Vec<RecentEvent>);

/// Get a user's kudosu history by their user id.
pub struct GetUserKudosu<'a> {
    fut: Option<Pending<'a, Vec<KudosuHistory>>>,
    osu: &'a Osu,
    limit: Option<u32>,
    offset: Option<u32>,

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

    #[inline]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset.replace(offset);

        self
    }

    fn start(&mut self) {
        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            query.push("offset", offset.to_string());
        }

        #[cfg(not(feature = "cache"))]
        {
            let user_id = self.user_id;
            let req = Request::from((query, Route::GetUserKudosu { user_id }));
            self.fut.replace(Box::pin(self.osu.inner.request(req)));
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| Request::from((query, Route::GetUserKudosu { user_id })))
                .and_then(move |req| osu.request(req));

            self.fut.replace(Box::pin(fut));
        }
    }
}

poll_req!(GetUserKudosu<'_> => Vec<KudosuHistory>);

/// Get the most played beatmaps of a user by their id.
pub struct GetUserMostPlayed<'a> {
    fut: Option<Pending<'a, Vec<MostPlayedMap>>>,
    osu: &'a Osu,
    limit: Option<u32>,
    offset: Option<u32>,

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

    #[inline]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset.replace(offset);

        self
    }

    fn start(&mut self) {
        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            query.push("offset", offset.to_string());
        }

        #[cfg(not(feature = "cache"))]
        {
            let req = Request::from((
                query,
                Route::GetUserBeatmapsets {
                    user_id: self.user_id,
                    map_type: "most_played",
                },
            ));

            self.fut.replace(Box::pin(self.osu.inner.request(req)));
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    Request::from((
                        query,
                        Route::GetUserBeatmapsets {
                            user_id,
                            map_type: "most_played",
                        },
                    ))
                })
                .and_then(move |req| osu.request(req));

            self.fut.replace(Box::pin(fut));
        }
    }
}

poll_req!(GetUserMostPlayed<'_> => Vec<MostPlayedMap>);

/// Get scores of a user by the user's id.
///
/// Either of the following methods **must** be specified before awaiting:
/// [`best`](crate::request::GetUserScores::best),
/// [`firsts`](crate::request::GetUserScores::firsts),
/// [`recent`](crate::request::GetUserScores::recent).
pub struct GetUserScores<'a> {
    fut: Option<Pending<'a, Vec<Score>>>,
    osu: &'a Osu,
    score_type: Option<&'static str>,
    limit: Option<u32>,
    offset: Option<u32>,
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
            score_type: None,
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
            score_type: None,
            limit: None,
            offset: None,
            include_fails: None,
            mode: None,
        }
    }

    #[inline]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset.replace(offset);

        self
    }

    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    #[inline]
    pub fn include_fails(mut self, include_fails: bool) -> Self {
        self.include_fails.replace(include_fails);

        self
    }

    #[inline]
    pub fn best(mut self) -> Self {
        self.score_type.replace("best");

        self
    }

    #[inline]
    pub fn firsts(mut self) -> Self {
        self.score_type.replace("firsts");

        self
    }

    #[inline]
    pub fn recent(mut self) -> Self {
        self.score_type.replace("recent");

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let score_type = self.score_type.ok_or(OsuError::MissingParameter {
            param: "score type",
        })?;

        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            query.push("offset", offset.to_string());
        }

        if let Some(mode) = self.mode {
            query.push("mode", mode.to_string());
        }

        #[cfg(not(feature = "cache"))]
        {
            let req = Request::from((
                query,
                Route::GetUserScores {
                    user_id: self.user_id,
                    score_type,
                },
            ));

            self.fut.replace(Box::pin(self.osu.inner.request(req)));
        }

        #[cfg(feature = "cache")]
        {
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    Request::from((
                        query,
                        Route::GetUserScores {
                            user_id,
                            score_type,
                        },
                    ))
                })
                .and_then(move |req| osu.request(req));

            self.fut.replace(Box::pin(fut));
        }

        Ok(())
    }
}

poll_req!(GetUserScores<'_> => OsuResult<Vec<Score>>);

/// Get a user by their id.
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

    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    fn start(&mut self) {
        let req = Request::from(Route::GetUser {
            user_id: self.user_id.take().unwrap(),
            mode: self.mode,
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetUser<'_> => User);

/// Get multiple users by their ids.
pub struct GetUsers<'a> {
    fut: Option<Pending<'a, Vec<UserCompact>>>,
    osu: &'a Osu,
    query: Option<Query>,
}

impl<'a> GetUsers<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_ids: &[u32]) -> Self {
        let mut query = Query::new();

        let iter = user_ids
            .iter()
            .take(50)
            .map(|user_id| ("id[]", user_id.to_string()));

        query.extend(iter);

        Self {
            fut: None,
            osu,
            query: Some(query),
        }
    }

    fn start(&mut self) {
        let query = self.query.take().unwrap();
        let req = Request::from((query, Route::GetUsers));
        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetUsers<'_> => Vec<UserCompact>);
