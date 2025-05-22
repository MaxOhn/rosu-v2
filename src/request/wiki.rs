use crate::{model::wiki::WikiPage, request::Request, routing::Route, Osu};

/// Get a [`WikiPage`] or image data.
#[must_use = "requests must be configured and executed"]
#[derive(Clone)]
pub struct GetWikiPage<'a> {
    osu: &'a Osu,
    locale: Box<str>,
    page: Option<Box<str>>,
}

impl<'a> GetWikiPage<'a> {
    pub(crate) fn new(osu: &'a Osu, locale: impl Into<String>) -> Self {
        Self {
            osu,
            locale: Box::from(locale.into()),
            page: None,
        }
    }

    /// Specify the page
    #[inline]
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.page = Some(Box::from(page.into()));

        self
    }
}

into_future! {
    |self: GetWikiPage<'_>| -> WikiPage {
        Request::new(Route::GetWikiPage {
            locale: self.locale,
            page: self.page,
        })
    }
}
