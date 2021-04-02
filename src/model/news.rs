use crate::{Osu, OsuResult};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct News {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cursor: Option<NewsCursor>,
    #[serde(rename = "news_posts")]
    pub posts: Vec<NewsPost>,
    pub search: NewsSearch,
    #[serde(rename = "news_sidebar")]
    pub sidebar: NewsSidebar,
}

impl News {
    #[inline]
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }

    /// If `has_more()` is true, the API can provide the next set of news and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<News>> {
        Some(osu.news().cursor(self.cursor?).await)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct NewsCursor {
    pub(crate) published_at: DateTime<Utc>,
    pub(crate) id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewsPost {
    #[serde(rename = "id")]
    pub post_id: u32,
    pub author: String,
    pub edit_url: String,
    pub first_image: String,
    pub published_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub slug: String,
    pub title: String,
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

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NewsSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cursor: Option<NewsCursor>,
    pub limit: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NewsSidebar {
    pub current_year: u32,
    #[serde(rename = "news_posts")]
    pub posts: Vec<NewsPost>,
    pub years: Vec<u32>,
}
