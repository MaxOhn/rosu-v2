use crate::{
    model::chat::{
        ChannelType, ChatChannel, ChatChannelInfo, ChatChannelMessage, ChatNewPrivateChannel,
        ChatSilenceHistory, ChatUpdate, SilenceHistoryFilter,
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
pub struct GetChatChannel<'a> {
    osu: &'a Osu,
    channel_id: u32,
}

impl<'a> GetChatChannel<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: u32) -> Self {
        Self { osu, channel_id }
    }
}

into_future! {
    |self: GetChatChannel<'_>| -> ChatChannelInfo {
        Request::new(Route::GetChatChannel { channel_id: self.channel_id })
    }
}

/// List all public channels that can be joined.
#[must_use = "requests must be configured and executed"]
pub struct GetChatChannelList<'a> {
    osu: &'a Osu,
}

impl<'a> GetChatChannelList<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu }
    }
}

into_future! {
    |self: GetChatChannelList<'_>| -> Vec<ChatChannel> {
        Request::new(Route::GetChatChannelList)
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
pub struct GetChatUpdates<'a> {
    osu: &'a Osu,
}

impl<'a> GetChatUpdates<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu }
    }
}

into_future! {
    |self: GetChatUpdates<'_>| -> ChatUpdate {
        Request::new(Route::GetChatUpdates)
    }
}

/// Join a public or multiplayer channel.
#[must_use = "requests must be configured and executed"]
pub struct PutChatJoinChannel<'a> {
    osu: &'a Osu,
    channel_id: u32,
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
        Request::new(Route::PutChatJoinChannel { channel_id: self.channel_id, user_id: self.user_id })
    }
}

/// Leave a public or multiplayer channel.
#[must_use = "requests must be configured and executed"]
pub struct DeleteChatLeaveChannel<'a> {
    osu: &'a Osu,
    channel_id: u32,
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
        Request::new(Route::DeleteChatLeaveChannel { channel_id: self.channel_id, user_id: self.user_id })
    }
}

struct PostChannelCreateAnnouncementChannelBody {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Leave a public or multiplayer channel.
#[must_use = "requests must be configured and executed"]
pub struct PostChatCreateAnnouncement<'a> {
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

/// Create a private channel with another user (PM).
#[must_use = "requests must be configured and executed"]
pub struct PostChatCreatePM<'a> {
    osu: &'a Osu,
    target_id: Option<u32>,
    message: Option<String>,
    is_action: Option<bool>,
    uuid: Option<String>,
}

impl<'a> PostChatCreatePM<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            osu,
            target_id: None,
            message: None,
            is_action: None,
            uuid: None,
        }
    }

    pub fn target_id(mut self, target_id: u32) -> Self {
        self.target_id = Some(target_id);
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn is_action(mut self, is_action: bool) -> Self {
        self.is_action = Some(is_action);
        self
    }

    pub fn uuid(mut self, uuid: String) -> Self {
        self.uuid = Some(uuid);
        self
    }

}

into_future! {
    |self: PostChatCreatePM<'_>| -> ChatNewPrivateChannel {
        let mut body = JsonBody::new();

        if let Some(target_id) = self.target_id {
            body.push_int("target_id", target_id);
        }

        if let Some(message) = self.message {
            body.push_str("message", &message);
        }

        if let Some(is_action) = self.is_action {
            body.push_bool("is_action", is_action);
        }

        if let Some(uuid) = self.uuid {
            body.push_str("uuid", &uuid);
        }

        Request::with_body(Route::PostChatCreatePM, body)
    }
}

/// Mark a channel as read, up to a specific message.
#[must_use = "requests must be configured and executed"]
pub struct PutChatMarkChannelAsRead<'a> {
    osu: &'a Osu,
    channel_id: u32,
    message_id: u32,
}

impl<'a> PutChatMarkChannelAsRead<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: u32, message_id: u32) -> Self {
        Self {
            osu,
            channel_id,
            message_id,
        }
    }
}

into_future! {
    |self: PutChatMarkChannelAsRead<'_>| -> () {
        Request::new(Route::PutChatMarkChannelAsRead { channel_id: self.channel_id, message_id: self.message_id })
    }
}

/// Send a message to a chat channel.
#[must_use = "requests must be configured and executed"]
pub struct PostChatChannelMessage<'a> {
    osu: &'a Osu,
    channel_id: u32,
    message: Option<String>,
    is_action: Option<bool>,
}

impl<'a> PostChatChannelMessage<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: u32) -> Self {
        Self {
            osu,
            channel_id,
            message: None,
            is_action: None,
        }
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn is_action(mut self, is_action: bool) -> Self {
        self.is_action = Some(is_action);
        self
    }
}

into_future! {
    |self: PostChatChannelMessage<'_>| -> ChatChannelMessage {
        let mut body = JsonBody::new();

        if let Some(message) = self.message {
            body.push_str("message", &message);
        }

        if let Some(is_action) = self.is_action {
            body.push_bool("is_action", is_action);
        }

        Request::with_body(Route::PostChatChannelMessage { channel_id: self.channel_id}, body)
    }
}
