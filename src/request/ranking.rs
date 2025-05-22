use crate::{
    model::{
        ranking::{ChartRankings, CountryRankings, RankingType, Rankings, Spotlight},
        user::CountryCode,
        DeserializedList, GameMode,
    },
    prelude::TeamRankings,
    request::{Query, Request},
    routing::Route,
    Osu,
};

use serde::Serialize;

/// Get a [`ChartRankings`] struct containing a [`Spotlight`], its
/// [`BeatmapsetExtended`]s, and participating [`User`]s.
///
/// The mapset will have their `maps` option filled.
///
/// The user statistics contain specific, spotlight related data.
/// All fields depends only on scores on maps of the spotlight.
/// The statistics vector is ordered by `ranked_score`.
/// The `user` option is filled.
///
/// [`BeatmapsetExtended`]: crate::model::beatmap::BeatmapsetExtended
/// [`User`]: crate::model::user::User
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetChartRankings<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    spotlight: Option<u32>,
}

impl<'a> GetChartRankings<'a> {
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            osu,
            mode,
            spotlight: None,
        }
    }

    /// Specify the spotlight id. If none is given,
    /// the latest spotlight will be returned.
    #[inline]
    pub const fn spotlight(mut self, spotlight_id: u32) -> Self {
        self.spotlight = Some(spotlight_id);

        self
    }
}

into_future! {
    |self: GetChartRankings<'_>| -> ChartRankings {
        Request::with_query(
            Route::GetRankings {
                mode: self.mode,
                ranking_type: RankingType::Charts,
            },
            Query::encode(&self),
        )
    }
}

/// Get a [`CountryRankings`] struct containing a vec of [`CountryRanking`]s
/// which will be sorted by the country's total pp.
///
/// [`CountryRanking`]: crate::model::ranking::CountryRanking
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetCountryRankings<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    #[serde(rename(serialize = "cursor[page]"))]
    page: Option<u32>,
}

impl<'a> GetCountryRankings<'a> {
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            osu,
            mode,
            page: None,
        }
    }

    /// Specify a page
    #[inline]
    pub const fn page(mut self, page: u32) -> Self {
        self.page = Some(page);

        self
    }
}

into_future! {
    |self: GetCountryRankings<'_>| -> CountryRankings {
        Request::with_query(
            Route::GetRankings {
                mode: self.mode,
                ranking_type: RankingType::Country,
            },
            Query::encode(&self),
        )
    }
}

/// Get a [`Rankings`] struct whose [`User`]s are sorted by their pp, i.e. the
/// current pp leaderboard.
///
/// [`User`]: crate::model::user::User
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetPerformanceRankings<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    country: Option<CountryCode>,
    variant: Option<&'static str>,
    #[serde(rename(serialize = "cursor[page]"))]
    page: Option<u32>,
}

impl<'a> GetPerformanceRankings<'a> {
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            osu,
            mode,
            country: None,
            variant: None,
            page: None,
        }
    }

    /// Specify a country code.
    #[inline]
    pub fn country(mut self, country: impl Into<CountryCode>) -> Self {
        self.country = Some(country.into());

        self
    }

    /// Consider only 4K scores. Only relevant for osu!mania.
    #[inline]
    pub const fn variant_4k(mut self) -> Self {
        self.variant = Some("4k");

        self
    }

    /// Consider only 7K scores. Only relevant for osu!mania.
    #[inline]
    pub const fn variant_7k(mut self) -> Self {
        self.variant = Some("7k");

        self
    }

    /// Pages range from 1 to 200.
    #[inline]
    pub const fn page(mut self, page: u32) -> Self {
        self.page = Some(page);

        self
    }
}

into_future! {
    |self: GetPerformanceRankings<'_>| -> Rankings {
        let req = Request::with_query(
            Route::GetRankings {
                mode: self.mode,
                ranking_type: RankingType::Performance,
            },
            Query::encode(&self),
        );

        (req, self.mode)
    } => |rankings, mode: GameMode| -> Rankings {
        rankings.mode = Some(mode);
        rankings.ranking_type = Some(RankingType::Performance);

        Ok(rankings)
    }
}

/// Get a [`Rankings`] struct whose [`User`]s are sorted by their ranked score,
/// i.e. the current ranked score leaderboard.
///
/// [`User`]: crate::model::user::User
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetScoreRankings<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    country: Option<CountryCode>,
    #[serde(rename(serialize = "cursor[page]"))]
    page: Option<u32>,
}

impl<'a> GetScoreRankings<'a> {
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            osu,
            mode,
            country: None,
            page: None,
        }
    }

    /// Specify a country code.
    #[inline]
    pub fn country(mut self, country: impl Into<CountryCode>) -> Self {
        self.country = Some(country.into());

        self
    }

    /// Pages range from 1 to 200.
    #[inline]
    pub const fn page(mut self, page: u32) -> Self {
        self.page = Some(page);

        self
    }
}

into_future! {
    |self: GetScoreRankings<'_>| -> Rankings {
        let req = Request::with_query(
            Route::GetRankings {
                mode: self.mode,
                ranking_type: RankingType::Score,
            },
            Query::encode(&self)
        );

        (req, self.mode)
    } => |rankings, mode: GameMode| -> Rankings {
        rankings.mode = Some(mode);
        rankings.ranking_type = Some(RankingType::Score);

        Ok(rankings)
    }
}

/// Get a vec of [`Spotlight`]s.
#[must_use = "requests must be configured and executed"]
pub struct GetSpotlights<'a> {
    osu: &'a Osu,
}

impl<'a> GetSpotlights<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self { osu }
    }
}

into_future! {
    |self: GetSpotlights<'_>| -> DeserializedList<Spotlight> {
        Request::new(Route::GetSpotlights)
    } => |spotlights, _| -> Vec<Spotlight> {
        Ok(spotlights.0)
    }
}

/// Get a [`TeamRankings`] struct whose entries are sorted by their pp.
#[must_use = "requests must be configured and executed"]
#[derive(Serialize)]
pub struct GetTeamRankings<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    #[serde(rename(serialize = "cursor[page]"))]
    page: Option<u32>,
}

impl<'a> GetTeamRankings<'a> {
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            osu,
            mode,
            page: None,
        }
    }

    /// Pages range from 1 to 200.
    #[inline]
    pub const fn page(mut self, page: u32) -> Self {
        self.page = Some(page);

        self
    }
}

into_future! {
    |self: GetTeamRankings<'_>| -> TeamRankings {
        let req = Request::with_query(
            Route::GetRankings {
                mode: self.mode,
                ranking_type: RankingType::Team,
            },
            Query::encode(&self),
        );

        (req, self.mode)
    } => |rankings, mode: GameMode| -> TeamRankings {
        rankings.mode = Some(mode);

        Ok(rankings)
    }
}
