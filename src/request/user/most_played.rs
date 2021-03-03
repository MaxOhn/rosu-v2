use crate::{
    model::MostPlayedMap,
    request::{Pending, Request, UserId},
    routing::Route,
    Osu, OsuResult,
};

use reqwest::multipart::Form;

/// Get the most played beatmaps of a user by their id.
pub struct GetUserMostPlayed<'a> {
    fut: Option<Pending<'a, Vec<MostPlayedMap>>>,
    osu: &'a Osu,
    user_id: Option<UserId>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl<'a> GetUserMostPlayed<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: impl Into<UserId>) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id.into()),
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

    fn start(&mut self) -> OsuResult<()> {
        let mut form = Form::new();

        if let Some(limit) = self.limit {
            form = form.text("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            form = form.text("offset", offset.to_string());
        }

        let req = Request::from((
            form,
            Route::GetUserBeatmapsets {
                user_id: self.user_id.take().unwrap(),
                map_type: "most_played",
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetUserMostPlayed<'_>, Vec<MostPlayedMap>); // TODO
