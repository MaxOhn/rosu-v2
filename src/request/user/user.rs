use crate::{
    model::{GameMode, User},
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

/// Get a user by their id.
pub struct GetUser<'a> {
    fut: Option<Pending<'a, User>>,
    osu: &'a Osu,
    user_id: u32,
    mode: Option<GameMode>,
}

impl<'a> GetUser<'a> {
    pub(crate) fn new(osu: &'a Osu, user_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            user_id,
            mode: None,
        }
    }

    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetUser {
            user_id: self.user_id,
            mode: self.mode,
        });

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetUser<'_>, User);
