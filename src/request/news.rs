use crate::{
    model::news::News,
    request::{Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

/// Get a [`News`] struct.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetNews<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    news: Option<()>, // TODO
    #[serde(rename = "cursor_string")]
    cursor: Option<&'a str>,
}

impl<'a> GetNews<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            osu,
            news: None,
            cursor: None,
        }
    }

    #[inline]
    pub(crate) const fn cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(cursor);

        self
    }
}

into_future! {
    |self: GetNews<'_>| -> News {
        Request::with_query(
            Route::GetNews { news: self.news },
            Query::encode(&self),
        )
    }
}
