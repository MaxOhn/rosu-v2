use super::serde_util;
use crate::model::user::User;

use serde::Deserialize;
use time::OffsetDateTime;

/// Details of a background
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct SeasonalBackground {
    /// URL to the image
    pub url: String,
    /// [`User`] of the artist of the art
    #[serde(rename = "user")]
    pub artist: User,
}

/// Collection of seasonal backgrounds
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct SeasonalBackgrounds {
    /// End date of the backgrounds
    #[serde(with = "serde_util::datetime")]
    pub ends_at: OffsetDateTime,
    /// List of backgrounds
    pub backgrounds: Vec<SeasonalBackground>,
}
