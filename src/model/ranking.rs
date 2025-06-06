use std::fmt;

use ::serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize,
};
use time::OffsetDateTime;

use crate::{model::user::CountryCode, Osu, OsuResult};

use super::{
    beatmap::BeatmapsetExtended,
    serde_util,
    user::{deserialize_country, Team, User, UserStatistics},
    CacheUserFn, ContainedUsers, GameMode,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ChartRankings {
    /// The list of beatmaps in the requested spotlight for the given mode
    #[serde(rename = "beatmapsets")]
    pub mapsets: Vec<BeatmapsetExtended>,
    #[serde(
        deserialize_with = "deserialize_user_stats_vec",
        serialize_with = "serialize_user_stats_vec"
    )]
    /// Score details ordered by score in descending order.
    pub ranking: Vec<User>,
    /// Spotlight details
    pub spotlight: Spotlight,
}

impl ContainedUsers for ChartRankings {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.mapsets.apply_to_users(f);
        self.ranking.apply_to_users(f);
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct CountryRanking {
    /// Active user count
    pub active_users: u32,
    /// Country name
    #[serde(deserialize_with = "deserialize_country")]
    pub country: String,
    #[serde(rename = "code")]
    pub country_code: CountryCode,
    /// Summed playcount for all users
    #[serde(rename = "play_count")]
    pub playcount: u64,
    /// Summed performance points for all users
    #[serde(rename = "performance")]
    pub pp: f32,
    /// Summed ranked score for all users
    pub ranked_score: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct CountryRankings {
    /// The next page of the ranking
    #[serde(
        default,
        rename = "cursor",
        deserialize_with = "deserialize_rankings_cursor",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page: Option<u32>,
    /// Country details ordered by pp in descending order.
    pub ranking: Vec<CountryRanking>,
    /// Total amount of countries
    pub total: u32,
}

impl CountryRankings {
    /// If `next_page` is `Some`, the API can provide the next set of countries and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu, mode: GameMode) -> Option<OsuResult<CountryRankings>> {
        Some(osu.country_rankings(mode).page(self.next_page?).await)
    }
}

impl ContainedUsers for CountryRankings {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Rankings {
    #[serde(default)]
    pub(crate) mode: Option<GameMode>,
    #[serde(
        default,
        rename = "cursor",
        deserialize_with = "deserialize_rankings_cursor",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page: Option<u32>,
    #[serde(
        deserialize_with = "deserialize_user_stats_vec",
        serialize_with = "serialize_user_stats_vec"
    )]
    pub ranking: Vec<User>,
    #[serde(default)]
    pub(crate) ranking_type: Option<RankingType>,
    pub total: u32,
}

impl ContainedUsers for Rankings {
    fn apply_to_users(&self, f: impl CacheUserFn) {
        self.ranking.apply_to_users(f);
    }
}

struct UserStatsVecVisitor;

impl<'de> Visitor<'de> for UserStatsVecVisitor {
    type Value = Vec<User>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a vec of UserStatistics structs")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut users = Vec::with_capacity(seq.size_hint().unwrap_or_default());

        while let Some(UserWrapper(user)) = seq.next_element()? {
            users.push(user);
        }

        Ok(users)
    }
}

struct UserWrapper(User);

impl<'de> Deserialize<'de> for UserWrapper {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(UserStatsVisitor).map(UserWrapper)
    }
}

struct UserStatsVisitor;

impl<'de> Visitor<'de> for UserStatsVisitor {
    type Value = User;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a UserStatistics struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut accuracy = None;
        let mut count_300 = None;
        let mut count_100 = None;
        let mut count_50 = None;
        let mut count_miss = None;
        let mut country_rank = None;
        let mut global_rank = None;
        let mut grade_counts = None;
        let mut is_ranked = None;
        let mut level = None;
        let mut max_combo = None;
        let mut playcount = None;
        let mut playtime = None;
        let mut pp = None;
        let mut ranked_score = None;
        let mut rank_change_since_30_days = None;
        let mut replays_watched = None;
        let mut total_hits = None;
        let mut total_score = None;

        let mut user = None;

