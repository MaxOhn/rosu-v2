#[allow(unused_imports)]
use crate::{
    error::OsuError,
    model::Cursor,
    request::{Pending, Query, Request},
    Osu,
};

// TODO: Documentation
#[allow(dead_code)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetMultiplayerScore<'a> {
    fut: Option<Pending<'a, ()>>, // TODO
    osu: &'a Osu,
    room: u32,
    playlist: u32,
    score_id: u32,
}

impl<'a> GetMultiplayerScore<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, room: u32, playlist: u32, score_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            room,
            playlist,
            score_id,
        }
    }

    fn start(&mut self) -> Pending<'a, ()> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.multiplayer_score.inc();

        Box::pin(async { Err(OsuError::UnavailableEndpoint) })

        // let req = Request::from(Route::GetMultiplayerScore {
        //     room: self.room,
        //     playlist: self.playlist,
        //     score_id: self.score_id,
        // });

        // Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetMultiplayerScore => ()); // TODO

// TODO: Documentation; test with room=70651 | playlist=153023
#[allow(dead_code)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetMultiplayerScores<'a> {
    fut: Option<Pending<'a, ()>>, // TODO
    osu: &'a Osu,
    room: u32,
    playlist: u32,
    limit: Option<u32>,
    sort: Option<&'static str>,
    cursor: Option<Cursor>,
}

impl<'a> GetMultiplayerScores<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, room: u32, playlist: u32) -> Self {
        Self {
            fut: None,
            osu,
            room,
            playlist,
            limit: None,
            sort: None,
            cursor: None,
        }
    }

    #[inline]
    pub fn sort_ascending(mut self) -> Self {
        self.sort.replace("sort_asc");

        self
    }

    #[inline]
    pub fn sort_descending(mut self) -> Self {
        self.sort.replace("sort_desc");

        self
    }

    fn start(&mut self) -> Pending<'a, ()> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.multiplayer_scores.inc();

        Box::pin(async { Err(OsuError::UnavailableEndpoint) })

        // let mut query = Query::new();

        // if let Some(limit) = self.limit {
        //     query.push("limit", limit.to_string());
        // }

        // if let Some(sort) = self.sort {
        //     query.push("sort", sort);
        // }

        // if let Some(_cursor) = self.cursor.take() {
        //     // TODO
        // }

        // let req = Request::from((
        //     query,
        //     Route::GetMultiplayerScores {
        //         room: self.room,
        //         playlist: self.playlist,
        //     },
        // ));

        // Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetMultiplayerScores => ()); // TODO

// TODO: Documentation
#[allow(dead_code)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetMultiplayerUserHighScore<'a> {
    fut: Option<Pending<'a, ()>>, // TODO
    osu: &'a Osu,
    room: u32,
    playlist: u32,
    user_id: u32,
}

impl<'a> GetMultiplayerUserHighScore<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, room: u32, playlist: u32, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            room,
            playlist,
            user_id,
        }
    }

    fn start(&mut self) -> Pending<'a, ()> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.multiplayer_user_highscore.inc();

        Box::pin(async { Err(OsuError::UnavailableEndpoint) })

        // let req = Request::from(Route::GetMultiplayerUserHighScore {
        //     room: self.room,
        //     playlist: self.playlist,
        //     user_id: self.user_id,
        // });

        // Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetMultiplayerUserHighScore => ()); // TODO
