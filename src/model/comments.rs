use super::UserCompact;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct Comment {
    #[serde(rename = "id")]
    pub comment_id: u32,
    pub commentable_id: u32,
    pub commentable_type: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub edited_at: Option<DateTime<Utc>>,
    pub edited_by_id: Option<u32>,
    pub legacy_name: Option<String>,
    pub message: Option<String>,
    pub message_html: Option<String>,
    pub parent_id: Option<u32>,
    pub pinned: bool,
    pub replies_count: u32,
    pub updated_at: DateTime<Utc>,
    pub user_id: u32,
    pub votes_count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommentBundle {
    pub commentable_meta: Vec<CommentableMeta>,
    pub comments: Vec<Comment>,
    pub(crate) cursor: CommentBundleCursor,
    pub has_more: bool,
    pub has_more_id: Option<u32>,
    pub included_comments: Vec<Comment>,
    pub pinned_comments: Option<Vec<Comment>>,
    pub sort: CommentSort,
    pub top_level_count: Option<u32>,
    pub total: Option<u32>,
    pub user_follow: bool,
    pub user_votes: Vec<u32>,
    pub users: Vec<UserCompact>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct CommentBundleCursor {
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) id: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommentSort {
    New,
    Old,
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

#[derive(Clone, Debug, Deserialize)]
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
