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
        let req = Request::new(Route::GetSeasonalBackgrounds);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetSeasonalBackgrounds => SeasonalBackgrounds);
