use crate::model::user::UserCompact;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Details of a background
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SeasonalBackground {
    /// URL to the image
    pub url: String,
    /// [`UserCompact`](crate::model::user::UserCompact) of the artist of the art
    #[serde(rename = "user")]
    pub artist: UserCompact,
}

/// Collection of seasonal backgrounds
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SeasonalBackgrounds {
    /// End date of the backgrounds
    pub ends_at: DateTime<Utc>,
    /// List of backgrounds
    pub backgrounds: Vec<SeasonalBackground>,
}
