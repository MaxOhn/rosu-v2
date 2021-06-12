use crate::{
    model::news::{News, NewsCursor},
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

/// Get a [`News`](crate::model::news::News) struct.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetNews<'a> {
    fut: Option<Pending<'a, News>>,
    osu: &'a Osu,
    news: Option<()>, // TODO
    cursor: Option<NewsCursor>,
}

impl<'a> GetNews<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            news: None,
            cursor: None,
        }
    }

    // TODO
    // #[inline]
    // pub fn news(mut self, news: ()) -> Self {
    //     self.news.replace(news);

    //     self
    // }

    #[inline]
    pub(crate) fn cursor(mut self, cursor: NewsCursor) -> Self {
        self.cursor.replace(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, News> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.news.inc();

        let mut query = Query::new();

        if let Some(cursor) = self.cursor {
            query
                .push("cursor[published_at]", &cursor.published_at) // TODO: Test
                .push("cursor[id]", &cursor.id);
        }

        let req = Request::new(Route::GetNews { news: self.news }).query(query);

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetNews => News);
