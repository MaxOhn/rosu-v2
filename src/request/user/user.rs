use crate::{
    model::{GameMode, User},
    request::{Pending, Request, UserId},
    routing::Route,
    Osu, OsuResult,
};

/// Get a user by their id.
pub struct GetUser<'a> {
    fut: Option<Pending<'a, User>>,
    osu: &'a Osu,
    user_id: Option<UserId>,
    mode: Option<GameMode>,
}

impl<'a> GetUser<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: impl Into<UserId>) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id.into()),
            mode: None,
        }
    }

    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetUser {
            user_id: self.user_id.take().unwrap(),
            mode: self.mode,
        });

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetUser<'_>, User);
