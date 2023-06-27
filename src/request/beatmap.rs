use crate::{
    model::{
        beatmap::{
            Beatmap, Beatmapset, BeatmapsetEvents, BeatmapsetSearchResult, BeatmapsetSearchSort,
            Genre, Language, RankStatus,
        },
        beatmap_::{
            BeatmapDifficultyAttributes, BeatmapDifficultyAttributesWrapper, Beatmaps,
            SearchRankStatus,
        },
        score_::{BeatmapScores, BeatmapUserScore, Score, Scores},
        Cursor, GameMode, GameMods,
    },
    prelude::BeatmapCompact,
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

use futures::future::TryFutureExt;
use std::{fmt::Write, mem};

use super::Body;
#[cfg(feature = "cache")]
use super::UserId;

/// Get a [`Beatmap`](crate::model::beatmap::Beatmap).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmap<'a> {
    fut: Option<Pending<'a, Beatmap>>,
    osu: &'a Osu,
    checksum: Option<String>,
    filename: Option<String>,
    map_id: Option<u32>,
}

impl<'a> GetBeatmap<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
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
        self.checksum.replace(checksum.into());

        self
    }

    /// Specify a beatmap filename
    #[inline]
    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename.replace(filename.into());

        self
    }

    /// Specify a beatmap id
    #[inline]
    pub fn map_id(mut self, map_id: u32) -> Self {
        self.map_id.replace(map_id);

        self
    }

    fn start(&mut self) -> Pending<'a, Beatmap> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmap.inc();

        let mut query = Query::new();

        if let Some(ref checksum) = self.checksum {
            query.push("checksum", checksum);
        }

        if let Some(ref filename) = self.filename {
            query.push("filename", filename);
        }

        if let Some(map_id) = self.map_id {
            query.push("id", map_id);
        }

        let req = Request::with_query(Route::GetBeatmap, query);

        let osu = self.osu;
        let fut = osu.request::<Beatmap>(req);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmap => Beatmap);

/// Get a vec of [`BeatmapCompact`](crate::model::beatmap::BeatmapCompact) by their map ids.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmaps<'a> {
    fut: Option<Pending<'a, Vec<BeatmapCompact>>>,
    osu: &'a Osu,
    query: Query,
}

impl<'a> GetBeatmaps<'a> {
    #[inline]
    pub(crate) fn new<I>(osu: &'a Osu, map_ids: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        let mut query = Query::new();

        for map_id in map_ids.into_iter().take(50) {
            query.push("ids[]", map_id);
        }

        Self {
            fut: None,
            osu,
            query,
        }
    }

    fn start(&mut self) -> Pending<'a, Vec<BeatmapCompact>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmaps.inc();

        let query = mem::take(&mut self.query);
        let req = Request::with_query(Route::GetBeatmaps, query);
        let osu = self.osu;

        let fut = osu.request::<Beatmaps>(req).map_ok(|maps| maps.maps);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmaps => Vec<BeatmapCompact>);

/// Get [`BeatmapDifficultyAttributes`] of a map by its map id.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapDifficultyAttributes<'a> {
    fut: Option<Pending<'a, BeatmapDifficultyAttributes>>,
    osu: &'a Osu,
    map_id: u32,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
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
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Specify the mods
    #[inline]
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.mods = Some(mods);

        self
    }

    fn start(&mut self) -> Pending<'a, BeatmapDifficultyAttributes> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmap_difficulty_attributes.inc();

        let route = Route::GetBeatmapDifficultyAttributes {
            map_id: self.map_id,
        };

        let mut body = Body::default();

        if let Some(mods) = self.mods {
            body.push_without_quotes("mods", mods.bits());
        }

        if let Some(mode) = self.mode {
            body.push_without_quotes("ruleset_id", mode as u32);
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

/// Get top scores of a beatmap by its id in form of a
/// vec of [`Score`](Score)s.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapScores<'a> {
    fut: Option<Pending<'a, Vec<Score>>>,
    osu: &'a Osu,
    map_id: u32,
    score_type: Option<&'static str>,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
    limit: Option<u32>,
    // ! Currently not working
    // offset: Option<u32>,
}

