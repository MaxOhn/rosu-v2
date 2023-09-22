use super::{serde_, Cursor};
use crate::{prelude::Username, Osu, OsuResult};

use serde::Deserialize;

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
// TODO
// #[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct News {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cursor: Option<Cursor>,
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
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }

    /// If [`has_more`](News::has_more) is true, the API can provide the next set of news and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<News>> {
        Some(osu.news().cursor(self.cursor.clone()?).await)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct NewsPost {
    #[serde(rename = "id")]
    pub post_id: u32,
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::UsernameWrapper))]
    pub author: Username,
    /// Link to the file view on GitHub.
    pub edit_url: String,
    /// Link to the first image in the document.
    pub first_image: String,
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub published_at: OffsetDateTime,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
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
// TODO
// #[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct NewsSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) cursor: Option<Cursor>,
    pub limit: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct NewsSidebar {
    pub current_year: u32,
    #[serde(rename = "news_posts")]
    pub posts: Vec<NewsPost>,
    pub years: Vec<u32>,
}
