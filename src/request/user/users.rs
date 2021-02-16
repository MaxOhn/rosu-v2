use crate::{
    model::UserCompact,
    request::{Pending, Request, UserId},
    routing::Route,
    Osu, OsuResult,
};

use reqwest::multipart::Form;

// TODO: Combine with GetUser(?)
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
        let mut form = Form::new();
        let user_ids = self.user_ids.take().unwrap();

        for user_id in user_ids.into_iter().take(50) {
            form = form.text("ids[]", user_id.to_string());
        }

        let req = Request::from((form, Route::GetUsers));
        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetUsers<'_>, Vec<UserCompact>);
