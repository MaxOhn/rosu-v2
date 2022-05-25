use crate::{
    model::{
        ranking_::{ChartRankings, CountryRankings, RankingType, Rankings, Spotlight},
        user_::CountryCode,
        GameMode,
    },
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

use futures::future::TryFutureExt;
use serde::Deserialize;

/// Get a [`ChartRankings`](crate::model::ranking::ChartRankings) struct
/// containing a [`Spotlight`](crate::model::ranking::Spotlight), its
/// [`Beatmapset`](crate::model::beatmap::Beatmapset)s, and participating
/// [`UserCompact`](crate::model::user::UserCompact).
///
/// The mapset will have their `maps` option filled.
///
/// The user statistics contain specific, spotlight related data.
/// All fields depends only on scores on maps of the spotlight.
/// The statistics vector is ordered by `ranked_score`.
/// The `user` option is filled.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetChartRankings<'a> {
    fut: Option<Pending<'a, ChartRankings>>,
    osu: &'a Osu,
    mode: GameMode,
    spotlight: Option<u32>,
}

impl<'a> GetChartRankings<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            fut: None,
            osu,
            mode,
            spotlight: None,
        }
    }

    /// Specify the spotlight id. If none is given,
    /// the latest spotlight will be returned.
    #[inline]
    pub fn spotlight(mut self, spotlight_id: u32) -> Self {
        self.spotlight.replace(spotlight_id);

        self
    }

    fn start(&mut self) -> Pending<'a, ChartRankings> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.chart_rankings.inc();

        let mut query = Query::new();

        if let Some(spotlight) = self.spotlight {
            query.push("spotlight", &spotlight);
        }

        let route = Route::GetRankings {
            mode: self.mode,
            ranking_type: RankingType::Charts,
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;
        let fut = osu.request::<ChartRankings>(req);

        #[cfg(feature = "cache")]
        let fut = fut.inspect_ok(move |chart| {
            for mapset in chart.mapsets.iter() {
                osu.update_cache(mapset.creator_id, &mapset.creator_name);
            }

            for user in chart.ranking.iter() {
                osu.update_cache(user.user_id, &user.username);
            }
        });

        Box::pin(fut)
    }
}

poll_req!(GetChartRankings => ChartRankings);

/// Get a [`CountryRankings`](crate::model::ranking::CountryRankings) struct
/// containing a vec of [`CountryRanking`](crate::model::ranking::CountryRanking)s
/// which will be sorted by the country's total pp.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetCountryRankings<'a> {
    fut: Option<Pending<'a, CountryRankings>>,
    osu: &'a Osu,
    mode: GameMode,
    page: Option<u32>,
}

impl<'a> GetCountryRankings<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            fut: None,
            osu,
            mode,
            page: None,
        }
    }

    /// Specify a page
    #[inline]
    pub fn page(mut self, page: u32) -> Self {
        self.page.replace(page);

        self
    }

    fn start(&mut self) -> Pending<'a, CountryRankings> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.country_rankings.inc();

        let mut query = Query::new();

        if let Some(page) = self.page {
            query.push("cursor[page]", &page);
        }

        let route = Route::GetRankings {
            mode: self.mode,
            ranking_type: RankingType::Country,
        };

        let req = Request::with_query(route, query);

        Box::pin(self.osu.request(req))
    }
}

poll_req!(GetCountryRankings => CountryRankings);

/// Get a [`Rankings`](crate::model::ranking::Rankings) struct whose
/// [`UserCompact`](crate::model::user::UserCompact)s are sorted
/// by their pp, i.e. the current pp leaderboard.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetPerformanceRankings<'a> {
    fut: Option<Pending<'a, Rankings>>,
    osu: &'a Osu,
    mode: GameMode,
    country: Option<CountryCode>,
    variant: Option<&'static str>,
    page: Option<u32>,
}

