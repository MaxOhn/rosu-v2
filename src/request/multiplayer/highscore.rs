use crate::{
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
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

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetUserHighScore {
            room: self.room,
            playlist: self.playlist,
            user_id: self.user_id,
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));

        Ok(())
    }
}

poll_req!(GetUserHighScore<'_>, u32); // TODO
