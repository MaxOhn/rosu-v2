use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum KudosuAction {
    #[serde(rename = "vote.give")]
    Give,
    #[serde(rename = "vote.revoke")]
    Revoke,
    #[serde(rename = "vote.reset")]
    Reset,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct KudosuGiver {
    url: String,
    username: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KudosuHistory {
    id: u32,
    /// Either `give`, `reset`, or `revoke`.
    action: KudosuAction,
    amount: i32,
    /// Object type which the exchange happened on (forum_post, etc).
    model: String,
    created_at: DateTime<Utc>,
    /// Simple detail of the user who started the exchange.
    giver: Option<KudosuGiver>,
    /// Simple detail of the object for display.
    post: KudosuPost,
}

impl PartialEq for KudosuHistory {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct KudosuPost {
    /// Url of the object.
    url: Option<String>,
    /// Title of the object. It'll be "[deleted beatmap]" for deleted beatmaps.
    title: String,
}
