use crate::{
    model::matches::{MatchList, OsuMatch},
    request::{Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

/// Get an [`OsuMatch`].
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetMatch<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    match_id: u32,
    after: Option<u64>,
    before: Option<u64>,
    limit: Option<usize>,
}

impl<'a> GetMatch<'a> {
    pub(crate) const fn new(osu: &'a Osu, match_id: u32) -> Self {
        Self {
            osu,
            match_id,
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Get the match state containing only events after the given event id.
    ///
    /// Note: The given event id won't be included.
    #[inline]
    pub const fn after(mut self, after: u64) -> Self {
        self.after = Some(after);

        self
    }

    /// Get the match state containing only events before the given event id.
    ///
    /// Note: The given event id won't be included.
    #[inline]
    pub const fn before(mut self, before: u64) -> Self {
        self.before = Some(before);

        self
    }

    /// Get the match state after at most `limit` many new events.
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }
}

into_future! {
    |self: GetMatch<'_>| -> OsuMatch {
        let route = Route::GetMatch {
            match_id: Some(self.match_id),
        };

        Request::with_query(route, Query::encode(&self))
    }
}

/// Get a [`MatchList`] containing all currently open multiplayer lobbies.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetMatches<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(rename = "cursor_string")]
    cursor: Option<&'a str>,
}

impl<'a> GetMatches<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu, cursor: None }
    }

    #[inline]
    pub(crate) const fn cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(cursor);

        self
    }
}

into_future! {
    |self: GetMatches<'_>| -> MatchList {
        Request::with_query(
            Route::GetMatch { match_id: None },
            Query::encode(&self),
        )
    }
}
