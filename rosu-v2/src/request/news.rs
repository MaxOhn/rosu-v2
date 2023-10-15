use crate::{
    model::{news_::News, Cursor},
    request::{serialize::maybe_cursor, Pending, Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

/// Get a [`News`](crate::model::news::News) struct.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetNews<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, News>>,
    #[serde(skip)]
    osu: &'a Osu,
    news: Option<()>, // TODO
    #[serde(flatten, serialize_with = "maybe_cursor")]
    cursor: Option<Cursor>,
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
    //     self.news = Some(news);

    //     self
    // }

    #[inline]
    pub(crate) fn cursor(mut self, cursor: Cursor) -> Self {
        self.cursor = Some(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, News> {
        let query = Query::encode(self);
        let req = Request::with_query(Route::GetNews { news: self.news }, query);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetNews => News);
