use crate::{
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

/// TODO: Documentation
pub struct GetUserHighScore<'a> {
    fut: Option<Pending<'a, u32>>, // TODO
    osu: &'a Osu,
    room: u32,
    playlist: u32,
    user_id: u32,
}

impl<'a> GetUserHighScore<'a> {
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

    fn start(&mut self) {
        let req = Request::from(Route::GetUserHighScore {
            room: self.room,
            playlist: self.playlist,
            user_id: self.user_id,
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetUserHighScore<'_> => u32); // TODO

/// TODO: Documentation
pub struct GetScore<'a> {
    fut: Option<Pending<'a, u32>>, // TODO
    osu: &'a Osu,
    room: u32,
    playlist: u32,
    score_id: u32,
}

impl<'a> GetScore<'a> {
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

    fn start(&mut self) {
        let req = Request::from(Route::GetScore {
            room: self.room,
            playlist: self.playlist,
            score_id: self.score_id,
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetScore<'_> => u32); // TODO

/// TODO: Documentation
pub struct GetScores<'a> {
    fut: Option<Pending<'a, u32>>, // TODO
    osu: &'a Osu,
    room: u32,
    playlist: u32,
    limit: Option<u32>,
    sort: Option<&'static str>,
    cursor: Option<()>, // TODO
}

impl<'a> GetScores<'a> {
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

    fn start(&mut self) {
        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", limit.to_string());
        }

        if let Some(sort) = self.sort {
            query.push("sort", sort);
        }

        if let Some(_cursor) = self.cursor.take() {
            // TODO
        }

        let req = Request::from((
            query,
            Route::GetScores {
                room: self.room,
                playlist: self.playlist,
            },
        ));

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetScores<'_> => u32); // TODO
