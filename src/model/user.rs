use super::{serde_util, GameMode};

use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use smallstr::SmallString;
use std::fmt;
use time::{Date, OffsetDateTime};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct AccountHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>, // TODO: Can be removed?
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub history_type: HistoryType,
    #[serde(with = "serde_util::datetime")]
    pub timestamp: OffsetDateTime,
    #[serde(rename = "length")]
    pub seconds: u32,
    pub permanent: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Badge {
    #[serde(with = "serde_util::datetime")]
    pub awarded_at: OffsetDateTime,
    pub description: String,
    pub image_url: String,
    pub url: String,
}

/// Country codes are at most 2 ASCII characters long
pub type CountryCode = SmallString<[u8; 2]>;

struct CountryVisitor;

impl<'de> Visitor<'de> for CountryVisitor {
    type Value = String;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a string or a map containing a `name` field")
    }

    #[inline]
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(v.to_owned())
    }

    #[inline]
    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut country = None;

        while let Some(key) = map.next_key()? {
            match key {
                "name" => country = Some(map.next_value()?),
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        country.ok_or_else(|| Error::missing_field("name"))
    }
}

struct OptionCountryVisitor;

impl<'de> Visitor<'de> for OptionCountryVisitor {
    type Value = Option<String>;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a string, a map containing a `name` field, or null")
    }

    #[inline]
    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(CountryVisitor).map(Some)
    }

    #[inline]
    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        self.visit_unit()
    }

    #[inline]
    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

pub(crate) fn deserialize_country<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    d.deserialize_any(CountryVisitor)
}

pub(crate) fn deserialize_maybe_country<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_option(OptionCountryVisitor)
}

/// Counts of grades of a [`UserExtended`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct GradeCounts {
    /// Number of SS ranked scores
    #[serde(deserialize_with = "deserialize_i32_default")]
    pub ss: i32,
    /// Number of Silver SS ranked scores
    #[serde(deserialize_with = "deserialize_i32_default")]
    pub ssh: i32,
    /// Number of S ranked scores
    #[serde(deserialize_with = "deserialize_i32_default")]
    pub s: i32,
    /// Number of Silver S ranked scores
    #[serde(deserialize_with = "deserialize_i32_default")]
    pub sh: i32,
    /// Number of A ranked scores
    #[serde(deserialize_with = "deserialize_i32_default")]
    pub a: i32,
}

#[inline]
fn deserialize_i32_default<'de, D: Deserializer<'de>>(d: D) -> Result<i32, D::Error> {
    <Option<i32> as Deserialize>::deserialize(d).map(Option::unwrap_or_default)
}

/// Describes a Group membership of a [`UserExtended`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Group {
    #[serde(rename = "colour")]
    pub color: Option<String>,
    pub description: Option<String>,
    /// Whether this group associates [`GameMode`]s with users' memberships.
    #[serde(rename = "has_playmodes")]
    pub has_modes: bool,
    pub id: u32,
    /// Unique string to identify the group.
    pub identifier: String,
    /// Whether members of this group are considered probationary.
    pub is_probationary: bool,
    /// [`GameMode`]s associated with this membership (`None` if `has_modes` is
    /// unset).
    #[serde(default, rename = "playmodes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<GameMode>>,
    pub name: String,
    /// Short name of the group for display.
    pub short_name: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "snake_case")]
pub enum HistoryType {
    Note,
    Restriction,
    TournamentBan,
    Silence,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Medal {
    pub description: String,
    pub grouping: String,
    pub icon_url: String,
    pub instructions: Option<String>,
    #[serde(rename = "id")]
    pub medal_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GameMode>,
    pub name: String,
    pub ordering: u32,
    pub slug: String,
}

impl PartialEq for Medal {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.medal_id == other.medal_id
    }
}

impl Eq for Medal {}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MedalCompact {
    #[serde(with = "serde_util::datetime")]
    pub achieved_at: OffsetDateTime,
    #[serde(rename = "achievement_id")]
    pub medal_id: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct MonthlyCount {
    #[serde(with = "serde_util::date")]
    pub start_date: Date,
    pub count: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ProfileBanner {
    pub id: u32,
    pub tournament_id: u32,
    pub image: String,
}

impl PartialEq for ProfileBanner {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.tournament_id == other.tournament_id
    }
}

impl Eq for ProfileBanner {}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub enum Playstyle {
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "keyboard")]
    Keyboard,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "touch")]
    Touch,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "snake_case")]
pub enum ProfilePage {
    Beatmaps,
    Historical,
    Kudosu,
    Me,
    Medals,
    RecentActivity,
    TopRanks,
}

