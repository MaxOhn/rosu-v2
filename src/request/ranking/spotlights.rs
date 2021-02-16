use crate::{
    model::Spotlight,
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

use serde::Deserialize;

// TODO: Test
/// Get the list of spotlights
pub struct GetSpotlights<'a> {
    fut: Option<Pending<'a, Spotlights>>,
    osu: &'a Osu,
}

impl<'a> GetSpotlights<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self { fut: None, osu }
    }

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetSpotlights);
        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

impl std::future::Future for GetSpotlights<'_> {
    type Output = crate::OsuResult<Vec<Spotlight>>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        loop {
            if let Some(fut) = self.as_mut().fut.as_mut() {
                return fut
                    .as_mut()
                    .poll(cx)
                    .map_ok(|spotlights| spotlights.spotlights);
            } else if let Err(why) = self.as_mut().start() {
                return std::task::Poll::Ready(Err(why));
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Spotlights {
    spotlights: Vec<Spotlight>,
}
