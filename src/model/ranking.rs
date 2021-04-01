use super::{
    beatmap::Beatmapset,
    user::{deserialize_country, UserStatistics},
    GameMode,
};
use crate::{Osu, OsuResult};

use chrono::{DateTime, Utc};
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChartRankings {
    #[serde(rename = "beatmapsets")]
    pub mapsets: Vec<Beatmapset>,
    pub ranking: Vec<UserStatistics>,
    pub spotlight: Spotlight,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CountryRanking {
    pub active_users: u32,
    #[serde(deserialize_with = "deserialize_country")]
    pub country: String,
    #[serde(rename = "code")]
    pub country_code: String,
    #[serde(rename = "play_count")]
    pub playcount: u64,
    #[serde(rename = "performance")]
    pub pp: f32,
    pub ranked_score: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CountryRankings {
    #[serde(
        default,
        rename = "cursor",
        deserialize_with = "deserialize_rankings_cursor",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page: Option<u32>,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    pub ranking: Vec<UserStatistics>,
    #[serde(default)]
    pub(crate) ranking_type: Option<RankingType>,
    pub total: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RankingType {
    Charts,
    Country,
    Performance,
    Score,
}

impl fmt::Display for RankingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self {
            Self::Charts => "charts",
            Self::Country => "country",
            Self::Performance => "performance",
            Self::Score => "score",
        };

        f.write_str(kind)
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
            RankingType::Charts | RankingType::Country => unreachable!(),
        };

        Some(rankings)
    }
}

struct RankingsCursorVisitor;

impl<'de> Visitor<'de> for RankingsCursorVisitor {
    type Value = Option<u32>;

    fn expecting(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        f.write_str("a u32, a map containing a `page` field, or null")
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Some(v as u32))
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(Self)
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut page = None;

        while let Some(key) = map.next_key()? {
            match key {
                "page" => {
                    page.replace(map.next_value()?);
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
