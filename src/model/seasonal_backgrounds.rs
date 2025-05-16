use crate::model::user::User;

use super::{serde_util, CacheUserFn, ContainedUsers};

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

impl ContainedUsers for SeasonalBackground {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.artist.apply_to_users(f);
    }
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

impl ContainedUsers for SeasonalBackgrounds {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.backgrounds.apply_to_users(f);
    }
}
