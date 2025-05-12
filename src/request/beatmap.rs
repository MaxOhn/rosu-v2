use crate::{
    model::{
        beatmap::{
            BeatmapDifficultyAttributes, BeatmapDifficultyAttributesWrapper, Beatmaps,
            SearchRankStatus,
        },
        beatmap::{
            BeatmapExtended, BeatmapsetEvents, BeatmapsetExtended, BeatmapsetSearchResult,
            BeatmapsetSearchSort, Genre, Language, RankStatus,
        },
        score::{BeatmapScores, BeatmapUserScore, Score, Scores},
        GameMode,
    },
    prelude::{Beatmap, GameModsIntermode},
    request::{
        serialize::{maybe_mode_as_str, maybe_mods_as_list},
        Pending, Query, Request,
    },
    routing::Route,
    Osu,
};

use futures::future::TryFutureExt;
use itoa::Buffer;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::{fmt::Write, mem};

use super::{JsonBody, UserId};

/// Get a [`BeatmapExtended`](crate::model::beatmap::BeatmapExtended).
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetBeatmap<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, BeatmapExtended>>,
    #[serde(skip)]
    osu: &'a Osu,
    checksum: Option<String>,
    filename: Option<String>,
    #[serde(rename(serialize = "id"))]
    map_id: Option<u32>,
}

impl<'a> GetBeatmap<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            checksum: None,
            filename: None,
            map_id: None,
        }
    }

    /// Specify a beatmap checksum
    #[inline]
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());

        self
    }

    /// Specify a beatmap filename
    #[inline]
    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());

        self
    }

    /// Specify a beatmap id
    #[inline]
    pub const fn map_id(mut self, map_id: u32) -> Self {
        self.map_id = Some(map_id);

        self
    }

    fn start(&mut self) -> Pending<'a, BeatmapExtended> {
        let query = Query::encode(self);
        let req = Request::with_query(Route::GetBeatmap, query);

        let osu = self.osu;
        let fut = osu.request::<BeatmapExtended>(req);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmap => BeatmapExtended);

/// Get a vec of [`Beatmap`](crate::model::beatmap::Beatmap) by their map ids.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmaps<'a> {
    fut: Option<Pending<'a, Vec<Beatmap>>>,
    osu: &'a Osu,
    query: String,
}

impl<'a> GetBeatmaps<'a> {
    #[inline]
    pub(crate) fn new<I>(osu: &'a Osu, map_ids: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        let mut query = String::new();
        let mut buf = Buffer::new();

        let mut iter = map_ids.into_iter().take(50);

        if let Some(map_id) = iter.next() {
            query.push_str("ids[]=");
            query.push_str(buf.format(map_id));

            for map_id in iter {
                query.push_str("&ids[]=");
                query.push_str(buf.format(map_id));
            }
        }

        Self {
            fut: None,
            osu,
            query,
        }
    }

    fn start(&mut self) -> Pending<'a, Vec<Beatmap>> {
        let query = mem::take(&mut self.query);
        let req = Request::with_query(Route::GetBeatmaps, query);
        let osu = self.osu;

        let fut = osu.request::<Beatmaps>(req).map_ok(|maps| maps.maps);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmaps => Vec<Beatmap>);

/// Get [`BeatmapDifficultyAttributes`] of a map by its map id.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapDifficultyAttributes<'a> {
    fut: Option<Pending<'a, BeatmapDifficultyAttributes>>,
    osu: &'a Osu,
    map_id: u32,
    mode: Option<GameMode>,
    mods: Option<u32>,
}

impl<'a> GetBeatmapDifficultyAttributes<'a> {
    pub(crate) fn new(osu: &'a Osu, map_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            mode: None,
            mods: None,
        }
    }

    /// Specify the mode
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Specify the mods
    #[inline]
    pub fn mods<M>(mut self, mods: M) -> Self
    where
        GameModsIntermode: From<M>,
    {
        self.mods = Some(GameModsIntermode::from(mods).bits());

        self
    }

    fn start(&mut self) -> Pending<'a, BeatmapDifficultyAttributes> {
        let route = Route::PostBeatmapDifficultyAttributes {
            map_id: self.map_id,
        };

        let mut body = JsonBody::new();

        if let Some(mods) = self.mods {
            body.push_int("mods", mods);
        }

        if let Some(mode) = self.mode {
            body.push_int("ruleset_id", mode as u32);
        }

        let req = Request::with_body(route, body);

        let fut = self
            .osu
            .request::<BeatmapDifficultyAttributesWrapper>(req)
            .map_ok(|a| a.attributes);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapDifficultyAttributes => BeatmapDifficultyAttributes);