/// Represents a User. Extends [`User`] object with additional attributes.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserExtended {
    /// url of user's avatar
    pub avatar_url: String,
    /// number of forum comments
    pub comments_count: usize,
    /// country of the user
    #[serde(deserialize_with = "deserialize_country")]
    pub country: String,
    /// two-letter code representing user's country
    pub country_code: CountryCode,
    /// urls for the profile cover
    pub cover: UserCover,
    /// Identifier of the default [`Group`] the user belongs to.
    #[serde(deserialize_with = "serde_util::from_option::deserialize")]
    pub default_group: String,
    /// discord tag, `None` if not specified by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    /// whether or not ever being a supporter in the past
    pub has_supported: bool,
    /// interests, `None` if not specified by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interests: Option<String>,
    /// has this account been active in the last x months?
    pub is_active: bool,
    /// is this a bot account?
    pub is_bot: bool,
    /// has this user been deleted?
    pub is_deleted: bool,
    /// is the user currently online? (either on lazer or the new website)
    pub is_online: bool,
    /// does this user have supporter?
    pub is_supporter: bool,
    /// date of account creation
    #[serde(with = "serde_util::datetime")]
    pub join_date: OffsetDateTime,
    /// current kudosu of the user
    pub kudosu: UserKudosu,
    /// last access time. `None` if the user hides online presence
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_util::option_datetime"
    )]
    pub last_visit: Option<OffsetDateTime>,
    /// location of the user, `None` if disabled by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// maximum number of users allowed to be blocked
    pub max_blocks: u32,
    /// maximum number of friends allowed to be added
    pub max_friends: u32,
    /// mode for this struct
    #[serde(rename = "playmode")]
    pub mode: GameMode,
    /// occupation, `None` if not specified by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    /// Device choices of the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playstyle: Option<Vec<Playstyle>>,
    /// whether or not the user allows PM from other than friends
    pub pm_friends_only: bool,
    /// number of forum posts
    #[serde(rename = "post_count")]
    pub forum_post_count: u32,
    /// colour of username/profile highlight, hex code (e.g. `"#333333"`)
    #[serde(
        default,
        rename = "profile_colour",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_color: Option<String>,
    /// ordered list of sections in user profile page
    pub profile_order: Vec<ProfilePage>,
    /// user-specific title
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// URL to the user title
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_url: Option<String>,
    /// twitter handle, `None` if not specified by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    /// unique identifier for user
    #[serde(rename = "id")]
    pub user_id: u32,
    /// user's display name
    pub username: Username,
    /// website, `None` if not specified by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_history: Option<Vec<AccountHistory>>,
    // pub active_tournament_banner: Option<ProfileBanner>, // TODO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<Badge>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beatmap_playcounts_count: Option<u32>,
    #[serde(
        default,
        rename = "favourite_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub favourite_mapset_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_count: Option<u32>,
    // friends: Option<>,
    #[serde(
        default,
        rename = "graveyard_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub graveyard_mapset_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(
        default,
        rename = "guest_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_mapset_count: Option<u32>,
    #[serde(
        default,
        rename = "rank_highest",
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_rank: Option<UserHighestRank>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bng: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_full_bn: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_gmt: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_limited_bn: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_moderator: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_nat: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_silenced: Option<bool>,
    #[serde(
        default,
        rename = "loved_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub loved_mapset_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping_follower_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_playcounts: Option<Vec<MonthlyCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<UserPage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_usernames: Option<Vec<Username>>,
    #[serde(
        default,
        deserialize_with = "rank_history_vec",
        skip_serializing_if = "Option::is_none"
    )]
    pub rank_history: Option<Vec<u32>>,
    /// Counts both ranked and approved mapsets
    #[serde(
        default,
        rename = "ranked_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub ranked_mapset_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replays_watched_counts: Option<Vec<MonthlyCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scores_best_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scores_first_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scores_recent_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<UserStatistics>,
    #[serde(default, alias = "statistics_rulesets")]
    pub statistics_modes: UserStatisticsModes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_level: Option<u8>,
    #[serde(
        default,
        rename = "pending_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_mapset_count: Option<u32>,
    #[serde(
        default,
        rename = "user_achievements",
        skip_serializing_if = "Option::is_none"
    )]
    pub medals: Option<Vec<MedalCompact>>,
}

