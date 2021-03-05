use crate::{
    model::OsuMatch,
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

/// Get the wiki article or image data
pub struct GetMatch<'a> {
    fut: Option<Pending<'a, OsuMatch>>,
    osu: &'a Osu,
    match_id: u32,
}

impl<'a> GetMatch<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, match_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            match_id,
        }
    }

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetMatch {
            match_id: self.match_id,
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));

        Ok(())
    }
}

poll_req!(GetMatch<'_>, OsuMatch);
