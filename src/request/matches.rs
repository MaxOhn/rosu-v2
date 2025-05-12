use crate::{
    model::matches::{MatchList, OsuMatch},
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

/// Get an [`OsuMatch`](crate::model::matches::OsuMatch) by its id
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetMatch<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, OsuMatch>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    match_id: u32,
    after: Option<u64>,
    before: Option<u64>,
    limit: Option<usize>,
}

impl<'a> GetMatch<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, match_id: u32) -> Self {
        Self {
            fut: None,
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

    fn start(&mut self) -> Pending<'a, OsuMatch> {
        let query = Query::encode(self);

        let route = Route::GetMatch {
            match_id: Some(self.match_id),
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;
        let fut = osu.request::<OsuMatch>(req);

        #[cfg(feature = "cache")]
        let fut = futures::TryFutureExt::inspect_ok(fut, move |osu_match| {
            for user in osu_match.users.values() {
                osu.update_cache(user.user_id, &user.username);
            }
        });

        Box::pin(fut)
    }
}

poll_req!(GetMatch => OsuMatch);

/// Get a [`MatchList`](crate::model::matches::MatchList) containing all
/// currently open multiplayer lobbies.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetMatches<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, MatchList>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(rename = "cursor_string")]
    cursor: Option<&'a str>,
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
    pub(crate) const fn cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(cursor);

        self
    }

    fn start(&mut self) -> Pending<'a, MatchList> {
        let query = Query::encode(self);
        let req = Request::with_query(Route::GetMatch { match_id: None }, query);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetMatches => MatchList);
