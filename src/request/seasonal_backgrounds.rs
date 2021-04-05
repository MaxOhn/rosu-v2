use crate::{
    model::seasonal_backgrounds::SeasonalBackgrounds,
    request::{Pending, Request},
    routing::Route,
    Osu,
};

/// Get [`SeasonalBackgrounds`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetSeasonalBackgrounds<'a> {
    fut: Option<Pending<'a, SeasonalBackgrounds>>,
    osu: &'a Osu,
}

impl<'a> GetSeasonalBackgrounds<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self { fut: None, osu }
    }

    fn start(&mut self) -> Pending<'a, SeasonalBackgrounds> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.seasonal_backgrounds.inc();

        let req = Request::from(Route::GetSeasonalBackgrounds);

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetSeasonalBackgrounds => SeasonalBackgrounds);
