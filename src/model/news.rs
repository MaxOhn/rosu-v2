use serde::Deserialize;
use time::OffsetDateTime;

use crate::{prelude::Username, Osu, OsuResult};

use super::{serde_util, CacheUserFn, ContainedUsers};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct News {
    #[serde(
        default,
        rename = "cursor_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) cursor: Option<String>,
    #[serde(rename = "news_posts")]
    pub posts: Vec<NewsPost>,
    pub search: NewsSearch,
    #[serde(rename = "news_sidebar")]
    pub sidebar: NewsSidebar,
}

impl News {
    /// Returns whether there is a next page of news results,
    /// retrievable via [`get_next`](News::get_next).
    #[inline]
    pub const fn has_more(&self) -> bool {
        self.cursor.is_some()
    }

    /// If [`has_more`](News::has_more) is true, the API can provide the next set of news and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<News>> {
        Some(osu.news().cursor(self.cursor.as_deref()?).await)
    }
}

impl ContainedUsers for News {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NewsPost {
    #[serde(rename = "id")]
    pub post_id: u32,
    pub author: Username,
    /// Link to the file view on GitHub.
    pub edit_url: String,
    /// Link to the first image in the document.
    pub first_image: String,
    #[serde(with = "serde_util::datetime")]
    pub published_at: OffsetDateTime,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_util::option_datetime"
    )]
    pub updated_at: Option<OffsetDateTime>,
    /// Filename without the extension, used in URLs.
    pub slug: String,
    pub title: String,
    /// First paragraph of `content` with HTML markup stripped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

impl PartialEq for NewsPost {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.post_id == other.post_id && self.updated_at == other.updated_at
    }
}

impl Eq for NewsPost {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NewsSearch {
    #[serde(
        default,
        rename = "cursor_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) cursor: Option<Box<str>>,
    pub limit: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct NewsSidebar {
    pub current_year: u32,
    #[serde(rename = "news_posts")]
    pub posts: Vec<NewsPost>,
    pub years: Vec<u32>,
}
