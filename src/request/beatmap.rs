use crate::{
    model::{
        beatmap::{Beatmap, Beatmapset, BeatmapsetEvents},
        score::{BeatmapScores, BeatmapUserScore},
        GameMode, GameMods,
    },
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

#[cfg(feature = "cache")]
use super::UserId;

#[cfg(feature = "cache")]
use futures::future::TryFutureExt;

/// Get a [`Beatmap`](crate::model::beatmap::Beatmap).
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

    fn start(&mut self) {
        let mut query = Query::new();

        if let Some(checksum) = self.checksum.take() {
            query.push("checksum", checksum);
        }

        if let Some(filename) = self.filename.take() {
            query.push("filename", filename);
        }

        if let Some(map_id) = self.map_id {
            query.push("id", map_id.to_string());
        }

        let req = Request::from((query, Route::GetBeatmap));

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetBeatmap<'_> => Beatmap);

/// Get top scores of a beatmap by its id.
pub struct GetBeatmapScores<'a> {
    fut: Option<Pending<'a, BeatmapScores>>,
    osu: &'a Osu,
    map_id: u32,
    score_type: Option<&'static str>,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
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

    fn start(&mut self) {
        let mut query = Query::new();

        if let Some(mode) = self.mode {
            query.push("mode", mode.to_string());
        }

        if let Some(_mods) = self.mods {
            // TODO
        }

        if let Some(_score_type) = self.score_type {
            // TODO
        }

        let req = Request::from((
            query,
            Route::GetBeatmapScores {
                map_id: self.map_id,
            },
        ));

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetBeatmapScores<'_> => BeatmapScores);

/// Get scores of a user on a beatmap by the user's and the map's id.
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

    fn start(&mut self) {
        let mut query = Query::new();

        if let Some(mode) = self.mode {
            query.push("mode", mode.to_string());
        }

        if let Some(_mods) = self.mods {
            // TODO
        }

        #[cfg(not(feature = "cache"))]
        {
            let req = Request::from((
                query,
                Route::GetBeatmapUserScore {
                    user_id: self.user_id,
                    map_id: self.map_id,
                },
            ));

            self.fut.replace(Box::pin(self.osu.inner.request(req)));
        }

        #[cfg(feature = "cache")]
        {
            let map_id = self.map_id;
            let osu = &self.osu.inner;

            let fut = self
                .osu
                .cache_user(self.user_id.take().unwrap())
                .map_ok(move |user_id| {
                    Request::from((query, Route::GetBeatmapUserScore { user_id, map_id }))
                })
                .and_then(move |req| osu.request(req));

            self.fut.replace(Box::pin(fut));
        }
    }
}

poll_req!(GetBeatmapUserScore<'_> => BeatmapUserScore);

/// Get a [`BeatmapsetEvents`] struct.
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

    fn start(&mut self) {
        let req = Request::from(Route::GetBeatmapset {
            mapset_id: self.mapset_id,
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetBeatmapset<'_> => Beatmapset);

/// Get a [`BeatmapsetEvents`] struct.
pub struct GetBeatmapsetEvents<'a> {
    fut: Option<Pending<'a, BeatmapsetEvents>>,
    osu: &'a Osu,
}

impl<'a> GetBeatmapsetEvents<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self { fut: None, osu }
    }

    fn start(&mut self) {
        let req = Request::from(Route::GetBeatmapsetEvents);

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetBeatmapsetEvents<'_> => BeatmapsetEvents);
