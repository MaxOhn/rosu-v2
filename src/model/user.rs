use super::GameMode;

use chrono::{Date, DateTime, NaiveDate, Utc};
use serde::{
    de::{Error, IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;

pub fn str_to_date<'de, D: Deserializer<'de>>(d: D) -> Result<Date<Utc>, D::Error> {
    let date: NaiveDate = Deserialize::deserialize(d)?;

    Ok(Date::from_utc(date, Utc))
}

#[derive(Debug, Deserialize)]
pub struct AccountHistory {
    pub id: u32,
    #[serde(rename = "type")]
    pub history_type: HistoryType,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "length")]
    pub seconds: u32,
}

#[derive(Debug, Deserialize)]
pub struct Badge {
    pub awarded_at: DateTime<Utc>,
    pub description: String,
    pub image_url: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    code: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct GradeCounts {
    pub ss: i32,
    pub ssh: i32,
    pub s: i32,
    pub sh: i32,
    pub a: i32,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub id: u32,
    pub identifier: String,
    pub is_probationary: bool,
    pub name: String,
    pub short_name: String,
    pub description: String,
    #[serde(rename = "colour")]
    pub color: String,
    #[serde(rename = "playmodes")]
    pub modes: Option<Vec<GameMode>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HistoryType {
    Note,
    Restriction,
    Silence,
}

#[derive(Debug, Deserialize)]
pub struct MedalCompact {
    achieved_at: DateTime<Utc>,
    #[serde(rename = "achievement_id")]
    medal_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct MonthlyCount {
    #[serde(deserialize_with = "str_to_date")]
    pub start_date: Date<Utc>,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct ProfileBanner {
    pub id: u32,
    pub tounament_id: u32,
    pub image: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Playstyle {
    Mouse,
    Keyboard,
    Tablet,
    Touch,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub cover_url: String,
    pub country: Country,
    pub country_code: String,
    pub cover: UserCover,
    pub default_group: String,
    pub discord: Option<String>,
    pub has_supported: bool,
    pub interests: Option<String>,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub join_date: DateTime<Utc>,
    pub kudosu: UserKudosu,
    pub last_visit: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub max_blocks: u32,
    pub max_friends: u32,
    #[serde(rename = "playmode")]
    pub mode: GameMode,
    pub occupation: Option<String>,
    pub playstyle: Option<Vec<Playstyle>>,
    pub pm_friends_only: bool,
    #[serde(rename = "post_count")]
    pub forum_post_count: u32,
    #[serde(rename = "profile_colour")]
    pub profile_color: Option<String>,
    pub profile_order: Vec<ProfilePage>,
    pub skype: Option<String>,
    pub title: Option<String>,
    pub title_url: Option<String>,
    pub twitter: Option<String>,
    #[serde(rename = "id")]
    pub user_id: u32,
    pub username: String,
    pub website: Option<String>,

    pub account_history: Option<Vec<AccountHistory>>,
    // pub active_tournament_banner: Option<ProfileBanner>,
    pub badges: Option<Vec<Badge>>,
    pub beatmap_playcounts_count: Option<u32>,
    // blocks: Option<>,
    // current_mode_rank: Option<>,
    pub favourite_beatmapset_count: Option<u32>,
    pub follower_count: Option<u32>,
    // friends: Option<>,
    pub graveyard_beatmapset_count: Option<u32>,
    pub groups: Option<Vec<Group>>,
    pub is_admin: Option<bool>,
    pub is_bng: Option<bool>,
    pub is_full_bn: Option<bool>,
    pub is_gmt: Option<bool>,
    pub is_limited_bn: Option<bool>,
    pub is_moderator: Option<bool>,
    pub is_nat: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_silenced: Option<bool>,
    pub loved_beatmapset_count: Option<u32>,
    pub monthly_playcounts: Option<Vec<MonthlyCount>>,
    pub page: Option<UserPage>,
    pub previous_usernames: Option<Vec<String>>,
    #[serde(deserialize_with = "rank_history_vec")]
    pub rank_history: Option<Vec<u32>>,
    pub ranked_and_approved_beatmapset_count: Option<u32>,
    pub replays_watched_counts: Option<Vec<MonthlyCount>>,
    pub scores_best_count: Option<u32>,
    pub scores_first_count: Option<u32>,
    pub scores_recent_count: Option<u32>,
    pub statistics: Option<UserStatistics>,
    pub support_level: Option<u8>,
    pub unranked_beatmapset_count: Option<u32>,
    pub unread_pm_count: Option<u32>,
    #[serde(rename = "user_achievements")]
    pub medals: Option<Vec<MedalCompact>>,
    // user_preferences: Option<>,
}

#[derive(Debug, Deserialize)]
pub struct UserCompact {
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: String,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub last_visit: Option<DateTime<Utc>>,
    pub pm_friends_only: bool,
    #[serde(rename = "profile_colour")]
    pub profile_color: Option<String>,
    #[serde(rename = "id")]
    pub user_id: u32,
    pub username: String,

    pub account_history: Option<Vec<AccountHistory>>,
    // pub active_tournament_banner: Option<ProfileBanner>,
    pub badges: Option<Vec<Badge>>,
    pub beatmap_playcounts_count: Option<u32>,
    // blocks: Option<>,
    pub country: Option<Country>,
    pub cover: Option<UserCover>,
    // current_mode_rank: Option<>,
    pub favourite_beatmapset_count: Option<u32>,
    pub follower_count: Option<u32>,
    // friends: Option<>,
    pub graveyard_beatmapset_count: Option<u32>,
    pub groups: Option<Vec<Group>>,
    pub is_admin: Option<bool>,
    pub is_bng: Option<bool>,
    pub is_full_bn: Option<bool>,
    pub is_gmt: Option<bool>,
    pub is_limited_bn: Option<bool>,
    pub is_moderator: Option<bool>,
    pub is_nat: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_silenced: Option<bool>,
    pub loved_beatmapset_count: Option<u32>,
    pub monthly_playcounts: Option<Vec<MonthlyCount>>,
    pub page: Option<UserPage>,
    pub previous_usernames: Option<Vec<String>>,
    #[serde(deserialize_with = "rank_history_vec")]
    pub rank_history: Option<Vec<u32>>,
    pub ranked_and_approved_beatmapset_count: Option<u32>,
    pub replays_watched_counts: Option<Vec<MonthlyCount>>,
    pub scores_best_count: Option<u32>,
    pub scores_first_count: Option<u32>,
    pub scores_recent_count: Option<u32>,
    pub statistics: Option<UserStatistics>,
    pub support_level: Option<u8>,
    pub unranked_beatmapset_count: Option<u32>,
    pub unread_pm_count: Option<u32>,
    // #[serde(rename = "user_achievements")]
    // pub medals: Option<Vec<MedalCompact>>, // TODO
    // user_preferences: Option<>,
}

#[derive(Debug, Deserialize)]
pub struct UserCover {
    custom_url: Option<String>,
    url: String,
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserKudosu {
    pub available: i32,
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct UserLevel {
    pub current: u32,
    pub progress: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserPage {
    html: String,
    raw: String,
}

#[derive(Debug, Deserialize)]
pub struct UserRank {
    global: u32,
    country: u32,
}

#[derive(Debug, Deserialize)]
pub struct UserStatistics {
    pub grade_counts: GradeCounts,
    #[serde(rename = "hit_accuracy")]
    pub accuracy: f32,
    pub is_ranked: bool,
    pub level: UserLevel,
    #[serde(rename = "maximum_combo")]
    pub max_combo: u32,
    #[serde(rename = "play_count")]
    pub playcount: u32,
    #[serde(rename = "play_time")]
    pub playtime: u32,
    pub pp: f32,
    pub pp_rank: u32,
    pub rank: Option<UserRank>,
    pub ranked_score: u64,
    #[serde(rename = "replays_watched_by_others")]
    pub replays_watched: u32,
    pub total_hits: u64,
    pub total_score: u64,
    #[serde(default)]
    pub user: Option<Box<UserCompact>>,
}

pub fn rank_history_vec<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u32>>, D::Error> {
    d.deserialize_map(RankHistoryVisitor)
}

struct RankHistoryVisitor;

impl<'de> Visitor<'de> for RankHistoryVisitor {
    type Value = Option<Vec<u32>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map containing the field `data`, or a list of u32")
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
}
