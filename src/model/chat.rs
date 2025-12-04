use serde::Deserialize;

use crate::prelude::UserSilence;

use super::{CacheUserFn, ContainedUsers};

/// Available filters for silence history received through the chat keepalive response
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SilenceHistoryFilter {
    SinceSilenceId(u32),
    SinceMessageId(u32),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChatSilenceHistory {
    pub silences: Vec<UserSilence>,
}

impl ContainedUsers for ChatSilenceHistory {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}
