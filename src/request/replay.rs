use crate::{
    future::BytesWrap,
    model::{CacheUserFn, ContainedUsers, GameMode},
    routing::Route,
    Osu,
};

use super::Request;

/// Get a raw replay as a `Vec<u8>`
#[must_use = "requests must be configured and executed"]
pub struct GetReplayRaw<'a> {
    osu: &'a Osu,
    mode: Option<GameMode>,
    score_id: u64,
}

impl<'a> GetReplayRaw<'a> {
    pub(crate) const fn new(osu: &'a Osu, score_id: u64) -> Self {
        Self {
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
}

into_future! {
    |self: GetReplayRaw<'_>| -> BytesWrap {
        Request::new(Route::GetReplay {
            mode: self.mode,
            score_id: self.score_id,
        })
    } => |bytes, _| -> Vec<u8> {
        Ok(Vec::from(bytes.0))
    }
}

impl ContainedUsers for Vec<u8> {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}

/// Get a [`Replay`].
///
/// [`Replay`]: osu_db::Replay
#[cfg(feature = "replay")]
#[cfg_attr(docsrs, doc(cfg(feature = "replay")))]
#[must_use = "requests must be configured and executed"]
pub struct GetReplay<'a> {
    osu: &'a Osu,
    mode: Option<GameMode>,
    score_id: u64,
}

#[cfg(feature = "replay")]
impl<'a> GetReplay<'a> {
    pub(crate) const fn new(osu: &'a Osu, score_id: u64) -> Self {
        Self {
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
}

#[cfg(feature = "replay")]
into_future! {
    |self: GetReplay<'_>| -> BytesWrap {
        Request::new(Route::GetReplay {
            mode: self.mode,
            score_id: self.score_id,
        })
    } => |bytes, _| -> osu_db::Replay {
        osu_db::Replay::from_bytes(&bytes.0).map_err(crate::error::OsuError::from)
    }
}

#[cfg(feature = "replay")]
impl ContainedUsers for osu_db::Replay {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}