#[derive(Copy, Clone, Debug)]
enum ScoreType {
    Country,
    Global,
}

impl ScoreType {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Country => "country",
            Self::Global => "global",
        }
    }
}

impl Serialize for ScoreType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Get top scores of a beatmap by its id in form of a
/// vec of [`Score`](Score)s.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetBeatmapScores<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Vec<Score>>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    map_id: u32,
    #[serde(rename(serialize = "type"))]
    score_type: Option<ScoreType>,
    #[serde(serialize_with = "maybe_mode_as_str")]
    mode: Option<GameMode>,
    #[serde(flatten, serialize_with = "maybe_mods_as_list")]
    mods: Option<GameModsIntermode>,
    limit: Option<u32>,
    legacy_only: bool,
    #[serde(skip)]
    legacy_scores: bool,
    // ! Currently not working
    // offset: Option<u32>,
}

impl<'a> GetBeatmapScores<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, map_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            score_type: None,
            mode: None,
            mods: None,
            limit: None,
            legacy_only: false,
            legacy_scores: false,
            // offset: None,
        }
    }

    /// Specify the mode of the scores
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Specify the mods of the scores
    #[inline]
    pub fn mods<M>(mut self, mods: M) -> Self
    where
        GameModsIntermode: From<M>,
    {
        self.mods = Some(GameModsIntermode::from(mods));

        self
    }

    /// Specify that the global leaderboard should be requested.
    #[inline]
    pub const fn global(mut self) -> Self {
        self.score_type = Some(ScoreType::Global);

        self
    }

    /// Specify that the national leaderboard should be requested.
    ///
    /// Note that you must be authenticated through OAuth and have osu!supporter to use this.
    #[inline]
    pub const fn country(mut self) -> Self {
        self.score_type = Some(ScoreType::Country);

        self
    }

    #[inline]
    pub const fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);

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

    // #[inline]
    // pub fn offset(mut self, offset: u32) -> Self {
    //     self.offset = Some(offset);

    //     self
    // }

    fn start(&mut self) -> Pending<'a, Vec<Score>> {
        let query = Query::encode(self);

        let route = Route::GetBeatmapScores {
            map_id: self.map_id,
        };

        let mut req = Request::with_query(route, query);

        if self.legacy_scores {
            req.api_version(0);
        }

        let osu = self.osu;
        let fut = osu.request::<BeatmapScores>(req).map_ok(|s| s.scores);

        #[cfg(feature = "cache")]
        let fut = fut.inspect_ok(move |scores| {
            for score in scores.iter() {
                if let Some(ref user) = score.user {
                    osu.update_cache(user.user_id, &user.username);
                }
            }
        });

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapScores => Vec<Score>);

/// Get [`BeatmapUserScore`](crate::model::score::BeatmapUserScore)
/// of a user on a beatmap by the user's and the map's id.
///
/// Note that the contained score will be the user's play on the map
/// with the most **score** across all mods, not pp.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetBeatmapUserScore<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, BeatmapUserScore>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    map_id: u32,
    #[serde(serialize_with = "maybe_mode_as_str")]
    mode: Option<GameMode>,
    #[serde(flatten, serialize_with = "maybe_mods_as_list")]
    mods: Option<GameModsIntermode>,
    legacy_only: bool,
    #[serde(skip)]
    legacy_scores: bool,

    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetBeatmapUserScore<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, map_id: u32, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            user_id,
            mode: None,
            mods: None,
            legacy_only: false,
            legacy_scores: false,
        }
    }

    /// Specify the mode
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Specify the mods
    #[inline]
    pub fn mods<M>(mut self, mods: M) -> Self
    where
        GameModsIntermode: From<M>,
    {
        self.mods = Some(GameModsIntermode::from(mods));

        self
    }

    /// Whether or not to exclude lazer scores.
    #[inline]
    pub const fn legacy_only(mut self, legacy_only: bool) -> Self {
        self.legacy_only = legacy_only;

        self
    }

    /// Specify whether the score should contain legacy data or not.
    ///
    /// Legacy data consists of a different grade calculation, less
    /// populated statistics, legacy mods, and a different score kind.
    #[inline]
    pub const fn legacy_scores(mut self, legacy_scores: bool) -> Self {
        self.legacy_scores = legacy_scores;

        self
    }

    fn start(&mut self) -> Pending<'a, BeatmapUserScore> {
        let query = Query::encode(self);

        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetBeatmapUserScore {
                user_id: self.user_id,
                map_id: self.map_id,
            };

            let mut req = Request::with_query(route, query);

            if self.legacy_scores {
                req.api_version(0);
            }

            Box::pin(osu.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let map_id = self.map_id;
            let legacy_scores = self.legacy_scores;
            let user_id = mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = self
                .osu
                .cache_user(user_id)
                .map_ok(move |user_id| {
                    let mut req =
                        Request::with_query(Route::GetBeatmapUserScore { user_id, map_id }, query);

                    if legacy_scores {
                        req.api_version(0);
                    }

                    req
                })
                .and_then(move |req| osu.request::<BeatmapUserScore>(req))
                .inspect_ok(move |score| {
                    if let Some(ref user) = score.score.user {
                        osu.update_cache(user.user_id, &user.username);
                    }
                });

            Box::pin(fut)
        }
    }
}