/// Mainly used for embedding in certain responses to save additional api lookups.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct User {
    /// url of user's avatar
    pub avatar_url: String,
    /// two-letter code representing user's country
    pub country_code: CountryCode,
    /// Identifier of the default [`Group`] the user belongs to.
    #[serde(deserialize_with = "serde_util::from_option::deserialize")]
    pub default_group: String,
    /// has this account been active in the last x months?
    pub is_active: bool,
    /// is this a bot account?
    pub is_bot: bool,
    /// has this user been deleted?
    pub is_deleted: bool,
    /// is the user currently online? (either on lazer or the new website)
    pub is_online: bool,
    /// does this user have supporter?
    pub is_supporter: bool,
    /// last access time. `None` if the user hides online presence
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_util::option_datetime"
    )]
    pub last_visit: Option<OffsetDateTime>,
    /// whether or not the user allows PM from other than friends
    pub pm_friends_only: bool,
    /// colour of username/profile highlight, hex code (e.g. `"#333333"`)
    #[serde(
        default,
        rename = "profile_colour",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_color: Option<String>,
    /// unique identifier for user
    #[serde(rename = "id")]
    pub user_id: u32,
    /// user's display name
    pub username: Username,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_history: Option<Vec<AccountHistory>>,
    // pub active_tournament_banner: Option<ProfileBanner>, // TODO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<Badge>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beatmap_playcounts_count: Option<u32>,
    #[serde(
        default,
        deserialize_with = "deserialize_maybe_country",
        skip_serializing_if = "Option::is_none"
    )]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<UserCover>,
    #[serde(
        default,
        rename = "favourite_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub favourite_mapset_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_count: Option<u32>,
    // friends: Option<>,
    #[serde(
        default,
        rename = "graveyard_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub graveyard_mapset_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(
        default,
        rename = "guest_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_mapset_count: Option<u32>,
    #[serde(
        default,
        rename = "rank_highest",
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_rank: Option<UserHighestRank>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bng: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_full_bn: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_gmt: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_limited_bn: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_moderator: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_nat: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_silenced: Option<bool>,
    #[serde(
        default,
        rename = "loved_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub loved_mapset_count: Option<u32>,
    #[serde(
        default,
        rename = "user_achievements",
        skip_serializing_if = "Option::is_none"
    )]
    pub medals: Option<Vec<MedalCompact>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_playcounts: Option<Vec<MonthlyCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<UserPage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_usernames: Option<Vec<Username>>,
    #[serde(
        default,
        deserialize_with = "rank_history_vec",
        skip_serializing_if = "Option::is_none"
    )]
    pub rank_history: Option<Vec<u32>>,
    /// Counts both ranked and approved mapsets
    #[serde(
        default,
        rename = "ranked_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub ranked_mapset_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replays_watched_counts: Option<Vec<MonthlyCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scores_best_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scores_first_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scores_recent_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<UserStatistics>,
    #[serde(default, alias = "statistics_rulesets")]
    pub statistics_modes: UserStatisticsModes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_level: Option<u8>,
    #[serde(
        default,
        rename = "pending_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_mapset_count: Option<u32>,
}

