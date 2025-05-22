use std::fmt;

use itoa::Buffer;
use serde::Serialize;
use smallstr::SmallString;

use crate::{
    model::{
        beatmap::{BeatmapsetExtended, MostPlayedMap},
        event::Event,
        kudosu::KudosuHistory,
        score::Score,
        user::{User, UserBeatmapsetsKind, UserExtended, Username},
        DeserializedList, GameMode,
    },
    request::{
        serialize::{maybe_bool_as_u8, maybe_mode_as_str, user_id_type},
        Query, Request,
    },
    routing::Route,
    Osu,
};

/// Either a user id as `u32` or a username as [`Username`].
///
/// Use the `From` implementations to create this enum.
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

/// Get the [`UserExtended`] of the authenticated user.
///
/// Note that the client has to be initialized with the `Identify` scope
/// through the OAuth process in order for this endpoint to not return an error.
///
/// See [`OsuBuilder::with_authorization`](crate::OsuBuilder::with_authorization).
#[must_use = "requests must be configured and executed"]
#[derive(Clone)]
pub struct GetOwnData<'a> {
    osu: &'a Osu,
    mode: Option<GameMode>,
}

impl<'a> GetOwnData<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu, mode: None }
    }

    /// Specify the mode for which the user data should be retrieved
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }
}

into_future! {
    |self: GetOwnData<'_>| -> UserExtended {
        Request::new(Route::GetOwnData { mode: self.mode })
    }
}

/// Get all friends of the authenticated user as a vec of [`User`].
///
/// Note that the client has to be initialized with the `FriendsRead` scope
/// through the OAuth process in order for this endpoint to not return an error.
///
/// See [`OsuBuilder::with_authorization`](crate::OsuBuilder::with_authorization).
#[must_use = "requests must be configured and executed"]
#[derive(Clone)]
pub struct GetFriends<'a> {
    osu: &'a Osu,
}

impl<'a> GetFriends<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu }
    }
}

into_future! {
    |self: GetFriends<'_>| -> Vec<User> {
        Request::new(Route::GetFriends)
    }
}

/// Get a [`UserExtended`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone)]
pub struct GetUser<'a> {
    osu: &'a Osu,
    user_id: UserId,
    mode: Option<GameMode>,
}

impl<'a> GetUser<'a> {
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            osu,
            user_id,
            mode: None,
        }
    }

    /// Specify the mode for which the user data should be retrieved
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Auxiliary function so that [`GetUser`]'s future can be created without
    /// an actual [`GetUser`] instance.
    ///
    /// Used for username caching.
    pub(crate) fn create_request(user_id: UserId, mode: Option<GameMode>) -> Request {
        #[derive(Serialize)]
        pub struct UserQuery {
            #[serde(rename(serialize = "key"), serialize_with = "user_id_type")]
            user_id: UserId,
        }

        let user_query = UserQuery { user_id };
        let query = Query::encode(&user_query);
        let user_id = user_query.user_id;

        let route = Route::GetUser { user_id, mode };

        Request::with_query(route, query)
    }
}

into_future! {
    |self: GetUser<'_>| -> UserExtended {
        Self::create_request(self.user_id, self.mode)
    }
}

/// Get the [`BeatmapsetExtended`]s of a user.
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetUserBeatmapsets<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    map_kind: UserBeatmapsetsKind,
    limit: Option<usize>,
    offset: Option<usize>,
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetUserBeatmapsets<'a> {
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId, kind: UserBeatmapsetsKind) -> Self {
        Self {
            osu,
            user_id,
            map_kind: kind,
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

    /// Only include mapsets of the specified type
    #[inline]
    pub const fn kind(mut self, kind: UserBeatmapsetsKind) -> Self {
        self.map_kind = kind;

        self
    }
}

into_future! {
    |self: GetUserBeatmapsets<'_>| -> Vec<BeatmapsetExtended> {
        GetUserBeatmapsetsData {
            map_kind: UserBeatmapsetsKind = self.map_kind,
            query: String = Query::encode(&self),
        }
    } => |user_id, data| {
        Request::with_query(
            Route::GetUserBeatmapsets {
                user_id,
                map_type: data.map_kind.as_str(),
            },
            data.query,
        )
    }
}

/// Get a user's kudosu history as a vec of [`KudosuHistory`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetUserKudosu<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetUserKudosu<'a> {
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
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
}

into_future! {
    |self: GetUserKudosu<'_>| -> Vec<KudosuHistory> {
        GetUserKudosuData {
            query: String = Query::encode(&self),
        }
    } => |user_id, data| {
        Request::with_query(Route::GetUserKudosu { user_id }, data.query)
    }
}

/// Get the most played beatmaps of a user as a vec of [`MostPlayedMap`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetUserMostPlayed<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetUserMostPlayed<'a> {
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
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
}

into_future! {
    |self: GetUserMostPlayed<'_>| -> Vec<MostPlayedMap> {
        GetUserMostPlayedData {
            query: String = Query::encode(&self),
        }
    } => |user_id, data| {
        let route = Route::GetUserBeatmapsets {
            user_id,
            map_type: "most_played",
        };

        Request::with_query(route, data.query)
    }
}

/// Get a vec of [`Event`] of a user.
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetRecentActivity<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    limit: Option<usize>,
    offset: Option<usize>,
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetRecentActivity<'a> {
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
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
}

into_future! {
    |self: GetRecentActivity<'_>| -> Vec<Event> {
        GetRecentActivityData {
            query: String = Query::encode(&self),
        }
    } => |user_id, data| {
        Request::with_query(Route::GetRecentActivity { user_id }, data.query)
    }
}

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

/// Get a vec of [`Score`]s of a user.
///
/// If no score type is specified by either
/// [`best`](crate::request::GetUserScores::best),
/// [`firsts`](crate::request::GetUserScores::firsts),
/// or [`recent`](crate::request::GetUserScores::recent), it defaults to `best`.
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetUserScores<'a> {
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
    legacy_only: bool,
    #[serde(skip)]
    legacy_scores: bool,
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetUserScores<'a> {
    pub(crate) const fn new(osu: &'a Osu, user_id: UserId) -> Self {
        Self {
            osu,
            user_id,
            score_type: ScoreType::Best,
            limit: None,
            offset: None,
            include_fails: None,
            mode: None,
            legacy_only: false,
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

    /// Whether or not to exclude lazer scores.
    #[inline]
    pub const fn legacy_only(mut self, legacy_only: bool) -> Self {
        self.legacy_only = legacy_only;

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
}

into_future! {
    |self: GetUserScores<'_>| -> Vec<Score> {
        GetUserScoresData {
            query: String = Query::encode(&self),
            score_type: ScoreType = self.score_type,
            legacy_scores: bool = self.legacy_scores,
        }
    } => |user_id, data| {
        let route = Route::GetUserScores {
            user_id,
            score_type: data.score_type,
        };

        let mut req = Request::with_query(route, data.query);

        if data.legacy_scores {
            req.api_version(0);
        }

        req
    }
}

/// Get a vec of [`User`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone)]
pub struct GetUsers<'a> {
    osu: &'a Osu,
    query: String,
}

impl<'a> GetUsers<'a> {
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

        Self { osu, query }
    }
}

into_future! {
    |self: GetUsers<'_>| -> DeserializedList<User> {
        Request::with_query(Route::GetUsers, self.query)
    } => |users, _| -> Vec<User> {
        Ok(users.0)
    }
}
