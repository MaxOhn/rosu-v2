use serde::{Deserialize, Serialize};

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Represents a wiki article
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct WikiPage {
    /// All available locales for the article
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_locales: Vec<String>,
    /// The layout type for the page
    pub layout: String,
    /// All lowercase BCP 47 language tag
    pub locale: String,
    /// Markdown content
    pub markdown: String,
    /// Path of the article
    pub path: String,
    /// The article's subtitle
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Associated tags for the article
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// The article's title
    pub title: String,
}