poll_req!(GetBeatmapUserScore => BeatmapUserScore);

/// Get the top score with each mod combination of a user on
/// a map in the form of a vec of [`Score`]s.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetBeatmapUserScores<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Vec<Score>>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    map_id: u32,
    #[serde(serialize_with = "maybe_mode_as_str")]
    mode: Option<GameMode>,
    legacy_only: bool,
    #[serde(skip)]
    legacy_scores: bool,

    #[serde(skip)]
    user_id: UserId,
}

impl<'a> GetBeatmapUserScores<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, map_id: u32, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            user_id,
            mode: None,
            legacy_only: false,
            legacy_scores: false,
        }
    }

    /// Specify the mode
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

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

    fn start(&mut self) -> Pending<'a, Vec<Score>> {
        let query = Query::encode(self);

        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetBeatmapUserScores {
                user_id: self.user_id,
                map_id: self.map_id,
            };

            let mut req = Request::with_query(route, query);

            if self.legacy_scores {
                req.api_version(0);
            }

            let fut = osu.request::<Scores>(req).map_ok(|scores| scores.scores);

            Box::pin(fut)
        }

        #[cfg(feature = "cache")]
        {
            let map_id = self.map_id;
            let legacy_scores = self.legacy_scores;
            let user_id = mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = self
                .osu
                .cache_user(user_id)
                .map_ok(move |user_id| {
                    let mut req =
                        Request::with_query(Route::GetBeatmapUserScores { user_id, map_id }, query);

                    if legacy_scores {
                        req.api_version(0);
                    }

                    req
                })
                .and_then(move |req| osu.request::<Scores>(req))
                .map_ok(|scores| scores.scores)
                .inspect_ok(move |scores| {
                    for score in scores.iter() {
                        if let Some(ref user) = score.user {
                            osu.update_cache(user.user_id, &user.username);
                        }
                    }
                });

            Box::pin(fut)
        }
    }
}

poll_req!(GetBeatmapUserScores => Vec<Score>);

/// Get a [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapset<'a> {
    fut: Option<Pending<'a, BeatmapsetExtended>>,
    osu: &'a Osu,
    mapset_id: u32,
}

impl<'a> GetBeatmapset<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, mapset_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            mapset_id,
        }
    }

    fn start(&mut self) -> Pending<'a, BeatmapsetExtended> {
        let req = Request::new(Route::GetBeatmapset {
            mapset_id: self.mapset_id,
        });

        let osu = self.osu;
        let fut = osu.request::<BeatmapsetExtended>(req);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapset => BeatmapsetExtended);

/// Get a [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended) from a beatmap ID.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetBeatmapsetFromMapId<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, BeatmapsetExtended>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(rename(serialize = "beatmap_id"))]
    map_id: u32,
}

impl<'a> GetBeatmapsetFromMapId<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
        }
    }

    fn start(&mut self) -> Pending<'a, BeatmapsetExtended> {
        let query = Query::encode(self);
        let req = Request::with_query(Route::GetBeatmapsetFromMapId, query);

        let osu = self.osu;
        let fut = osu.request::<BeatmapsetExtended>(req);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapsetFromMapId => BeatmapsetExtended);

/// Get a [`BeatmapsetEvents`](crate::model::beatmap::BeatmapsetEvents) struct.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapsetEvents<'a> {
    fut: Option<Pending<'a, BeatmapsetEvents>>,
    osu: &'a Osu,
}

impl<'a> GetBeatmapsetEvents<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self { fut: None, osu }
    }

    fn start(&mut self) -> Pending<'a, BeatmapsetEvents> {
        let req = Request::new(Route::GetBeatmapsetEvents);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetBeatmapsetEvents => BeatmapsetEvents);

