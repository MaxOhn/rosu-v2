use super::{user::UserCompact, Cursor};
use crate::{prelude::Username, request::GetUser, Osu, OsuResult};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents an single comment.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Comment {
    /// the ID of the comment
    #[serde(rename = "id")]
    pub comment_id: u32,
    /// ID of the object the comment is attached to
    pub commentable_id: u32,
    /// type of object the comment is attached to
    pub commentable_type: String,
    /// ISO 8601 date
    pub created_at: DateTime<Utc>,
    /// ISO 8601 date if the comment was deleted; `None`, otherwise
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    /// ISO 8601 date if the comment was edited; `None`, otherwise
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<DateTime<Utc>>,
    /// user id of the user that edited the post; `None`, otherwise
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_by_id: Option<u32>,
    /// username displayed on legacy comments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_name: Option<Username>,
    /// markdown of the comment's content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// html version of the comment's content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_html: Option<String>,
    /// ID of the comment's parent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u32>,
    /// Pin status of the comment
    pub pinned: bool,
    /// number of replies to the comment
    pub replies_count: u32,
    /// ISO 8601 date
    pub updated_at: DateTime<Utc>,
    /// user ID of the poster
    pub user_id: u32,
    /// number of votes
    pub votes_count: u32,
}

impl Comment {
    /// Request the [`User`](crate::model::user::User) of a comment
    #[inline]
    pub fn get_user<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        osu.user(self.user_id)
    }
}

impl PartialEq for Comment {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.comment_id == other.comment_id && self.user_id == other.user_id
    }
}

impl Eq for Comment {}

/// Comments and related data.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommentBundle {
    /// ID of the object the comment is attached to
    pub commentable_meta: Vec<CommentableMeta>,
    /// List of comments ordered according to `sort`
    pub comments: Vec<Comment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cursor: Option<Cursor>,
    /// If there are more comments or replies available
    pub(crate) has_more: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more_id: Option<u32>,
    /// Related comments; e.g. parent comments and nested replies
    pub included_comments: Vec<Comment>,
    /// Pinned comments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_comments: Option<Vec<Comment>>,
    /// order of comments
    pub sort: CommentSort,
    /// Number of comments at the top level. Not returned for replies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_level_count: Option<u32>,
    /// Total number of comments. Not retuned for replies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    /// is the current user watching the comment thread?
    pub user_follow: bool,
    /// IDs of the comments in the bundle the current user has upvoted
    pub user_votes: Vec<u32>,
    /// List of users related to the comments
    pub users: Vec<UserCompact>,
}

impl CommentBundle {
    /// Returns whether there is a next page of comments,
    /// retrievable via [`get_next`](CommentBundle::get_next).
    #[inline]
    pub fn has_more(&self) -> bool {
        self.has_more
    }

    /// If [`has_more`](CommentBundle::has_more) is true, the API can provide the next set of comments and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<CommentBundle>> {
        debug_assert!(self.has_more == self.cursor.is_some());

        Some(osu.comments().cursor(self.cursor.clone()?).await)
    }
}

/// Available orders for comments
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CommentSort {
    /// Sort by date, newest first
    #[serde(rename = "new")]
    New,
    /// Sort by date, oldest first
    #[serde(rename = "old")]
    Old,
    /// Sort by vote count
    #[serde(rename = "top")]
    Top,
}

impl fmt::Display for CommentSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sort = match self {
            Self::New => "new",
            Self::Old => "old",
            Self::Top => "top",
        };

        f.write_str(sort)
    }
}

/// Metadata of the object that a comment is attached to.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommentableMeta {
    Full {
        /// the ID of the object
        id: u32,
        /// the type of the object
        #[serde(rename = "type")]
        kind: String,
        owner_id: u32,
        owner_title: String,
        /// display title
        title: String,
        /// url of the object
        url: String,
    },
    Title {
        /// display title
        title: String,
    },
}
