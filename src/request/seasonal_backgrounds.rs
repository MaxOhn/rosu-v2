use crate::{
    model::seasonal_backgrounds::SeasonalBackgrounds, request::Request, routing::Route, Osu,
};

/// Get [`SeasonalBackgrounds`].
#[must_use = "requests must be configured and executed"]
pub struct GetSeasonalBackgrounds<'a> {
    osu: &'a Osu,
}

impl<'a> GetSeasonalBackgrounds<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu }
    }
}

into_future! {
    |self: GetSeasonalBackgrounds<'_>| -> SeasonalBackgrounds {
        Request::new(Route::GetSeasonalBackgrounds)
    }
}