/// Get a [`BeatmapsetSearchResult`](crate::model::beatmap::BeatmapsetSearchResult)
/// struct containing the first page of maps that fit the search query.
///
/// The default search parameters are:
/// - mode: any
/// - status: has leaderboard (ranked, loved, approved, and qualified)
/// - genre: any
/// - language: any
/// - extra: does neither contain "have video" nor "have storyboard"
/// - general: recommended, converts, follows, spotlights, and featured artists are all disabled
/// - nsfw: allowed
/// - sort: by relevance, descending
///
/// The contained [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended)s will have the
/// following options filled: `artist_unicode`, `legacy_thread_url`, `maps`,
/// `ranked_date` and `submitted_date` if available, and `title_unicode`.
///
/// The search query allows the following options to be specified: `ar`, `artist`,
/// `bpm`, `created`, `creator`, `cs`, `dr` (hp drain rate), `keys`, `length`,
/// `ranked`, `stars`, and `status`.
///
/// ## Example
///
/// ```
/// // Search for mapsets from Sotarks that have a map with no more than AR 9.
/// let query = "creator=sotarks ar<9";
///
/// // Loved mapsets from Camellia including at least one map above 8 stars
/// let query = "status=loved artist=camellia stars>8";
/// ```
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapsetSearch<'a> {
    fut: Option<Pending<'a, BeatmapsetSearchResult>>,
    osu: &'a Osu,
    query: Option<String>,
    mode: Option<u8>,
    status: Option<SearchRankStatus>,
    genre: Option<u8>,
    language: Option<u8>,
    video: bool,
    storyboard: bool,
    recommended: bool,
    converts: bool,
    follows: bool,
    spotlights: bool,
    featured_artists: bool,
    nsfw: bool,
    sort: Option<BeatmapsetSearchSort>,
    descending: bool,
    page: Option<u32>,
    cursor: Option<&'a str>,
}

impl<'a> GetBeatmapsetSearch<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            query: None,
            mode: None,
            status: None,
            genre: None,
            language: None,
            video: false,
            storyboard: false,
            recommended: false,
            converts: false,
            follows: false,
            spotlights: false,
            featured_artists: false,
            nsfw: true,
            sort: None,
            descending: true,
            page: None,
            cursor: None,
        }
    }

    /// Specify a search query.
    #[inline]
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());

        self
    }

    /// Specify the mode for which the mapsets has to have at least one map.
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode as u8);

        self
    }

    /// Specify a status for the mapsets, defaults to `has_leaderboard`
    /// i.e. ranked, loved, approved, and qualified. To allow any status,
    /// specify `None`.
    ///
    /// ## Note
    /// The API does not seem to filter for the `RankStatus::Approved` status
    /// specifically.
    #[inline]
    pub const fn status(mut self, status: Option<RankStatus>) -> Self {
        let status = match status {
            Some(RankStatus::WIP) => SearchRankStatus::Specific(RankStatus::Pending),
            Some(status) => SearchRankStatus::Specific(status),
            None => SearchRankStatus::Any,
        };

        self.status = Some(status);

        self
    }

    /// Specify a genre for the mapsets, defaults to `Any`.
    #[inline]
    pub const fn genre(mut self, genre: Genre) -> Self {
        self.genre = Some(genre as u8);

        self
    }

    /// Specify a language for the mapsets, defaults to `Any`.
    #[inline]
    pub const fn language(mut self, language: Language) -> Self {
        self.language = Some(language as u8);

        self
    }

    /// Specify whether mapsets can have a video, defaults to `false`.
    #[inline]
    pub const fn video(mut self, video: bool) -> Self {
        self.video = video;

        self
    }

    /// Specify whether mapsets can have a storyboard, defaults to `false`.
    #[inline]
    pub const fn storyboard(mut self, storyboard: bool) -> Self {
        self.storyboard = storyboard;

        self
    }

    /// Only include mapsets containing a beatmap around the authorized user's
    /// recommended difficulty level.
    ///
    /// This has only an effect for oauth-clients.
    #[inline]
    pub const fn recommended(mut self, recommended: bool) -> Self {
        self.recommended = recommended;

        self
    }

    /// Specify whether converted mapsets should be included, defaults to
    /// `false`.
    #[inline]
    pub const fn converts(mut self, converts: bool) -> Self {
        self.converts = converts;

        self
    }

    /// Only include mapsets of mappers that the authorized user follows.
    ///
    /// This has only an effect for oauth-clients.
    #[inline]
    pub const fn follows(mut self, follows: bool) -> Self {
        self.follows = follows;

        self
    }

    /// Specify whether only mapsets that are currently spotlighted should be
    /// included, defaults to `false`.
    #[inline]
    pub const fn spotlights(mut self, spotlights: bool) -> Self {
        self.spotlights = spotlights;

        self
    }

    /// Specify whether only mapsets of featured artists should be included,
    /// defaults to `false`.
    #[inline]
    pub const fn featured_artists(mut self, featured_artists: bool) -> Self {
        self.featured_artists = featured_artists;

        self
    }

    /// Specify whether mapsets *can* be NSFW, defaults to `true`.
    #[inline]
    pub const fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;

        self
    }

    /// Specify a page
    #[inline]
    pub const fn page(mut self, page: u32) -> Self {
        self.page = Some(page);

        self
    }

    /// Specify how the result should be sorted
    #[inline]
    pub const fn sort(mut self, sort: BeatmapsetSearchSort, descending: bool) -> Self {
        self.sort = Some(sort);
        self.descending = descending;

        self
    }

    #[inline]
    pub(crate) const fn cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, BeatmapsetSearchResult> {
        let query = Query::encode(self);

        let q = self.query.take();
        let mode = self.mode;
        let status = self.status;
        let genre = self.genre;
        let language = self.language;
        let video = self.video;
        let storyboard = self.storyboard;
        let recommended = self.recommended;
        let converts = self.converts;
        let follows = self.follows;
        let spotlights = self.spotlights;
        let featured_artists = self.featured_artists;
        let nsfw = self.nsfw;

        let req = Request::with_query(Route::GetBeatmapsetSearch, query);
        let osu = self.osu;

        let fut = osu
            .request::<BeatmapsetSearchResult>(req)
            .map_ok(move |mut search_result| {
                let params = &mut search_result.params;
                params.query = q;
                params.mode = mode;
                params.status = status;
                params.genre = genre;
                params.language = language;
                params.video = video;
                params.storyboard = storyboard;
                params.recommended = recommended;
                params.converts = converts;
                params.follows = follows;
                params.spotlights = spotlights;
                params.featured_artists = featured_artists;
                params.nsfw = nsfw;

                search_result
            });

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapsetSearch => BeatmapsetSearchResult);

