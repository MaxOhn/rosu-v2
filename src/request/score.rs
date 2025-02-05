use futures::TryFutureExt;
use rosu_mods::GameMode;

use crate::{prelude::Score, routing::Route, Osu};

use super::{Pending, Request};

/// Get a [`Score`] struct.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetScore<'a> {
    fut: Option<Pending<'a, Score>>,
    osu: &'a Osu,
    mode: Option<GameMode>,
    score_id: u64,
    legacy_scores: bool,
}

impl<'a> GetScore<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, score_id: u64) -> Self {
        Self {
            fut: None,
            osu,
            mode: None,
            score_id,
            legacy_scores: false,
        }
    }

    /// Specify the mode
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Specify whether the score should contain legacy data or not.
    ///
    /// Legacy data consists of a different grade calculation, less
    /// populated statistics, legacy mods, and a different score kind.
    #[inline]
    pub const fn legacy_scores(mut self, legacy_scores: bool) -> Self {
        self.legacy_scores = legacy_scores;

        self
    }

    fn start(&mut self) -> Pending<'a, Score> {
        let route = Route::GetScore {
            mode: self.mode,
            score_id: self.score_id,
        };

        let osu = self.osu;
        let mut req = Request::new(route);

        if self.legacy_scores {
            req.api_version(0);
        }

        let fut = osu.request::<Score>(req);

        #[cfg(feature = "cache")]
        let fut = fut.inspect_ok(move |score| {
            if let Some(ref user) = score.user {
                osu.update_cache(user.user_id, &user.username);
            }
        });

        Box::pin(fut)
    }
}

poll_req!(GetScore => Score);
