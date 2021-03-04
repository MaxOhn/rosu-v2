use crate::{
    model::UserCompact,
    request::{Pending, Query, Request, UserId},
    routing::Route,
    Osu, OsuResult,
};

/// Get multiple users by their ids.
pub struct GetUsers<'a> {
    fut: Option<Pending<'a, Vec<UserCompact>>>,
    osu: &'a Osu,
    user_ids: Option<Vec<UserId>>,
}

impl<'a> GetUsers<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_ids: Vec<UserId>) -> Self {
        Self {
            fut: None,
            osu,
            user_ids: Some(user_ids),
        }
    }

    fn start(&mut self) -> OsuResult<()> {
        let mut query = Query::new();

        // * user_ids is capped to 50 elements in `Osu::users`
        let user_ids = self.user_ids.take().unwrap();

        let iter = user_ids
            .into_iter()
            .map(|user_id| ("id[]", user_id.to_string()));

        query.extend(iter);

        let req = Request::from((query, Route::GetUsers));
        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetUsers<'_>, Vec<UserCompact>);