        while let Some(key) = map.next_key()? {
            match key {
                "count_300" => count_300 = Some(map.next_value()?),
                "count_100" => count_100 = Some(map.next_value()?),
                "count_50" => count_50 = Some(map.next_value()?),
                "count_miss" => count_miss = Some(map.next_value()?),
                "hit_accuracy" => accuracy = Some(map.next_value()?),
                "country_rank" => country_rank = map.next_value()?,
                "global_rank" => global_rank = map.next_value()?,
                "grade_counts" => grade_counts = Some(map.next_value()?),
                "is_ranked" => is_ranked = Some(map.next_value()?),
                "level" => level = Some(map.next_value()?),
                "maximum_combo" => max_combo = Some(map.next_value()?),
                "play_count" => playcount = Some(map.next_value()?),
                "play_time" => {
                    playtime = Some(map.next_value::<Option<u32>>()?.unwrap_or_default());
                }
                "pp" => pp = Some(map.next_value::<Option<f32>>()?.unwrap_or_default()),
                "ranked_score" => ranked_score = Some(map.next_value()?),
                "rank_change_since_30_days" => rank_change_since_30_days = Some(map.next_value()?),
                "replays_watched_by_others" => replays_watched = Some(map.next_value()?),
                "total_hits" => total_hits = Some(map.next_value()?),
                "total_score" => total_score = Some(map.next_value()?),
                "user" => user = map.next_value()?,
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        let accuracy = accuracy.ok_or_else(|| Error::missing_field("hit_accuracy"))?;
        let count_300 = count_300.ok_or_else(|| Error::missing_field("count_300"))?;
        let count_100 = count_100.ok_or_else(|| Error::missing_field("count_100"))?;
        let count_50 = count_50.ok_or_else(|| Error::missing_field("count_50"))?;
        let count_miss = count_miss.ok_or_else(|| Error::missing_field("count_miss"))?;
        let grade_counts = grade_counts.ok_or_else(|| Error::missing_field("grade_counts"))?;
        let is_ranked = is_ranked.ok_or_else(|| Error::missing_field("is_ranked"))?;
        let level = level.ok_or_else(|| Error::missing_field("level"))?;
        let max_combo = max_combo.ok_or_else(|| Error::missing_field("maximum_combo"))?;
        let playcount = playcount.ok_or_else(|| Error::missing_field("play_count"))?;
        let playtime = playtime.ok_or_else(|| Error::missing_field("play_time"))?;
        let pp = pp.ok_or_else(|| Error::missing_field("pp"))?;
        let ranked_score = ranked_score.ok_or_else(|| Error::missing_field("ranked_score"))?;
        let rank_change_since_30_days = rank_change_since_30_days.unwrap_or_default();
        let replays_watched =
            replays_watched.ok_or_else(|| Error::missing_field("replays_watched_by_others"))?;
        let total_hits = total_hits.ok_or_else(|| Error::missing_field("total_hits"))?;
        let total_score = total_score.ok_or_else(|| Error::missing_field("total_score"))?;
        let mut user: User = user.ok_or_else(|| Error::missing_field("user"))?;

        let stats = UserStatistics {
            accuracy,
            count_300,
            count_100,
            count_50,
            count_miss,
            country_rank,
            global_rank,
            grade_counts,
            is_ranked,
            level,
            max_combo,
            playcount,
            playtime,
            pp,
            ranked_score,
            rank_change_since_30_days,
            replays_watched,
            total_hits,
            total_score,
        };

        user.statistics = Some(stats);

        Ok(user)
    }
}

fn deserialize_user_stats_vec<'de, D>(d: D) -> Result<Vec<User>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_seq(UserStatsVecVisitor)
}

#[cfg(feature = "serialize")]
struct UserCompactBorrowed<'u>(&'u User);

#[cfg(feature = "serialize")]
impl serde::Serialize for UserCompactBorrowed<'_> {
    fn serialize<S: serde::ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let user = self.0;

        let UserStatistics {
            accuracy,
            count_300,
            count_100,
            count_50,
            count_miss,
            country_rank,
            global_rank,
            grade_counts,
            is_ranked,
            level,
            max_combo,
            playcount,
            playtime,
            pp,
            ranked_score,
            rank_change_since_30_days,
            replays_watched,
            total_hits,
            total_score,
        } = user.statistics.as_ref().unwrap();

        let len = 17 + usize::from(country_rank.is_some()) + usize::from(global_rank.is_some());

        let mut s = s.serialize_struct("UserStatistics", len)?;
        s.serialize_field("hit_accuracy", accuracy)?;

        if let Some(rank) = country_rank {
            s.serialize_field("country_rank", rank)?;
        }

        if let Some(rank) = global_rank {
            s.serialize_field("global_rank", rank)?;
        }

        s.serialize_field("grade_counts", grade_counts)?;
        s.serialize_field("is_ranked", is_ranked)?;
        s.serialize_field("level", level)?;
        s.serialize_field("maximum_combo", max_combo)?;
        s.serialize_field("play_count", playcount)?;
        s.serialize_field("play_time", playtime)?;
        s.serialize_field("pp", pp)?;
        s.serialize_field("ranked_score", ranked_score)?;
        s.serialize_field("rank_change_since_30_days", rank_change_since_30_days)?;
        s.serialize_field("replays_watched_by_others", replays_watched)?;
        s.serialize_field("total_hits", total_hits)?;
        s.serialize_field("total_score", total_score)?;
        s.serialize_field("count_300", count_300)?;
        s.serialize_field("count_100", count_100)?;
        s.serialize_field("count_50", count_50)?;
        s.serialize_field("count_miss", count_miss)?;
        s.serialize_field("user", &UserWithoutStats::new(user))?;

        s.end()
    }
}

