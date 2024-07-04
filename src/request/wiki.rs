use crate::{
    model::wiki::WikiPage,
    request::{Pending, Request},
    routing::Route,
    Osu,
};

/// Get a [`WikiPage`](crate::model::wiki::WikiPage) or image data.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetWikiPage<'a> {
    fut: Option<Pending<'a, WikiPage>>, // TODO: Make this enum; either WikiPage or binary blob
    osu: &'a Osu,
    locale: Option<Box<str>>,
    page: Option<Box<str>>,
}

impl<'a> GetWikiPage<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, locale: impl Into<String>) -> Self {
        Self {
            fut: None,
            osu,
            locale: Some(Box::from(locale.into())),
            page: None,
        }
    }

    /// Specify the page
    #[inline]
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(Box::from(page.into()));

        self
    }

    fn start(&mut self) -> Pending<'a, WikiPage> {
        let req = Request::new(Route::GetWikiPage {
            locale: self.locale.take().unwrap(),
            page: self.page.take(),
        });

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetWikiPage => WikiPage);