impl<'a> GetPerformanceRankings<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            fut: None,
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
        self.country.replace(country.into());

        self
    }

    /// Consider only 4K scores. Only relevant for osu!mania.
    #[inline]
    pub fn variant_4k(mut self) -> Self {
        self.variant.replace("4k");

        self
    }

    /// Consider only 7K scores. Only relevant for osu!mania.
    #[inline]
    pub fn variant_7k(mut self) -> Self {
        self.variant.replace("7k");

        self
    }

    /// Pages range from 1 to 200.
    #[inline]
    pub fn page(mut self, page: u32) -> Self {
        self.page.replace(page);

        self
    }

    fn start(&mut self) -> Pending<'a, Rankings> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.performance_rankings.inc();

        let mode = self.mode;
        let mut query = Query::new();

        if let Some(ref country) = self.country {
            query.push("country", country);
        }

        // ! Adjust filter once there are non-mania variants
        if let Some(variant) = self.variant.filter(|_| mode == GameMode::MNA) {
            query.push("variant", &variant);
        }

        if let Some(page) = self.page {
            query.push("cursor[page]", &page);
        }

        let route = Route::GetRankings {
            mode,
            ranking_type: RankingType::Performance,
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;

        let fut = osu
            .request::<Rankings>(req)
            .map_ok(move |mut rankings: Rankings| {
                rankings.mode.replace(mode);

                #[cfg(not(feature = "rkyv"))]
                rankings.ranking_type.replace(RankingType::Performance);

                #[cfg(feature = "cache")]
                for user in rankings.ranking.iter() {
                    osu.update_cache(user.user_id, &user.username);
                }

                rankings
            });

        Box::pin(fut)
    }
}

poll_req!(GetPerformanceRankings => Rankings);

/// Get a [`Rankings`](crate::model::ranking::Rankings) struct whose
/// [`UserCompact`](crate::model::user::UserCompact)s are sorted
/// by their ranked score, i.e. the current ranked score leaderboard.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetScoreRankings<'a> {
    fut: Option<Pending<'a, Rankings>>,
    osu: &'a Osu,
    mode: GameMode,
    page: Option<u32>,
}

impl<'a> GetScoreRankings<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            fut: None,
            osu,
            mode,
            page: None,
        }
    }

    /// Pages range from 1 to 200.
    #[inline]
    pub fn page(mut self, page: u32) -> Self {
        self.page.replace(page);

        self
    }

    fn start(&mut self) -> Pending<'a, Rankings> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.score_rankings.inc();

        let mode = self.mode;
        let mut query = Query::new();

        if let Some(page) = self.page {
            query.push("cursor[page]", &page);
        }

        let route = Route::GetRankings {
            mode,
            ranking_type: RankingType::Score,
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;

        let fut = osu
            .request::<Rankings>(req)
            .map_ok(move |mut rankings: Rankings| {
                rankings.mode.replace(mode);

                #[cfg(not(feature = "rkyv"))]
                rankings.ranking_type.replace(RankingType::Score);

                #[cfg(feature = "cache")]
                for user in rankings.ranking.iter() {
                    osu.update_cache(user.user_id, &user.username);
                }

                rankings
            });

        Box::pin(fut)
    }
}

poll_req!(GetScoreRankings => Rankings);

/// Get a vec of [`Spotlight`](crate::model::ranking::Spotlight)s.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetSpotlights<'a> {
    fut: Option<Pending<'a, Vec<Spotlight>>>,
    osu: &'a Osu,
}

impl<'a> GetSpotlights<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self { fut: None, osu }
    }

    fn start(&mut self) -> Pending<'a, Vec<Spotlight>> {
        #[cfg(feature = "metrics")]
        self.osu.metrics.spotlights.inc();

        let req = Request::new(Route::GetSpotlights);
        let fut = self.osu.request::<Spotlights>(req).map_ok(|s| s.spotlights);

        Box::pin(fut)
    }
}

poll_req!(GetSpotlights => Vec<Spotlight>);

#[derive(Deserialize)]
struct Spotlights {
    spotlights: Vec<Spotlight>,
}
