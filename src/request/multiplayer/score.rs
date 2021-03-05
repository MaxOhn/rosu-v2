use crate::{
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

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

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetScore {
            room: self.room,
            playlist: self.playlist,
            score_id: self.score_id,
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));

        Ok(())
    }
}

poll_req!(GetScore<'_>, u32); // TODO
