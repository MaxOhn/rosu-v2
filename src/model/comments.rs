use super::UserCompact;
use crate::{request::GetUser, Osu, OsuResult};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Comment {
    #[serde(rename = "id")]
    pub comment_id: u32,
    pub commentable_id: u32,
    pub commentable_type: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_by_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u32>,
    pub pinned: bool,
    pub replies_count: u32,
    pub updated_at: DateTime<Utc>,
    pub user_id: u32,
    pub votes_count: u32,
}

impl Comment {
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommentBundle {
    pub commentable_meta: Vec<CommentableMeta>,
    pub comments: Vec<Comment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cursor: Option<CommentBundleCursor>,
    pub has_more: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more_id: Option<u32>,
    pub included_comments: Vec<Comment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_comments: Option<Vec<Comment>>,
    pub sort: CommentSort,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_level_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    pub user_follow: bool,
    pub user_votes: Vec<u32>,
    pub users: Vec<UserCompact>,
}

impl CommentBundle {
    /// If `has_more` is true, the API can provide the next set of comments and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<CommentBundle>> {
        debug_assert!(self.has_more == self.cursor.is_some());

        Some(osu.comments().cursor(self.cursor?).await)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct CommentBundleCursor {
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) id: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CommentSort {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "old")]
    Old,
    #[serde(rename = "top")]
    Top,
}

impl fmt::Display for CommentSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sort = match self {
            Self::New => "new",
            Self::Old => "old",
            Self::Top => "top",
        };

        f.write_str(sort)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommentableMeta {
    Full {
        id: u32,
        #[serde(rename = "type")]
        kind: String,
        owner_id: u32,
        owner_title: String,
        title: String,
        url: String,
    },
    Title {
        title: String,
    },
}
