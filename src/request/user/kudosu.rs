use crate::{
    model::KudosuHistory,
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

use reqwest::multipart::Form;

/// Get a user's kudosu history by their user id.
pub struct GetUserKudosu<'a> {
    fut: Option<Pending<'a, Vec<KudosuHistory>>>,
    osu: &'a Osu,
    target_user: u32,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl<'a> GetUserKudosu<'a> {
    pub(crate) fn new(osu: &'a Osu, target_user: u32) -> Self {
        Self {
            fut: None,
            osu,
            target_user,
            limit: None,
            offset: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

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
            Route::GetUserKudosu {
                user_id: self.target_user,
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetUserKudosu<'_>, Vec<KudosuHistory>);
