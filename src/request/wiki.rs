use crate::{
    model::WikiPage,
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

/// Get the wiki article or image data
pub struct GetWikiPage<'a> {
    fut: Option<Pending<'a, WikiPage>>, // TODO: Make this enum; either WikiPage or binary blob
    osu: &'a Osu,
    page: Option<String>,
}

impl<'a> GetWikiPage<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            page: None,
        }
    }

    #[inline]
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page.replace(page.into());

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetWikiPage {
            page: self.page.take(),
        });

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetWikiPage<'_>, WikiPage);