impl From<UserExtended> for User {
    fn from(user: UserExtended) -> Self {
        Self {
            avatar_url: user.avatar_url,
            country_code: user.country_code,
            default_group: user.default_group,
            is_active: user.is_active,
            is_bot: user.is_bot,
            is_deleted: user.is_deleted,
            is_online: user.is_online,
            is_supporter: user.is_supporter,
            last_visit: user.last_visit,
            pm_friends_only: user.pm_friends_only,
            profile_color: user.profile_color,
            user_id: user.user_id,
            username: user.username,
            account_history: user.account_history,
            badges: user.badges,
            beatmap_playcounts_count: user.beatmap_playcounts_count,
            country: Some(user.country),
            cover: Some(user.cover),
            favourite_mapset_count: user.favourite_mapset_count,
            follower_count: user.follower_count,
            graveyard_mapset_count: user.graveyard_mapset_count,
            groups: user.groups,
            guest_mapset_count: user.guest_mapset_count,
            highest_rank: user.highest_rank,
            is_admin: user.is_admin,
            is_bng: user.is_bng,
            is_full_bn: user.is_full_bn,
            is_gmt: user.is_gmt,
            is_limited_bn: user.is_limited_bn,
            is_moderator: user.is_moderator,
            is_nat: user.is_nat,
            is_silenced: user.is_silenced,
            loved_mapset_count: user.loved_mapset_count,
            medals: user.medals,
            monthly_playcounts: user.monthly_playcounts,
            page: user.page,
            previous_usernames: user.previous_usernames,
            rank_history: user.rank_history,
            ranked_mapset_count: user.ranked_mapset_count,
            replays_watched_counts: user.replays_watched_counts,
            scores_best_count: user.scores_best_count,
            scores_first_count: user.scores_first_count,
            scores_recent_count: user.scores_recent_count,
            statistics: user.statistics,
            statistics_modes: UserStatisticsModes::default(),
            support_level: user.support_level,
            pending_mapset_count: user.pending_mapset_count,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Users {
    pub(crate) users: Vec<User>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserCover {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_url: Option<String>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserHighestRank {
    pub rank: u32,
    #[serde(with = "serde_util::datetime")]
    pub updated_at: OffsetDateTime,
}

/// Kudosu of a [`UserExtended`]
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserKudosu {
    /// Currently available kudosu
    pub available: i32,
    /// Total gained kudosu
    pub total: i32,
}

/// Level progression of a [`UserExtended`].
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserLevel {
    /// The current level
    pub current: u32,
    /// Percentage to the next level between `0.0` and `100.0`
    pub progress: u32,
}

impl UserLevel {
    /// Combine `self.current` and `self.progress` into a corresponding f32.
    ///
    /// # Example
    ///
    /// ```
    /// use rosu_v2::model::user::UserLevel;
    ///
    /// let level = UserLevel { current: 100, progress: 25 };
    /// assert_eq!(level.float(), 100.25);
    /// ```
    #[inline]
    pub fn float(&self) -> f32 {
        self.current as f32 + self.progress as f32 / 100.0
    }
}

/// osu! usernames are at most 15 ASCII characters long
pub type Username = SmallString<[u8; 15]>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserPage {
    pub html: String,
    pub raw: String,
}

/// A summary of various gameplay statistics for a [`UserExtended`]. Specific to a [`GameMode`]
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserStatistics {
    /// Hit accuracy percentage
    #[serde(rename = "hit_accuracy")]
    pub accuracy: f32,
    /// Total number of n300s
    pub count_300: u32,
    /// Total number of n100s
    pub count_100: u32,
    /// Total number of n50s
    pub count_50: u32,
    /// Total number of misses
    pub count_miss: u32,
    /// Current country rank according to pp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_rank: Option<u32>,
    /// Current global rank according to pp
    pub global_rank: Option<u32>,
    /// Counts of grades
    pub grade_counts: GradeCounts,
    /// Is actively ranked
    pub is_ranked: bool,
    /// The user's level progression
    pub level: UserLevel,
    /// Highest maximum combo
    #[serde(rename = "maximum_combo")]
    pub max_combo: u32,
    /// Number of maps played
    #[serde(rename = "play_count")]
    pub playcount: u32,
    /// Cumulative time played in seconds
    #[serde(rename = "play_time", deserialize_with = "maybe_u32")]
    pub playtime: u32,
    /// Performance points
    #[serde(deserialize_with = "deserialize_f32_default")]
    pub pp: f32,
    /// Current ranked score
    pub ranked_score: u64,
    /// Rank change in the last 30 days
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_change_since_30_days: Option<i32>,
    /// Number of replays watched by other users
    #[serde(rename = "replays_watched_by_others")]
    pub replays_watched: u32,
    /// Total number of hits
    pub total_hits: u64,
    /// Total score
    pub total_score: u64,
}

#[inline]
fn deserialize_f32_default<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
    <Option<f32> as Deserialize>::deserialize(d).map(Option::unwrap_or_default)
}

#[inline]
fn maybe_u32<'de, D: Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    <Option<u32> as Deserialize>::deserialize(d).map(Option::unwrap_or_default)
}

#[inline]
fn rank_history_vec<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u32>>, D::Error> {
    d.deserialize_option(RankHistoryVisitor)
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct UserStatisticsModes {
    pub osu: Option<UserStatistics>,
    pub taiko: Option<UserStatistics>,
    #[serde(alias = "fruits")]
    pub catch: Option<UserStatistics>,
    pub mania: Option<UserStatistics>,
}

struct RankHistoryVisitor;

impl<'de> Visitor<'de> for RankHistoryVisitor {
    type Value = Option<Vec<u32>>;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a map containing the field `data`, or a list of u32")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let capacity = seq.size_hint().unwrap_or(0);
        let mut rank_history_vec = Vec::with_capacity(capacity);

        while let Some(next) = seq.next_element()? {
            rank_history_vec.push(next);
        }

        Ok(Some(rank_history_vec))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut rank_history_vec: Option<Option<Vec<u32>>> = None;

        while let Some(key) = map.next_key::<&str>()? {
            if key == "data" && rank_history_vec.is_none() {
                rank_history_vec = Some(map.next_value()?);
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }

        rank_history_vec.ok_or_else(|| Error::missing_field("data"))
    }

    #[inline]
    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(RankHistoryVisitor)
    }

    #[inline]
    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        self.visit_unit()
    }

    #[inline]
    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}
