use crate::{
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

use reqwest::multipart::Form;

/// TODO: Documentation
pub struct GetScores<'a> {
    fut: Option<Pending<'a, u32>>, // TODO
    osu: &'a Osu,
    room: u32,
    playlist: u32,
    limit: Option<u32>,
    sort: Option<&'static str>,
    // TODO: Cursor
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

    fn start(&mut self) -> OsuResult<()> {
        let mut form = Form::new();

        if let Some(limit) = self.limit {
            form = form.text("limit", limit.to_string());
        }

        if let Some(sort) = self.sort {
            form = form.text("sort", sort);
        }

        // TODO: Add cursor to form

        let req = Request::from((
            form,
            Route::GetScores {
                room: self.room,
                playlist: self.playlist,
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetScores<'_>, u32); // TODO