impl<'a> GetBeatmapScores<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            score_type: None, // TODO
            mode: None,
            mods: None,
            limit: None,
            // offset: None,
        }
    }

    /// Specify the mode of the scores
    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    /// Specify the mods of the scores
    #[deprecated = "Does not currently work since the API requires osu!supporter for this feature"]
    #[inline]
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.mods.replace(mods);

        self
    }

    /// Specify the score type of the scores
    #[deprecated = "Does not currently work since the API requires osu!supporter for this feature"]
    #[inline]
    pub fn score_type(mut self, score_type: &'static str) -> Self {
        self.score_type.replace(score_type);

        self
    }

    #[inline]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

    // #[inline]
    // pub fn offset(mut self, offset: u32) -> Self {
    //     self.offset.replace(offset);

    //     self
    // }

    fn start(&mut self) -> Pending<'a, Vec<Score>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmap_scores.inc();

        let mut query = Query::new();

        if let Some(mode) = self.mode {
            query.push("mode", &mode.to_string());
        }

        if let Some(mods) = self.mods {
            for m in mods {
                query.push("mods[]", &m.to_string());
            }
        }

        if let Some(score_type) = self.score_type {
            query.push("type", score_type);
        }

        if let Some(limit) = self.limit {
            query.push("limit", limit);
        }

        // if let Some(offset) = self.offset {
        //     query.push("offset", offset);
        // }

        let route = Route::GetBeatmapScores {
            map_id: self.map_id,
        };

        let req = Request::with_query(route, query);
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
pub struct GetBeatmapUserScore<'a> {
    fut: Option<Pending<'a, BeatmapUserScore>>,
    osu: &'a Osu,
    map_id: u32,
    mode: Option<GameMode>,
    mods: Option<GameMods>,

    #[cfg(not(feature = "cache"))]
    user_id: u32,

    #[cfg(feature = "cache")]
    user_id: UserId,
}

impl<'a> GetBeatmapUserScore<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            user_id,
            mode: None,
            mods: None,
        }
    }

    #[cfg(feature = "cache")]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            user_id,
            mode: None,
            mods: None,
        }
    }

    /// Specify the mode
    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    /// Specify the mods
    #[inline]
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.mods.replace(mods);

        self
    }

    fn start(&mut self) -> Pending<'a, BeatmapUserScore> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmap_user_score.inc();

        let mut query = Query::new();

        if let Some(mode) = self.mode {
            query.push("mode", &mode.to_string());
        }

        if let Some(mods) = self.mods {
            for m in mods {
                query.push("mods[]", &m.to_string());
            }
        }

        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetBeatmapUserScore {
                user_id: self.user_id,
                map_id: self.map_id,
            };

            let req = Request::with_query(route, query);

            Box::pin(osu.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let map_id = self.map_id;
            let user_id = mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = self
                .osu
                .cache_user(user_id)
                .map_ok(move |user_id| {
                    Request::with_query(Route::GetBeatmapUserScore { user_id, map_id }, query)
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
pub struct GetBeatmapUserScores<'a> {
    fut: Option<Pending<'a, Vec<Score>>>,
    osu: &'a Osu,
    map_id: u32,
    mode: Option<GameMode>,

    #[cfg(not(feature = "cache"))]
    user_id: u32,

    #[cfg(feature = "cache")]
    user_id: UserId,
}

impl<'a> GetBeatmapUserScores<'a> {
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            user_id,
            mode: None,
        }
    }

    #[cfg(feature = "cache")]
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32, user_id: UserId) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            user_id,
            mode: None,
        }
    }

    /// Specify the mode
    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<Score>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmap_user_score.inc();

        let mut query = Query::new();

        if let Some(mode) = self.mode {
            query.push("mode", &mode.to_string());
        }

        let osu = self.osu;

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetBeatmapUserScores {
                user_id: self.user_id,
                map_id: self.map_id,
            };

            let req = Request::with_query(route, query);
            let fut = osu.request::<Scores>(req).map_ok(|scores| scores.scores);

            Box::pin(fut)
        }

        #[cfg(feature = "cache")]
        {
            let map_id = self.map_id;
            let user_id = mem::replace(&mut self.user_id, UserId::Id(0));

            let fut = self
                .osu
                .cache_user(user_id)
                .map_ok(move |user_id| {
                    Request::with_query(Route::GetBeatmapUserScores { user_id, map_id }, query)
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

/// Get a [`Beatmapset`](crate::model::beatmap::Beatmapset).
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapset<'a> {
    fut: Option<Pending<'a, Beatmapset>>,
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

    fn start(&mut self) -> Pending<'a, Beatmapset> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmapset.inc();

        let req = Request::new(Route::GetBeatmapset {
            mapset_id: self.mapset_id,
        });

        let osu = self.osu;
        let fut = osu.request::<Beatmapset>(req);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapset => Beatmapset);

/// Get a [`Beatmapset`](crate::model::beatmap::Beatmapset) from a beatmap ID.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapsetFromMapId<'a> {
    fut: Option<Pending<'a, Beatmapset>>,
    osu: &'a Osu,
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

    fn start(&mut self) -> Pending<'a, Beatmapset> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmapset_from_map_id.inc();

        let mut query = Query::new();

        query.push("beatmap_id", self.map_id);

        let req = Request::with_query(Route::GetBeatmapsetFromMapId, query);

        let osu = self.osu;
        let fut = osu.request::<Beatmapset>(req);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapsetFromMapId => Beatmapset);

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
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmapset_events.inc();

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
/// - nsfw: allowed
/// - sort: by relevance, descending
///
/// The contained [`Beatmapset`](crate::model::beatmap::Beatmapset)s will have the
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
    nsfw: bool,
    sort: Option<BeatmapsetSearchSort>,
    descending: bool,
    cursor: Option<Cursor>,
}

