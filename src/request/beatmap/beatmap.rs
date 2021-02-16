use crate::{
    model::Beatmap,
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

/// Get a beatmap by its id.
pub struct GetBeatmap<'a> {
    fut: Option<Pending<'a, Beatmap>>,
    osu: &'a Osu,
    map_id: u32,
}

impl<'a> GetBeatmap<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
        }
    }

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetBeatmap {
            map_id: self.map_id,
        });

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetBeatmap<'_>, Beatmap);