impl Serialize for GetBeatmapsetSearch<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;

        if let Some(ref query) = self.query {
            map.serialize_entry("q", query)?;
        }

        if let Some(ref mode) = self.mode {
            map.serialize_entry("m", mode)?;
        }

        if let Some(status) = self.status {
            struct Status(SearchRankStatus);

            impl Serialize for Status {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self.0 {
                        SearchRankStatus::Specific(status) => {
                            let mut buf = String::new();
                            let _ = write!(buf, "{status:?}");

                            // SAFETY: Debug formats of RankStatus are guaranteed
                            // to only contain ASCII chars and have a length >= 1.
                            unsafe { buf.as_bytes_mut()[0].make_ascii_lowercase() }

                            serializer.serialize_str(&buf)
                        }
                        SearchRankStatus::Any => serializer.serialize_str("any"),
                    }
                }
            }

            map.serialize_entry("s", &Status(status))?;
        }

        if let Some(ref genre) = self.genre {
            map.serialize_entry("g", genre)?;
        }

        if let Some(ref language) = self.language {
            map.serialize_entry("l", language)?;
        }

        let extra = match (self.video, self.storyboard) {
            (false, false) => None,
            (false, true) => Some("storyboard"),
            (true, false) => Some("video"),
            (true, true) => Some("storyboard.video"),
        };

        if let Some(ref extra) = extra {
            map.serialize_entry("e", extra)?;
        }

        let mut general = None::<String>;

        let mut add_general = |should_add: bool, value: &str| {
            if !should_add {
                return;
            }

            let is_some = general.is_some();
            let general = general.get_or_insert_with(String::new);

            if is_some {
                general.push('.');
            }

            general.push_str(value);
        };

        add_general(self.recommended, "recommended");
        add_general(self.converts, "converts");
        add_general(self.follows, "follows");
        add_general(self.spotlights, "spotlights");
        add_general(self.featured_artists, "featured_artists");

        if let Some(ref general) = general {
            map.serialize_entry("c", general)?;
        }

        map.serialize_entry("nsfw", &self.nsfw)?;

        if let Some(ref page) = self.page {
            map.serialize_entry("page", page)?;
        }

        if let Some(cursor) = self.cursor {
            map.serialize_entry("cursor_string", cursor)?;
        }

        if let Some(ref sort) = self.sort {
            let mut buf = String::with_capacity(16);
            let _ = write!(buf, "{sort}_");
            let order = if self.descending { "desc" } else { "asc" };
            buf.push_str(order);

            map.serialize_entry("sort", &buf)?;
        }

        map.end()
    }
}
