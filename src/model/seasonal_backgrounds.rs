use crate::model::user::UserCompact;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SeasonalBackground {
    pub url: String,
    #[serde(rename = "user")]
    pub artist: UserCompact,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SeasonalBackgrounds {
    pub ends_at: DateTime<Utc>,
    pub backgrounds: Vec<SeasonalBackground>,
}
