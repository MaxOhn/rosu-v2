use serde::Deserialize;
use time::OffsetDateTime;

use super::serde_util;
use crate::model::ContainedUsers;

/// Changelog listing entry
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChangelogListing {
    /// List of all game update streams (stable, lazer, etc)
    pub streams: Vec<Stream>,
    /// List of builds included
    pub builds: Vec<Build>,
    /// Search query inputs
    pub search: Search,
}

impl ContainedUsers for ChangelogListing {
    fn apply_to_users(&self, _f: impl super::CacheUserFn) {}
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Stream {
    /// Build stream ID
    pub id: i64,
    /// Build stream title (stable40, lazer, etc)
    pub name: String,
    /// User-friendly build stream name
    pub display_name: String,
    /// Whether the build is displayed
    pub is_featured: bool,
    /// Latest deployed build information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_build: Option<Box<Build>>,
    /// Current live user count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Build {
    /// Release build ID
    pub id: i64,
    /// Release build version
    pub version: Option<String>,
    /// User-friendly release build version
    pub display_version: String,
    /// Current live user count for the build
    pub users: i64,
    /// Build release date
    #[serde(with = "serde_util::datetime")]
    pub created_at: OffsetDateTime,
    pub update_stream: Option<Stream>, // it is tagged as nullable but why would it be?
    pub changelog_entries: Option<Vec<ChangelogEntry>>,
    pub youtube_id: Option<String>,
    /// Previous and next versions to this build
    pub versions: Option<Versions>,
}

impl ContainedUsers for Build {
    fn apply_to_users(&self, _f: impl super::CacheUserFn) {}
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Versions {
    pub next: Option<Box<Build>>,
    pub previous: Option<Box<Build>>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChangelogEntry {
    pub id: Option<i64>,
    pub repository: Option<String>,
    pub github_pull_request_id: Option<i64>,
    pub github_url: Option<String>,
    /// Changelog entry URL in the news listing
    pub url: Option<String>,
    #[serde(rename = "type")]
    /// Changelog entry type
    pub entry_type: String, // TODO, technically defined but I can't read PHP
    /// Changelog category
    pub category: Option<String>, // TODO, technically defined but I can't read PHP
    /// Changelog entry title
    pub title: Option<String>,
    #[cfg_attr(
        feature = "serialize",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_html: Option<String>,
    pub major: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_util::option_datetime"
    )]
    pub created_at: Option<OffsetDateTime>,
    pub github_user: GithubUser,
}

/// Github user behind the specific change
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GithubUser {
    /// Display name of the user, may differ from github
    pub display_name: String,
    /// Github profile URL
    pub github_url: Option<String>,
    /// Github username
    pub github_username: Option<String>,
    pub id: Option<i64>,
    pub osu_username: Option<String>,
    pub user_id: Option<i64>,
    pub user_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Search {
    pub stream: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub max_id: Option<i64>,
    pub limit: i64,
}