impl<'a> GetBeatmapsetSearch<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
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
            nsfw: true,
            sort: None,
            descending: true,
            cursor: None,
        }
    }

    /// Specify a search query.
    #[inline]
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query.replace(query.into());

        self
    }

    /// Specify the mode for which the mapsets has to have at least one map.
    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode as u8);

        self
    }

    /// Allow any status for mapsets. To specify a specific one, use the
    /// [`status`](crate::request::GetBeatmapsetSearch::status) method.
    #[inline]
    pub fn any_status(mut self) -> Self {
        self.status.replace(SearchRankStatus::Any);

        self
    }

    /// Specify a status for the mapsets, defaults to `has_leaderboard`
    /// i.e. ranked, loved, approved, and qualified. To allow any status, use the
    /// [`any_status`](crate::request::GetBeatmapsetSearch::any_status) method.
    ///
    /// ## Note
    /// The API does not seem to filter for the `RankStatus::Approved` status specifically.
    #[inline]
    pub fn status(mut self, mut status: RankStatus) -> Self {
        if status == RankStatus::WIP {
            status = RankStatus::Pending;
        }

        self.status.replace(SearchRankStatus::Specific(status));

        self
    }

    /// Specify a genre for the mapsets, defaults to `Any`.
    #[inline]
    pub fn genre(mut self, genre: Genre) -> Self {
        self.genre.replace(genre as u8);

        self
    }

    /// Specify a language for the mapsets, defaults to `Any`.
    #[inline]
    pub fn language(mut self, language: Language) -> Self {
        self.language.replace(language as u8);

        self
    }

    /// Specify whether mapsets can have a video, defaults to `false`.
    #[inline]
    pub fn video(mut self, video: bool) -> Self {
        self.video = video;

        self
    }

    /// Specify whether mapsets can have a storyboard, defaults to `false`.
    #[inline]
    pub fn storyboard(mut self, storyboard: bool) -> Self {
        self.storyboard = storyboard;

        self
    }

    /// Specify whether mapsets can be NSFW, defaults to `true`.
    #[inline]
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;

        self
    }

    /// Specify how the result should be sorted
    #[inline]
    pub fn sort(mut self, sort: BeatmapsetSearchSort, descending: bool) -> Self {
        self.sort.replace(sort);
        self.descending = descending;

        self
    }

    #[inline]
    pub(crate) fn cursor(mut self, cursor: Cursor) -> Self {
        self.cursor.replace(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, BeatmapsetSearchResult> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.beatmapset_search.inc();

        let q = self.query.take();
        let mode = self.mode;
        let status = self.status;
        let genre = self.genre;
        let language = self.language;
        let video = self.video;
        let storyboard = self.storyboard;
        let nsfw = self.nsfw;

        let mut query = Query::new();

        if let Some(ref q) = q {
            query.push("q", q);
        }

        if let Some(mode) = mode {
            query.push("m", mode);
        }

        match status {
            None => {}
            Some(SearchRankStatus::Specific(status)) => {
                let mut buf = String::new();
                let _ = write!(buf, "{:?}", status);

                // SAFETY: Debug formats of RankStatus are guaranteed
                // to only contain ASCII chars and have a length >= 1.
                unsafe { buf.as_bytes_mut()[0].make_ascii_lowercase() }

                query.push("s", buf);
            }
            Some(SearchRankStatus::Any) => {
                query.push("s", "any");
            }
        }

        if let Some(genre) = genre {
            query.push("g", genre);
        }

        if let Some(language) = language {
            query.push("l", language);
        }

        let extra = match (video, storyboard) {
            (false, false) => None,
            (false, true) => Some("storyboard"),
            (true, false) => Some("video"),
            (true, true) => Some("storyboard.video"),
        };

        if let Some(extra) = extra {
            query.push("e", extra);
        }

        query.push("nsfw", nsfw);

        if let Some(cursor) = self.cursor.take() {
            cursor.push_to_query(&mut query);
        }

        if let Some(ref sort) = self.sort {
            let mut buf = String::with_capacity(16);
            let _ = write!(buf, "{}_", sort);
            let order = if self.descending { "desc" } else { "asc" };
            buf.push_str(order);

            query.push("sort", buf);
        }

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
                params.nsfw = nsfw;

                search_result
            });

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapsetSearch => BeatmapsetSearchResult);

/// Get a [`Score`](crate::model::score::Score) struct.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetScore<'a> {
    fut: Option<Pending<'a, Score>>,
    osu: &'a Osu,
    mode: GameMode,
    score_id: u64,
}

impl<'a> GetScore<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, score_id: u64, mode: GameMode) -> Self {
        Self {
            fut: None,
            osu,
            mode,
            score_id,
        }
    }

    fn start(&mut self) -> Pending<'a, Score> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.score.inc();

        let route = Route::GetScore {
            mode: self.mode,
            score_id: self.score_id,
        };

        let osu = self.osu;
        let fut = osu.request::<Score>(Request::new(route));

        #[cfg(feature = "cache")]
        let fut = fut.inspect_ok(move |score| {
            if let Some(ref user) = score.user {
                osu.update_cache(user.user_id, &user.username);
            }
        });

        Box::pin(fut)
    }
}

poll_req!(GetScore => Score);
