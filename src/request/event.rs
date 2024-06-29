use futures::TryFutureExt;
use serde::Serialize;

use crate::{
    model::event::{EventSort, Events},
    routing::Route,
    Osu,
};

use super::{Pending, Query, Request};

/// Get a vec of [`Event`](crate::model::event::Event).
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetEvents<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Events>>,
    #[serde(skip)]
    osu: &'a Osu,
    sort: Option<EventSort>,
    #[serde(rename = "cursor_string")]
    cursor: Option<&'a str>,
}

impl<'a> GetEvents<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            sort: None,
            cursor: None,
        }
    }

    /// Sorting option
    #[inline]
    pub const fn sort(mut self, sort: EventSort) -> Self {
        self.sort = Some(sort);

        self
    }

    /// Cursor for pagination
    #[inline]
    pub const fn cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, Events> {
        let sort = self.sort;
        let query = Query::encode(self);
        let req = Request::with_query(Route::GetEvents, query);

        let fut = self.osu.request::<Events>(req).map_ok(move |mut events| {
            events.sort = sort;

            events
        });

        Box::pin(fut)
    }
}

poll_req!(GetEvents => Events);
