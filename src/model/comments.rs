use super::UserCompact;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Comment {
    comment_id: u32,
    commentable_id: u32,
    commentable_type: String,
    created_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    edited_at: Option<DateTime<Utc>>,
    edited_by_id: Option<u32>,
    legacy_name: Option<String>,
    message: Option<String>,
    message_html: Option<String>,
    parent_id: Option<u32>,
    pinned: bool,
    replies_count: u32,
    updated_at: DateTime<Utc>,
    user_id: u32,
    votes_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct CommentBundle {
    commentable_meta: Vec<CommentableMeta>,
    comments: Vec<Comment>,
    has_more: bool,
    has_more_id: Option<u32>,
    include_comments: Vec<Comment>,
    pinned_comments: Option<Vec<Comment>>,
    sort: CommentSort,
    top_level_count: Option<u32>,
    total: Option<u32>,
    user_follow: bool,
    user_votes: Vec<u32>,
    users: Vec<UserCompact>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct CommentableMeta {
    id: u32,
    title: String,
    object_type: String,
    url: String,
}
