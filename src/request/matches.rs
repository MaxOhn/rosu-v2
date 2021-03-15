use crate::{
    model::matches::{MatchList, MatchListCursor, OsuMatch},
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

/// Get an [`OsuMatch`](crate::model::matches::OsuMatch) by its id
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetMatch<'a> {
    fut: Option<Pending<'a, OsuMatch>>,
    osu: &'a Osu,
    match_id: u32,
    after: Option<u64>,
    limit: Option<u32>,
}

impl<'a> GetMatch<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, match_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            match_id,
            after: None,
            limit: None,
        }
    }

    /// Get the match state containing only events after the given event id.
    ///
    /// Note: The given event id won't be included.
    #[inline]
    pub fn after(mut self, after: u64) -> Self {
        self.after.replace(after);

        self
    }

    /// Get the match state after at most `limit` many new events.
    #[inline]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Pending<'a, OsuMatch> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.osu_match.inc();

        let mut query = Query::new();

        if let Some(after) = self.after {
            query.push("after", after.to_string());
        }

        if let Some(limit) = self.limit {
            query.push("limit", limit.to_string());
        }

        let req = Request::from((
            query,
            Route::GetMatch {
                match_id: Some(self.match_id),
            },
        ));

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetMatch<'_> => OsuMatch);

/// Get a [`MatchList`](crate::model::matches::MatchList) containing all
/// currently open multiplayer lobbies.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetMatches<'a> {
    fut: Option<Pending<'a, MatchList>>,
    osu: &'a Osu,
    cursor: Option<MatchListCursor>,
}

impl<'a> GetMatches<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self {
            fut: None,
            osu,
            cursor: None,
        }
    }

    #[inline]
    pub(crate) fn cursor(mut self, cursor: MatchListCursor) -> Self {
        self.cursor.replace(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, MatchList> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.match_list.inc();

        let mut query = Query::new();

        if let Some(cursor) = self.cursor.take() {
            query.push("cursor[match_id]", cursor.match_id.to_string());
        }

        let req = Request::from((query, Route::GetMatch { match_id: None }));

        Box::pin(self.osu.inner.request(req))
    }
}

poll_req!(GetMatches<'_> => MatchList);
