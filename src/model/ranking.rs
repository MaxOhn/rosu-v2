use crate::model::{Beatmapset, UserStatistics};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rankings {
    pub(crate) cursor: RankingsCursor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapsets: Option<Vec<Beatmapset>>,
    pub ranking: Vec<UserStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spotlight: Option<Spotlight>,
    pub total: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct RankingsCursor {
    page: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Spotlight {
    pub end_date: DateTime<Utc>,
    pub mode_specific: bool,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<u32>,
    #[serde(rename = "id")]
    pub spotlight_id: u32,
    #[serde(rename = "type")]
    pub spotlight_type: String,
    pub start_date: DateTime<Utc>,
}

impl PartialEq for Spotlight {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.spotlight_id == other.spotlight_id
            && self.start_date == other.start_date
            && self.end_date == other.end_date
    }
}

impl Eq for Spotlight {}
