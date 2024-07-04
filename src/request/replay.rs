use futures::TryFutureExt;

#[cfg(feature = "replay")]
use futures::FutureExt;

#[cfg(feature = "replay")]
use osu_db::Replay;

use crate::{
    prelude::GameMode,
    request::{Pending, Request},
    routing::Route,
    Osu,
};

/// Get a raw replay in form of a `Vec<u8>`
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetReplayRaw<'a> {
    fut: Option<Pending<'a, Vec<u8>>>,
    osu: &'a Osu,
    mode: Option<GameMode>,
    score_id: u64,
}

impl<'a> GetReplayRaw<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, score_id: u64) -> Self {
        Self {
            fut: None,
            osu,
            mode: None,
            score_id,
        }
    }

    /// Specify the mode
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    fn start(&mut self) -> Pending<'a, Vec<u8>> {
        let route = Route::GetReplay {
            mode: self.mode,
            score_id: self.score_id,
        };

        let fut = self.osu.request_raw(Request::new(route)).map_ok(Vec::from);

        Box::pin(fut)
    }
}

poll_req!(GetReplayRaw => Vec<u8>);

/// Get a [`Replay`](osu_db::Replay)
#[cfg(feature = "replay")]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetReplay<'a> {
    fut: Option<Pending<'a, Replay>>,
    inner: Option<GetReplayRaw<'a>>,
}

#[cfg(feature = "replay")]
impl<'a> GetReplay<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, score_id: u64) -> Self {
        Self {
            fut: None,
            inner: Some(GetReplayRaw::new(osu, score_id)),
        }
    }

    /// Specify the mode
    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.inner = self.inner.map(|raw| raw.mode(mode));

        self
    }

    fn start(&mut self) -> Pending<'a, Replay> {
        let fut = self.inner.take().unwrap().map(|res| {
            res.and_then(|bytes| Replay::from_bytes(&bytes).map_err(crate::error::OsuError::from))
        });

        Box::pin(fut)
    }
}

#[cfg(feature = "replay")]
poll_req!(GetReplay => Replay);
