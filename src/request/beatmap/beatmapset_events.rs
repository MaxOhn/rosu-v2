use crate::{
    model::BeatmapsetEvents,
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

/// Get a [`BeatmapsetEvents`] struct.
pub struct GetBeatmapsetEvents<'a> {
    fut: Option<Pending<'a, BeatmapsetEvents>>,
    osu: &'a Osu,
}

impl<'a> GetBeatmapsetEvents<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self { fut: None, osu }
    }

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetBeatmapsetEvents);

        self.fut.replace(Box::pin(self.osu.inner.request(req)));

        Ok(())
    }
}

poll_req!(GetBeatmapsetEvents<'_>, BeatmapsetEvents);
