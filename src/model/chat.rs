use serde::Deserialize;
use time::OffsetDateTime;

use crate::prelude::{User, UserSilence};

use super::{serde_util, CacheUserFn, ContainedUsers};

pub type ChatChannelId = u64;
pub type ChatMessageId = u64;
pub type ChatSilenceId = u32;

/// Available filters for silence history received through the chat keepalive response
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum SilenceHistoryFilter {
    SinceSilenceId(ChatSilenceId),
    SinceMessageId(ChatMessageId),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChatSilenceHistory {
    pub silences: Vec<UserSilence>,
}

impl ContainedUsers for ChatSilenceHistory {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}

/// Combined information about a chat channel and its users online. The list of online users is
/// empty for all [`ChannelType`]s except PM.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChatChannelInfo {
    pub channel: ChatChannel,
    pub users: Vec<User>,
}

impl ContainedUsers for ChatChannelInfo {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.users.apply_to_users(f);
    }
}

/// Chat channel type. For permission checks for joining, see
/// [osu!web Documentation § ChannelType](https://osu.ppy.sh/docs/index.html#channeltype).
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ChannelType {
    #[serde(rename = "ANNOUNCE")]
    Announce,
    #[serde(rename = "GROUP")]
    Group,
    #[serde(rename = "MULTIPLAYER")]
    Multiplayer,
    PM,
    #[serde(rename = "PRIVATE")]
    Private,
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "SPECTATOR")]
    Spectator,
}

impl From<ChannelType> for &str {
    fn from(value: ChannelType) -> Self {
        match value {
            ChannelType::Announce => "ANNOUNCE",
            ChannelType::Group => "GROUP",
            ChannelType::Multiplayer => "MULTIPLAYER",
            ChannelType::PM => "PM",
            ChannelType::Private => "PRIVATE",
            ChannelType::Public => "PUBLIC",
            ChannelType::Spectator => "SPECTATOR",
        }
    }
}

/// User capabilities and properties related to a specific channel.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChatChannelUserAttributes {
    // User can send messages to the channel.
    pub can_message: bool,
    // User can see who is currently in the channel.
    pub can_list_users: Option<bool>,
    // The reason why messages cannot be sent to this channel.
    pub can_message_error: Option<String>,
    // `message_id` of the last read message.
    pub last_read_id: Option<u32>,
}

/// Chat channel information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChatChannel {
    #[serde(rename = "channel_id")]
    pub id: ChatChannelId,
    pub name: String,
    pub description: Option<String>,
    /// Path to the chat icon, relative to the website domain.
    pub icon: Option<String>,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub message_length_limit: u32,
    #[serde(rename = "moderated")]
    pub is_readonly: bool,
    /// Value from requests that is relayed back to the sender.
    pub uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_user_attributes: Option<ChatChannelUserAttributes>,
    /// `message_id` of last known message (only returned in presence responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<ChatMessageId>,
    /// Array of `user_id`s that are in the channel (not included for [`ChannelType::Public`] channels)
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<u32>>,
}

impl ContainedUsers for ChatChannel {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum ChannelMessageType {
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "plain")]
    Plain,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChatChannelMessage {
    #[serde(rename = "message_id")]
    pub id: ChatMessageId,
    pub channel_id: ChatChannelId,
    pub sender_id: u32,
    pub content: String,
    pub is_action: bool,
    #[serde(with = "serde_util::datetime")]
    pub timestamp: OffsetDateTime,
    #[serde(rename = "type")]
    pub message_type: ChannelMessageType,
    /// Message identifier originally sent by client.
    pub uuid: Option<String>,
    pub sender: Option<User>,
}

impl ContainedUsers for ChatChannelMessage {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.sender.apply_to_users(f);
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChatUpdate {
    pub presence: Vec<ChatChannel>,
    pub silences: Vec<UserSilence>,
}

impl ContainedUsers for ChatUpdate {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChatNewPrivateChannel {
    pub channel: ChatChannel,
    pub message: ChatChannelMessage,
}

impl ContainedUsers for ChatNewPrivateChannel {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.message.sender.apply_to_users(f);
    }
}
