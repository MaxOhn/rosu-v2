use super::{serde_, Cursor};

use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt;
use time::OffsetDateTime;

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
// TODO
// #[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct ForumPosts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    pub posts: Vec<ForumPost>,
    pub search: ForumPostsSearch,
    pub topic: ForumTopic,
}

impl ForumPosts {
    /// Checks whether the cursor field is `Some` which in turn
    /// can be used to retrieve the next set of posts.
    ///
    /// The next set can then be retrieved by providing this
    /// [`Cursor`] to [`GetForumPosts::cursor`](crate::request::GetForumPosts::cursor).
    /// Be sure all other parameters stay the same.
    #[inline]
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }
}

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct ForumPost {
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub created_at: OffsetDateTime,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub deleted_at: Option<OffsetDateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub edited_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_by_id: Option<u32>,
    pub forum_id: u32,
    /// Post content in HTML format
    pub html: String,
    #[serde(rename = "id")]
    pub post_id: u64,
    /// Post content in BBCode format
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

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a ForumPost struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        #[derive(Deserialize)]
        struct DateTimeWrapper(#[serde(with = "serde_::datetime")] OffsetDateTime);

        #[derive(Deserialize)]
        struct OptionDateTimeWrapper(
            #[serde(with = "serde_::option_datetime")] Option<OffsetDateTime>,
        );

        let mut created_at: Option<DateTimeWrapper> = None;
        let mut deleted_at: Option<OptionDateTimeWrapper> = None;
        let mut edited_at: Option<OptionDateTimeWrapper> = None;
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
                "created_at" => created_at = Some(map.next_value()?),
                "deleted_at" => deleted_at = Some(map.next_value()?),
                "edited_at" => edited_at = Some(map.next_value()?),
                "edited_by_id" => edited_by_id = map.next_value()?,
                "forum_id" => forum_id = Some(map.next_value()?),
                "html" => html = Some(map.next_value()?),
                "id" => post_id = Some(map.next_value()?),
                "raw" => raw = Some(map.next_value()?),
                "topic_id" => topic_id = Some(map.next_value()?),
                "user_id" => user_id = Some(map.next_value()?),
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        let DateTimeWrapper(created_at) =
            created_at.ok_or_else(|| Error::missing_field("created_at"))?;
        let forum_id = forum_id.ok_or_else(|| Error::missing_field("forum_id"))?;
        let html = html.ok_or_else(|| Error::missing_field("body or html"))?;
        let post_id = post_id.ok_or_else(|| Error::missing_field("id"))?;
        let raw = raw.ok_or_else(|| Error::missing_field("body or raw"))?;
        let topic_id = topic_id.ok_or_else(|| Error::missing_field("topic_id"))?;
        let user_id = user_id.ok_or_else(|| Error::missing_field("user_id"))?;

        Ok(ForumPost {
            created_at,
            deleted_at: deleted_at.and_then(|wrapper| wrapper.0),
            edited_at: edited_at.and_then(|wrapper| wrapper.0),
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
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(ForumPostVisitor)
    }
}

impl PartialEq for ForumPost {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.post_id == other.post_id && self.edited_at == other.edited_at
    }
}

impl Eq for ForumPost {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct ForumPostsSearch {
    pub limit: u32,
    pub sort: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct ForumTopic {
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub created_at: OffsetDateTime,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub deleted_at: Option<OffsetDateTime>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub updated_at: Option<OffsetDateTime>,
    pub user_id: u32,
}

impl PartialEq for ForumTopic {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.topic_id == other.topic_id && self.updated_at == other.updated_at
    }
}

impl Eq for ForumTopic {}
