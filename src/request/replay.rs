#![cfg(feature = "replay")]
use futures::FutureExt;
use osu_db::Replay;

use crate::{
    prelude::GameMode,
    request::{Pending, Request},
    routing::Route,
    Osu,
};

/// Get a [`Replay`](osu_db::Replay)
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetReplay<'a> {
    fut: Option<Pending<'a, Replay>>,
    osu: &'a Osu,
    mode: GameMode,
    score_id: u64,
}

impl<'a> GetReplay<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, mode: GameMode, score_id: u64) -> Self {
        Self {
            fut: None,
            osu,
            mode,
            score_id,
        }
    }

    fn start(&mut self) -> Pending<'a, Replay> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.replay.inc();

        let route = Route::GetReplay {
            mode: self.mode,
            score_id: self.score_id,
        };

        let fut = self.osu.request_raw(Request::new(route)).map(|res| {
            let bytes = res?;
            let replay = Replay::from_bytes(&bytes)?;

            Ok(replay)
        });

        Box::pin(fut)
    }
}

poll_req!(GetReplay => Replay);
