use crate::{
    model::KudosuHistory,
    request::{Pending, Query, Request, UserId},
    routing::Route,
    Osu, OsuResult,
};

/// Get a user's kudosu history by their user id.
pub struct GetUserKudosu<'a> {
    fut: Option<Pending<'a, Vec<KudosuHistory>>>,
    osu: &'a Osu,
    user_id: Option<UserId>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl<'a> GetUserKudosu<'a> {
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
        let mut query = Query::new();

        if let Some(limit) = self.limit {
            query.push("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            query.push("offset", offset.to_string());
        }

        let req = Request::from((
            query,
            Route::GetUserKudosu {
                user_id: self.user_id.take().unwrap(),
            },
        ));

        self.fut.replace(Box::pin(self.osu.inner.request(req)));

        Ok(())
    }
}

poll_req!(GetUserKudosu<'_>, Vec<KudosuHistory>);
