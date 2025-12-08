use crate::{
    error::OsuError,
    future::{EmptyWrap, OsuFuture},
    model::chat::{
        ChannelType, ChatChannel, ChatChannelId, ChatChannelInfo, ChatChannelMessage,
        ChatMessageId, ChatNewPrivateChannel, ChatSilenceHistory, ChatSilenceId, ChatUpdate,
        SilenceHistoryFilter,
    },
    request::{Query, Request},
    routing::Route,
    Osu,
};

use bytes::{BufMut, BytesMut};
use serde::Serialize;

/// Indicate that you are online, and get a list of recent silences.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct PostChatKeepalive<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    history_since: Option<ChatSilenceId>,
    since: Option<ChatMessageId>,
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
    channel_id: ChatChannelId,
}

impl<'a> GetChatChannel<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: ChatChannelId) -> Self {
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
    channel_id: ChatChannelId,
    limit: Option<u32>,
    since: Option<ChatMessageId>,
    until: Option<ChatMessageId>,
}

impl<'a> GetChatChannelMessages<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: ChatChannelId) -> Self {
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
    pub const fn since_message_id(mut self, message_id: ChatMessageId) -> Self {
        self.since = Some(message_id);
        self
    }

    /// The message up to which to return results (inclusive).
    #[inline]
    pub const fn until_message_id(mut self, message_id: ChatMessageId) -> Self {
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
    channel_id: ChatChannelId,
    user_id: u32,
}

impl<'a> PutChatJoinChannel<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: ChatChannelId, user_id: u32) -> Self {
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
    channel_id: ChatChannelId,
    user_id: u32,
}

impl<'a> DeleteChatLeaveChannel<'a> {
    pub(crate) const fn new(osu: &'a Osu, channel_id: ChatChannelId, user_id: u32) -> Self {
        Self {
            osu,
            channel_id,
            user_id,
        }
    }
}

into_future! {
    |self: DeleteChatLeaveChannel<'_>| -> EmptyWrap {
        Request::new(Route::DeleteChatLeaveChannel { channel_id: self.channel_id, user_id: self.user_id })
    } => |_empty, _| -> () {
        Ok(())
    }
}

#[derive(Serialize)]
struct PostChannelCreateAnnouncementChannelBody {
    name: String,
    description: String,
}

/// Creates a new announcement channel.
#[derive(Serialize)]
#[must_use = "requests must be configured and executed"]
pub struct PostChatCreateAnnouncement<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    channel: PostChannelCreateAnnouncementChannelBody,
    message: String,
    #[serde(rename = "target_ids")]
    user_ids: Vec<u32>,
    #[serde(rename = "type")]
    kind: &'static str,
}

impl<'a> PostChatCreateAnnouncement<'a> {
    pub(crate) const fn new(
        osu: &'a Osu,
        name: String,
        description: String,
        message: String,
        user_ids: Vec<u32>,
    ) -> Self {
        Self {
            osu,
            channel: PostChannelCreateAnnouncementChannelBody { name, description },
            message,
            user_ids,
            kind: ChannelType::Announce.as_str(),
        }
    }
}

into_future! {
    |self: PostChatCreateAnnouncement<'_>| -> ChatChannel {
        let mut bytes = BytesMut::new();

        if let Err(err) = serde_json::to_writer((&mut bytes).writer(), &self) {
            return OsuFuture::from_error(OsuError::Serialize(err));
        }

        let body = bytes.freeze();

        Request::with_body(Route::PostChatCreateAnnouncement, body)
    }
}

/// Create a private channel with another user (PM).
#[derive(Serialize)]
#[must_use = "requests must be configured and executed"]
pub struct PostChatCreatePM<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(rename = "target_id")]
    user_id: u32,
    message: String,
    is_action: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
}

impl<'a> PostChatCreatePM<'a> {
    pub(crate) fn new(
        osu: &'a Osu,
        user_id: u32,
        message: impl Into<String>,
        is_action: bool,
    ) -> Self {
        Self {
            osu,
            user_id,
            message: message.into(),
            is_action,
            uuid: None,
        }
    }

    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());

        self
    }
}

into_future! {
    |self: PostChatCreatePM<'_>| -> ChatNewPrivateChannel {
        let mut bytes = BytesMut::new();

        if let Err(err) = serde_json::to_writer((&mut bytes).writer(), &self) {
            return OsuFuture::from_error(OsuError::Serialize(err));
        }

        let body = bytes.freeze();

        Request::with_body(Route::PostChatCreatePM, body)
    }
}

/// Mark a channel as read, up to a specific message.
#[must_use = "requests must be configured and executed"]
pub struct PutChatMarkChannelAsRead<'a> {
    osu: &'a Osu,
    channel_id: ChatChannelId,
    message_id: ChatMessageId,
}

impl<'a> PutChatMarkChannelAsRead<'a> {
    pub(crate) const fn new(
        osu: &'a Osu,
        channel_id: ChatChannelId,
        message_id: ChatMessageId,
    ) -> Self {
        Self {
            osu,
            channel_id,
            message_id,
        }
    }
}

into_future! {
    |self: PutChatMarkChannelAsRead<'_>| -> EmptyWrap {
        Request::new(Route::PutChatMarkChannelAsRead { channel_id: self.channel_id, message_id: self.message_id })
    } => |_empty, _| -> () {
        Ok(())
    }
}

/// Send a message to a chat channel.
#[derive(Serialize)]
#[must_use = "requests must be configured and executed"]
pub struct PostChatChannelMessage<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    channel_id: ChatChannelId,
    message: String,
    is_action: bool,
}

impl<'a> PostChatChannelMessage<'a> {
    pub(crate) fn new(
        osu: &'a Osu,
        channel_id: ChatChannelId,
        message: impl Into<String>,
        is_action: bool,
    ) -> Self {
        Self {
            osu,
            channel_id,
            message: message.into(),
            is_action,
        }
    }
}

into_future! {
    |self: PostChatChannelMessage<'_>| -> ChatChannelMessage {
        let mut bytes = BytesMut::new();

        if let Err(err) = serde_json::to_writer((&mut bytes).writer(), &self) {
            return OsuFuture::from_error(OsuError::Serialize(err));
        }

        let body = bytes.freeze();

        Request::with_body(Route::PostChatChannelMessage { channel_id: self.channel_id}, body)
    }
}
