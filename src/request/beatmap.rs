use crate::{
    model::{
        beatmap::{
            Beatmap, Beatmapset, BeatmapsetEvents, BeatmapsetSearchCursor, BeatmapsetSearchResult,
            BeatmapsetSearchSort, Genre, Language, RankStatus, SearchRankStatus,
        },
        score::{BeatmapScores, BeatmapUserScore, Score},
        GameMode, GameMods,
    },
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

use futures::future::TryFutureExt;
use std::fmt::Write;

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

    #[inline]
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum.replace(checksum.into());

        self
    }

    #[inline]
    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename.replace(filename.into());

        self
    }

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
            query.push("id", &map_id.to_string());
        }

        let req = Request::new(Route::GetBeatmap).query(query);

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetBeatmap => Beatmap);

/// Get top scores of a beatmap by its id in form of a
/// vec of [`Score`](crate::model::score::Score)s.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetBeatmapScores<'a> {
    fut: Option<Pending<'a, Vec<Score>>>,
    osu: &'a Osu,
    map_id: u32,
    score_type: Option<&'static str>,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
    // ! Currently not working
    // limit: Option<u32>,
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
            // limit: None,
            // offset: None,
        }
    }

    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    #[deprecated = "Does not currently work since the API requires osu!supporter for this feature"]
    #[inline]
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.mods.replace(mods);

        self
    }

    #[deprecated = "Does not currently work since the API requires osu!supporter for this feature"]
    #[inline]
    pub fn score_type(mut self, score_type: &'static str) -> Self {
        self.score_type.replace(score_type);

        self
    }

    // #[inline]
    // pub fn limit(mut self, limit: u32) -> Self {
    //     self.limit.replace(limit);

    //     self
    // }

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
            query.push("type", &score_type);
        }

        // if let Some(limit) = self.limit {
        //     query.push("limit", limit.to_string());
        // }

        // if let Some(offset) = self.offset {
        //     query.push("offset", offset.to_string());
        // }

        let route = Route::GetBeatmapScores {
            map_id: self.map_id,
        };

        let req = Request::new(route).query(query);

        let fut = self
            .osu
            .inner
            .request(req)
            .map_ok(|s: BeatmapScores| s.scores);

        Box::pin(fut)
    }
}

poll_req!(GetBeatmapScores => Vec<Score>);

/// Get [`BeatmapUserScore`](crate::model::score::BeatmapUserScore)
/// of a user on a beatmap by the user's and the map's id.
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
    user_id: Option<UserId>,
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
            user_id: Some(user_id),
            mode: None,
            mods: None,
        }
    }

    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

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

        #[cfg(not(feature = "cache"))]
        {
            let route = Route::GetBeatmapUserScore {
                user_id: self.user_id,
                map_id: self.map_id,
            };

            let req = Request::new(route).query(query);

            Box::pin(self.osu.inner.request(req))
        }

        #[cfg(feature = "cache")]
        {
            let map_id = self.map_id;
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    Request::new(Route::GetBeatmapUserScore { user_id, map_id }).query(query)
                })
                .and_then(move |req| osu.request(req));

            Box::pin(fut)
        }
    }
}

poll_req!(GetBeatmapUserScore => BeatmapUserScore);

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

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetBeatmapset => Beatmapset);

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

        Box::pin(self.osu.inner.request(req))
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
/// - extra: does neither contain have video nor storyboard
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
    cursor: Option<BeatmapsetSearchCursor>,
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

    /// Specify whether mapsets can have a video, defaults to false.
    #[inline]
    pub fn video(mut self, video: bool) -> Self {
        self.video = video;

        self
    }

    /// Specify whether mapsets can have a storyboard, defaults to false.
    #[inline]
    pub fn storyboard(mut self, storyboard: bool) -> Self {
        self.storyboard = storyboard;

        self
    }

    /// Specify whether mapsets can be NSFW, defaults to true.
    #[inline]
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;

        self
    }

    #[inline]
    pub fn sort(mut self, sort: BeatmapsetSearchSort, descending: bool) -> Self {
        self.sort.replace(sort);
        self.descending = descending;

        self
    }

    #[inline]
    pub(crate) fn cursor(mut self, cursor: BeatmapsetSearchCursor) -> Self {
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
            query.push("m", &mode.to_string());
        }

        match status {
            None => {}
            Some(SearchRankStatus::Specific(status)) => {
                let mut buf = String::new();
                let _ = write!(buf, "{:?}", status);

                // SAFETY: Debug formats of RankStatus are guaranteed
                // to only contain ASCII chars and have a length >= 1.
                unsafe { buf.as_bytes_mut()[0].make_ascii_lowercase() }

                let _ = query.push("s", &buf);
            }
            Some(SearchRankStatus::Any) => {
                let _ = query.push("s", &"any");
            }
        }

        if let Some(genre) = genre {
            query.push("g", &genre.to_string());
        }

        if let Some(language) = language {
            query.push("l", &language.to_string());
        }

        let extra = match (video, storyboard) {
            (false, false) => None,
            (false, true) => Some("storyboard"),
            (true, false) => Some("video"),
            (true, true) => Some("storyboard.video"),
        };

        if let Some(extra) = extra {
            query.push("e", &extra);
        }

        query.push("nsfw", &nsfw.to_string());

        if let Some(cursor) = self.cursor.take() {
            query.push("cursor[_id]", &cursor.id);

            if let Some(score) = cursor.score {
                query.push("cursor[_score]", &score.to_string());
            }

            if let Some(playcount) = cursor.playcount {
                query.push("cursor[play_count]", &playcount);
            }
        }

        if let Some(ref sort) = self.sort {
            let mut buf = String::with_capacity(16);
            let _ = write!(buf, "{}_", sort);
            let order = if self.descending { "desc" } else { "asc" };
            buf.push_str(order);

            query.push("sort", &buf);
        }

        let req = Request::new(Route::GetBeatmapsetSearch).query(query);

        let fut =
            self.osu
                .inner
                .request(req)
                .map_ok(move |mut search_result: BeatmapsetSearchResult| {
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