// Serializing a `User` reference without statistics
#[cfg(feature = "serialize")]
#[derive(serde::Serialize)]
struct UserWithoutStats<'u> {
    pub avatar_url: &'u String,
    pub country_code: &'u CountryCode,
    pub default_group: &'u String,
    pub is_active: &'u bool,
    pub is_bot: &'u bool,
    pub is_deleted: &'u bool,
    pub is_online: &'u bool,
    pub is_supporter: &'u bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_util::option_datetime"
    )]
    pub last_visit: &'u Option<OffsetDateTime>,
    pub pm_friends_only: &'u bool,
    #[serde(rename = "profile_colour", skip_serializing_if = "Option::is_none")]
    pub profile_color: &'u Option<String>,
    #[serde(rename = "id")]
    pub user_id: &'u u32,
    pub username: &'u crate::prelude::Username,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_history: &'u Option<Vec<crate::prelude::AccountHistory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges: &'u Option<Vec<crate::prelude::Badge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beatmap_playcounts_count: &'u Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: &'u Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: &'u Option<crate::prelude::UserCover>,
    #[serde(
        rename = "favourite_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub favourite_mapset_count: &'u Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_count: &'u Option<u32>,
    #[serde(
        rename = "graveyard_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub graveyard_mapset_count: &'u Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: &'u Option<Vec<crate::prelude::Group>>,
    #[serde(
        rename = "guest_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_mapset_count: &'u Option<u32>,
    #[serde(
        default,
        rename = "rank_highest",
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_rank: &'u Option<crate::prelude::UserHighestRank>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: &'u Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bng: &'u Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_full_bn: &'u Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_gmt: &'u Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_limited_bn: &'u Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moderator: &'u Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_nat: &'u Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_silenced: &'u Option<bool>,
    #[serde(
        rename = "loved_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub loved_mapset_count: &'u Option<u32>,
    #[serde(rename = "user_achievements", skip_serializing_if = "Option::is_none")]
    pub medals: &'u Option<Vec<crate::prelude::MedalCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_playcounts: &'u Option<Vec<crate::prelude::MonthlyCount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: &'u Option<crate::prelude::UserPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_usernames: &'u Option<Vec<crate::prelude::Username>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_history: &'u Option<Vec<u32>>,
    #[serde(
        rename = "ranked_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub ranked_mapset_count: &'u Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replays_watched_counts: &'u Option<Vec<crate::prelude::MonthlyCount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores_best_count: &'u Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores_first_count: &'u Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores_recent_count: &'u Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_level: &'u Option<u8>,
    #[serde(
        rename = "pending_beatmapset_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_mapset_count: &'u Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: &'u Option<crate::prelude::Team>,
}

#[cfg(feature = "serialize")]
impl<'u> UserWithoutStats<'u> {
    const fn new(user: &'u User) -> Self {
        let User {
            avatar_url,
            country_code,
            default_group,
            is_active,
            is_bot,
            is_deleted,
            is_online,
            is_supporter,
            last_visit,
            pm_friends_only,
            profile_color,
            user_id,
            username,
            account_history,
            badges,
            beatmap_playcounts_count,
            country,
            cover,
            favourite_mapset_count,
            follower_count,
            graveyard_mapset_count,
            groups,
            guest_mapset_count,
            highest_rank,
            is_admin,
            is_bng,
            is_full_bn,
            is_gmt,
            is_limited_bn,
            is_moderator,
            is_nat,
            is_silenced,
            loved_mapset_count,
            medals,
            monthly_playcounts,
            page,
            previous_usernames,
            rank_history,
            ranked_mapset_count,
            replays_watched_counts,
            scores_best_count,
            scores_first_count,
            scores_recent_count,
            statistics: _,
            statistics_modes: _,
            support_level,
            pending_mapset_count,
            team,
        } = user;

        Self {
            avatar_url,
            country_code,
            default_group,
            is_active,
            is_bot,
            is_deleted,
            is_online,
            is_supporter,
            last_visit,
            pm_friends_only,
            profile_color,
            user_id,
            username,
            account_history,
            badges,
            beatmap_playcounts_count,
            country,
            cover,
            favourite_mapset_count,
            follower_count,
            graveyard_mapset_count,
            groups,
            guest_mapset_count,
            highest_rank,
            is_admin,
            is_bng,
            is_full_bn,
            is_gmt,
            is_limited_bn,
            is_moderator,
            is_nat,
            is_silenced,
            loved_mapset_count,
            medals,
            monthly_playcounts,
            page,
            previous_usernames,
            rank_history,
            ranked_mapset_count,
            replays_watched_counts,
            scores_best_count,
            scores_first_count,
            scores_recent_count,
            support_level,
            pending_mapset_count,
            team,
        }
    }
}

