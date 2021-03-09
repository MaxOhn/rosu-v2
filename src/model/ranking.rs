use super::{beatmap::Beatmapset, user::UserStatistics};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Rankings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<RankingsCursor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapsets: Option<Vec<Beatmapset>>,
    pub ranking: Vec<UserStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spotlight: Option<Spotlight>,
    pub total: u32,
}

impl Rankings {
    /// Checks whether the cursor field is `Some` which in turn
    /// can be used to retrieve the next set of rankings.
    ///
    /// The next set can then be retrieved by providing this
    /// [`RankingsCursor`](crate::model::ranking::RankingsCursor) to
    /// [`GetRankings::cursor`](crate::request::GetRankings::cursor).
    /// Be sure all other parameters stay the same.
    #[inline]
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RankingsCursor {
    pub page: u32,
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