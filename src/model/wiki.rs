use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct WikiPage {
    pub layout: String,
    pub locale: String,
    pub markdown: String,
    pub path: String,
    pub subtitle: Option<String>,
    pub tags: Vec<String>,
    pub title: String,
}
