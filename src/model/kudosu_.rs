use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::user_::Username;

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub enum KudosuAction {
    #[serde(rename = "recalculate.reset")]
    RecalculateReset,
    #[serde(rename = "vote.give")]
    VoteGive,
    #[serde(rename = "vote.revoke")]
    VoteRevoke,
    #[serde(rename = "vote.reset")]
    VoteReset,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct KudosuGiver {
    pub url: String,
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::UsernameWrapper))]
    pub username: Username,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct KudosuHistory {
    pub id: u32,
    /// Either `give`, `reset`, or `revoke`.
    pub action: KudosuAction,
    pub amount: i32,
    // pub details: _; // TODO
    /// Object type which the exchange happened on (forum_post, etc).
    pub model: String,
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub created_at: DateTime<Utc>,
    /// Simple detail of the user who started the exchange.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub giver: Option<KudosuGiver>,
    /// Simple detail of the object for display.
    pub post: KudosuPost,
}

impl PartialEq for KudosuHistory {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for KudosuHistory {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct KudosuPost {
    /// Url of the object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Title of the object. It'll be "[deleted beatmap]" for deleted beatmaps.
    pub title: String,
}
