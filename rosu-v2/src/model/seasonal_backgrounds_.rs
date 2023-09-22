use super::serde_;
use crate::model::user_::User;

use serde::Deserialize;
use time::OffsetDateTime;

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Details of a background
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct SeasonalBackground {
    /// URL to the image
    pub url: String,
    /// [`User`](crate::model::user::User) of the artist of the art
    #[serde(rename = "user")]
    pub artist: User,
}

/// Collection of seasonal backgrounds
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct SeasonalBackgrounds {
    /// End date of the backgrounds
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub ends_at: OffsetDateTime,
    /// List of backgrounds
    pub backgrounds: Vec<SeasonalBackground>,
}
