use serde::Serialize;

use crate::{
    model::event::{EventSort, Events},
    routing::Route,
    Osu,
};

use super::{Query, Request};

/// Get [`Events`].
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetEvents<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    sort: Option<EventSort>,
    #[serde(rename = "cursor_string")]
    cursor: Option<&'a str>,
}

impl<'a> GetEvents<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
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
}

into_future! {
    |self: GetEvents<'_>| -> Events {
        (
            Request::with_query(Route::GetEvents, Query::encode(&self)),
            self.sort,
        )
    } => |events, sort: Option<EventSort>| -> Events {
        events.sort = sort;

        Ok(events)
    }
}
