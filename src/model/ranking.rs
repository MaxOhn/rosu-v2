use crate::model::{Beatmapset, UserStatistics};

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Rankings {
    // cursor: Cursor // TODO
    pub mapsets: Option<Vec<Beatmapset>>,
    pub ranking: Vec<UserStatistics>,
    pub spotlight: Option<Spotlight>,
    pub total: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Spotlight {
    pub end_date: DateTime<Utc>,
    pub mode_specific: bool,
    pub name: String,
    pub participant_count: Option<u32>,
    #[serde(rename = "id")]
    pub spotlight_id: u32,
    pub spotlight_type: Option<String>,
    pub start_date: DateTime<Utc>,
}
