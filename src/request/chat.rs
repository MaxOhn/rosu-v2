use crate::{
    model::chat::{
        ChatChannel, ChatChannelInfo, ChatChannelMessage, ChatSilenceHistory, SilenceHistoryFilter,
    },
    request::{Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

/// Indicate that you are online, and get a list of recent silences.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct PostChatKeepalive<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    history_since: Option<u32>,
    since: Option<u32>,
}

impl<'a> PostChatKeepalive<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            osu,
            history_since: None,
            since: None,
        }
    }

    #[inline]
    pub const fn silence_history_since(mut self, f: SilenceHistoryFilter) -> Self {
        match f {
            SilenceHistoryFilter::SinceSilenceId(id) => self.history_since = Some(id),
            SilenceHistoryFilter::SinceMessageId(id) => self.since = Some(id),
        }
        self
    }
}

into_future! {
    |self: PostChatKeepalive<'_>| -> ChatSilenceHistory {
        Request::with_query(Route::PostChatKeepalive, Query::encode(&self))
    }
}

/// Read information about a specific channel.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetChatChannel<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    channel_id: u32,
}

impl<'a> GetChatChannel<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: u32) -> Self {
        Self { osu, channel_id }
    }
}

into_future! {
    |self: GetChatChannel<'_>| -> ChatChannelInfo {
        Request::with_query(Route::GetChatChannel { channel_id: self.channel_id }, Query::encode(&self))
    }
}

/// List all public channels that can be joined.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetChatChannelList<'a> {
    #[serde(skip)]
    osu: &'a Osu,
}

impl<'a> GetChatChannelList<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu }
    }
}

into_future! {
    |self: GetChatChannelList<'_>| -> Vec<ChatChannel> {
        Request::with_query(Route::GetChatChannelList, Query::encode(&self))
    }
}

/// Read recent messages from a chat channel.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetChatChannelMessages<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    channel_id: u32,
    limit: Option<u32>,
    since: Option<u32>,
    until: Option<u32>,
}

impl<'a> GetChatChannelMessages<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: u32) -> Self {
        Self {
            osu,
            channel_id,
            limit: None,
            since: None,
            until: None,
        }
    }

    #[inline]
    pub const fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// The message after which to return results (non-inclusive).
    #[inline]
    pub const fn since_message_id(mut self, message_id: u32) -> Self {
        self.since = Some(message_id);
        self
    }

    /// The message up to which to return results (inclusive).
    #[inline]
    pub const fn until_message_id(mut self, message_id: u32) -> Self {
        self.until = Some(message_id);
        self
    }
}

into_future! {
    |self: GetChatChannelMessages<'_>| -> Vec<ChatChannelMessage> {
        Request::with_query(Route::GetChatChannelMessages { channel_id: self.channel_id }, Query::encode(&self))
    }
}
