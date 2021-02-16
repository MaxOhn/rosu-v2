use crate::model::{Beatmapset, UserStatistics};

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rankings {
    // cursor: Cursor // TODO
    mapsets: Option<Vec<Beatmapset>>,
    ranking: Vec<UserStatistics>,
    spotlight: Option<Spotlight>,
    total: u32,
}

#[derive(Debug, Deserialize)]
pub struct Spotlight {
    end_date: DateTime<Utc>,
    mode_specific: bool,
    name: String,
    participant_count: Option<u32>,
    #[serde(rename = "id")]
    spotlight_id: u32,
    spotlight_type: Option<String>,
    start_date: DateTime<Utc>,
}
