use super::serde_;
use crate::model::user_::UserCompact;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Details of a background
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct SeasonalBackground {
    /// URL to the image
    pub url: String,
    /// [`UserCompact`](crate::model::user::UserCompact) of the artist of the art
    #[serde(rename = "user")]
    pub artist: UserCompact,
}

/// Collection of seasonal backgrounds
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct SeasonalBackgrounds {
    /// End date of the backgrounds
    #[serde(with = "serde_::datetime_full")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub ends_at: OffsetDateTime,
    /// List of backgrounds
    pub backgrounds: Vec<SeasonalBackground>,
}
