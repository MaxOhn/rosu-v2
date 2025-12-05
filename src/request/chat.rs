use crate::{
    model::chat::{
        ChannelType, ChatChannel, ChatChannelInfo, ChatChannelMessage, ChatSilenceHistory,
        ChatUpdate, SilenceHistoryFilter,
    },
    request::{JsonBody, Query, Request},
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

/// Read the list of channels the current user is in, as well as the list of silences that were
/// recently issued there.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetChatUpdates<'a> {
    #[serde(skip)]
    osu: &'a Osu,
}

impl<'a> GetChatUpdates<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu }
    }
}

into_future! {
    |self: GetChatUpdates<'_>| -> ChatUpdate {
        Request::with_query(Route::GetChatUpdates, Query::encode(&self))
    }
}

/// Join a public or multiplayer channel.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct PutChatJoinChannel<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    channel_id: u32,
    #[serde(skip)]
    user_id: u32,
}

impl<'a> PutChatJoinChannel<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: u32, user_id: u32) -> Self {
        Self {
            osu,
            channel_id,
            user_id,
        }
    }
}

into_future! {
    |self: PutChatJoinChannel<'_>| -> ChatChannel {
        Request::with_query(Route::PutChatJoinChannel { channel_id: self.channel_id, user_id: self.user_id }, Query::encode(&self))
    }
}

/// Leave a public or multiplayer channel.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct DeleteChatLeaveChannel<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    channel_id: u32,
    #[serde(skip)]
    user_id: u32,
}

impl<'a> DeleteChatLeaveChannel<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: u32, user_id: u32) -> Self {
        Self {
            osu,
            channel_id,
            user_id,
        }
    }
}

into_future! {
    |self: DeleteChatLeaveChannel<'_>| -> () {
        Request::with_query(Route::DeleteChatLeaveChannel { channel_id: self.channel_id, user_id: self.user_id }, Query::encode(&self))
    }
}

#[derive(Serialize)]
struct PostChannelCreateAnnouncementChannelBody {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Leave a public or multiplayer channel.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct PostChatCreateAnnouncement<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    channel: PostChannelCreateAnnouncementChannelBody,
    message: Option<String>,
    target_ids: Option<Vec<u32>>,
}

impl<'a> PostChatCreateAnnouncement<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            osu,
            channel: PostChannelCreateAnnouncementChannelBody {
                name: None,
                description: None,
            },
            message: None,
            target_ids: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.channel.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.channel.description = Some(description);
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<u32>) -> Self {
        self.target_ids = Some(user_ids);
        self
    }
}

into_future! {
    |self: PostChatCreateAnnouncement<'_>| -> ChatChannel {
        let mut body = JsonBody::new();

        body.push_key(b"channel");
        body.push_prefix();
        if let Some(name) = &self.channel.name {
            body.push_str("name", name);
        }
        if let Some(description) = &self.channel.description {
            body.push_str("description", description);
        }
        body.push_prefix();

        if let Some(message) = self.message {
            body.push_str("message", &message);
        }

        body.push_str("type", Into::<&str>::into(ChannelType::Announce));

        if let Some(target_ids) = self.target_ids {
            body.push_array("target_ids", &target_ids);
        }

        Request::with_body(Route::PostChatCreateAnnouncement, body)
    }
}