#[cfg(feature = "serialize")]
fn serialize_user_stats_vec<S: serde::ser::Serializer>(
    users: &[User],
    s: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::SerializeSeq;

    let mut seq = s.serialize_seq(Some(users.len()))?;

    for user in users {
        seq.serialize_element(&UserCompactBorrowed(user))?;
    }

    seq.end()
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "lowercase")]
pub(crate) enum RankingType {
    Charts,
    Country,
    Performance,
    Score,
    Team,
}

impl RankingType {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Charts => "charts",
            Self::Country => "country",
            Self::Performance => "performance",
            Self::Score => "score",
            Self::Team => "team",
        }
    }
}

impl Rankings {
    /// If `next_page` is `Some`, the API can provide the next set of users and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<Rankings>> {
        let page = self.next_page?;
        let mode = self.mode?;
        let kind = self.ranking_type?;

        let rankings = match kind {
            RankingType::Performance => osu.performance_rankings(mode).page(page).await,
            RankingType::Score => osu.score_rankings(mode).page(page).await,
            RankingType::Charts | RankingType::Country | RankingType::Team => unreachable!(),
        };

        Some(rankings)
    }
}

struct RankingsCursorVisitor;

impl<'de> Visitor<'de> for RankingsCursorVisitor {
    type Value = Option<u32>;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("a u32, a map containing a `page` field, or null")
    }

    #[inline]
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Some(v as u32))
    }

    #[inline]
    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(Self)
    }

    #[inline]
    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut page = None;

        while let Some(key) = map.next_key()? {
            match key {
                "page" => {
                    page = Some(map.next_value()?);
                }
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        page.ok_or_else(|| Error::missing_field("page")).map(Some)
    }
}

fn deserialize_rankings_cursor<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u32>, D::Error> {
    d.deserialize_option(RankingsCursorVisitor)
}

/// The details of a spotlight.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Spotlight {
    /// The end date of the spotlight.
    #[serde(with = "serde_util::datetime")]
    pub end_date: OffsetDateTime,
    /// If the spotlight has different mades specific to each [`GameMode`].
    pub mode_specific: bool,
    /// The name of the spotlight.
    pub name: String,
    /// The number of users participating in this spotlight. This is only shown when viewing a single spotlight.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<u32>,
    /// The ID of this spotlight.
    #[serde(rename = "id")]
    pub spotlight_id: u32,
    /// The type of spotlight.
    #[serde(rename = "type")]
    pub spotlight_type: String,
    /// The starting date of the spotlight.
    #[serde(with = "serde_util::datetime")]
    pub start_date: OffsetDateTime,
}

impl ContainedUsers for Spotlight {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
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

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct TeamRankingsEntry {
    pub team_id: u32,
    #[serde(rename = "ruleset_id")]
    pub mode: GameMode,
    #[serde(rename = "play_count")]
    pub playcount: u64,
    pub ranked_score: u64,
    pub performance: f32,
    pub member_count: u32,
    pub team: Team,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct TeamRankings {
    pub ranking: Vec<TeamRankingsEntry>,
    /// Total amount of teams
    pub total: u32,
    #[serde(
        default,
        rename = "cursor",
        deserialize_with = "deserialize_rankings_cursor",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page: Option<u32>,
    #[serde(default)]
    pub(crate) mode: Option<GameMode>,
}

impl TeamRankings {
    /// If `next_page` is `Some`, the API can provide the next set of users and this method will request them.
    /// Otherwise, this method returns `None`.
    #[inline]
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<TeamRankings>> {
        let page = self.next_page?;
        let mode = self.mode?;

        Some(osu.team_rankings(mode).page(page).await)
    }
}

impl ContainedUsers for TeamRankings {
    fn apply_to_users(&self, _: impl CacheUserFn) {}
}
