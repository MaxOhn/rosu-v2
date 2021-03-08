use chrono::{DateTime, Utc};
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForumPosts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<ForumPostsCursor>,
    pub posts: Vec<ForumPost>,
    pub search: ForumPostsSearch,
    pub topic: ForumTopic,
}

impl ForumPosts {
    /// Checks whether the cursor field is `Some` which in turn
    /// can be used to retrieve the next set of posts.
    ///
    /// The next set can then be retrieved by providing this
    /// [`ForumPostsCursor`] to [`GetForumPosts::cursor`](crate::request::GetForumPosts::cursor).
    /// Be sure all other parameters stay the same.
    #[inline]
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ForumPost {
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_by_id: Option<u32>,
    pub forum_id: u32,
    pub html: String,
    #[serde(rename = "id")]
    pub post_id: u64,
    pub raw: String,
    pub topic_id: u64,
    pub user_id: u32,
}

struct ForumPostVisitor;

#[derive(Deserialize)]
struct ForumPostBody {
    html: String,
    raw: String,
}

impl<'de> Visitor<'de> for ForumPostVisitor {
    type Value = ForumPost;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a ForumPost struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut created_at = None;
        let mut deleted_at = None;
        let mut edited_at = None;
        let mut edited_by_id = None;
        let mut forum_id = None;
        let mut html = None;
        let mut post_id = None;
        let mut raw = None;
        let mut topic_id = None;
        let mut user_id = None;

        while let Some(key) = map.next_key()? {
            match key {
                "body" => {
                    let body: ForumPostBody = map.next_value()?;

                    html.replace(body.html);
                    raw.replace(body.raw);
                }
                "created_at" => {
                    created_at.replace(map.next_value()?);
                }
                "deleted_at" => deleted_at = map.next_value()?,
                "edited_at" => edited_at = map.next_value()?,
                "edited_by_id" => edited_by_id = map.next_value()?,
                "forum_id" => {
                    forum_id.replace(map.next_value()?);
                }
                "html" => {
                    html.replace(map.next_value()?);
                }
                "id" => {
                    post_id.replace(map.next_value()?);
                }
                "raw" => {
                    raw.replace(map.next_value()?);
                }
                "topic_id" => {
                    topic_id.replace(map.next_value()?);
                }
                "user_id" => {
                    user_id.replace(map.next_value()?);
                }
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        let created_at = created_at.ok_or_else(|| Error::missing_field("created_at"))?;
        let forum_id = forum_id.ok_or_else(|| Error::missing_field("forum_id"))?;
        let html = html.ok_or_else(|| Error::missing_field("body or html"))?;
        let post_id = post_id.ok_or_else(|| Error::missing_field("id"))?;
        let raw = raw.ok_or_else(|| Error::missing_field("body or raw"))?;
        let topic_id = topic_id.ok_or_else(|| Error::missing_field("topic_id"))?;
        let user_id = user_id.ok_or_else(|| Error::missing_field("user_id"))?;

        Ok(ForumPost {
            created_at,
            deleted_at,
            edited_at,
            edited_by_id,
            forum_id,
            html,
            post_id,
            raw,
            topic_id,
            user_id,
        })
    }
}

impl<'de> Deserialize<'de> for ForumPost {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(ForumPostVisitor)
    }
}

impl PartialEq for ForumPost {
    fn eq(&self, other: &Self) -> bool {
        self.post_id == other.post_id && self.edited_at == other.edited_at
    }
}

impl Eq for ForumPost {}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForumPostsCursor {
    #[serde(rename = "id")]
    pub post_id: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForumPostsSearch {
    pub limit: u32,
    pub sort: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForumTopic {
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    pub first_post_id: u64,
    pub forum_id: u32,
    pub is_locked: bool,
    #[serde(rename = "type")]
    pub kind: String, // TODO
    pub last_post_id: u64,
    pub post_count: u32,
    pub title: String,
    #[serde(rename = "id")]
    pub topic_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub user_id: u32,
}

impl PartialEq for ForumTopic {
    fn eq(&self, other: &Self) -> bool {
        self.topic_id == other.topic_id && self.updated_at == other.updated_at
    }
}

impl Eq for ForumTopic {}
