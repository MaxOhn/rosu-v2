use crate::{
    model::matches::{MatchList, MatchListCursor, OsuMatch},
    request::{Pending, Query, Request},
    routing::Route,
    Osu, OsuResult,
};

/// Get an [`OsuMatch`](crate::model::matches::OsuMatch) by its id
pub struct GetMatch<'a> {
    fut: Option<Pending<'a, OsuMatch>>,
    osu: &'a Osu,
    match_id: u32,
}

impl<'a> GetMatch<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, match_id: u32) -> Self {
        Self {
            fut: None,
            osu,
            match_id,
        }
    }

    fn start(&mut self) -> OsuResult<()> {
        let req = Request::from(Route::GetMatch {
            match_id: Some(self.match_id),
        });

        self.fut.replace(Box::pin(self.osu.inner.request(req)));

        Ok(())
    }
}

poll_req!(GetMatch<'_>, OsuMatch);

/// Get a [`MatchList`](crate::model::matches::MatchList) containing all
/// currently open multiplayer lobbies.
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

    fn start(&mut self) -> OsuResult<()> {
        let mut query = Query::new();

        if let Some(cursor) = self.cursor.take() {
            query.push("cursor[match_id]", cursor.match_id.to_string());
        }

        let req = Request::from((query, Route::GetMatch { match_id: None }));

        self.fut.replace(Box::pin(self.osu.inner.request(req)));

        Ok(())
    }
}

poll_req!(GetMatches<'_>, MatchList);
