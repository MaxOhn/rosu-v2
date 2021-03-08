use crate::{
    model::wiki::WikiPage,
    request::{Pending, Request},
    routing::Route,
    Osu,
};

/// Get the wiki article or image data
pub struct GetWikiPage<'a> {
    fut: Option<Pending<'a, WikiPage>>, // TODO: Make this enum; either WikiPage or binary blob
    osu: &'a Osu,
    locale: Option<String>,
    page: Option<String>,
}

impl<'a> GetWikiPage<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, locale: impl Into<String>) -> Self {
        Self {
            fut: None,
            osu,
            locale: Some(locale.into()),
            page: None,
        }
    }

    #[inline]
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page.replace(page.into());

        self
    }

    fn start(&mut self) {
        let req = Request::from(Route::GetWikiPage {
            locale: self.locale.take().unwrap(),
            page: self.page.take(),
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

poll_req!(GetWikiPage<'_> => WikiPage);
