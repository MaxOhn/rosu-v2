use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WikiPage {
    layout: String,
    locale: String,
    markdown: String,
    path: String,
    subtitle: Option<String>,
    tags: Vec<String>,
    title: String,
}
