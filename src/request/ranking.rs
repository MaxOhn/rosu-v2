use crate::{
    model::{
        ranking::{ChartRankings, CountryRankings, RankingType, Rankings, Spotlight},
        user::CountryCode,
        GameMode,
    },
    prelude::TeamRankings,
    request::{Pending, Query, Request},
    routing::Route,
    Osu,
};

use futures::future::TryFutureExt;
use serde::{Deserialize, Serialize};

/// Get a [`ChartRankings`](crate::model::ranking::ChartRankings) struct
/// containing a [`Spotlight`](crate::model::ranking::Spotlight), its
/// [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended)s, and participating
/// [`User`](crate::model::user::User).
///
/// The mapset will have their `maps` option filled.
///
/// The user statistics contain specific, spotlight related data.
/// All fields depends only on scores on maps of the spotlight.
/// The statistics vector is ordered by `ranked_score`.
/// The `user` option is filled.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetChartRankings<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, ChartRankings>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    spotlight: Option<u32>,
}

impl<'a> GetChartRankings<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
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
    pub const fn spotlight(mut self, spotlight_id: u32) -> Self {
        self.spotlight = Some(spotlight_id);

        self
    }

    fn start(&mut self) -> Pending<'a, ChartRankings> {
        let query = Query::encode(self);

        let route = Route::GetRankings {
            mode: self.mode,
            ranking_type: RankingType::Charts,
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;
        let fut = osu.request::<ChartRankings>(req);

        #[cfg(feature = "cache")]
        let fut = fut.inspect_ok(move |chart| {
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
#[derive(Serialize)]
pub struct GetCountryRankings<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, CountryRankings>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    #[serde(rename(serialize = "cursor[page]"))]
    page: Option<u32>,
}

impl<'a> GetCountryRankings<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            fut: None,
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

    fn start(&mut self) -> Pending<'a, CountryRankings> {
        let query = Query::encode(self);

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
/// [`User`](crate::model::user::User)s are sorted
/// by their pp, i.e. the current pp leaderboard.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetPerformanceRankings<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Rankings>>,
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
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
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

    fn start(&mut self) -> Pending<'a, Rankings> {
        let mode = self.mode;
        let query = Query::encode(self);

        let route = Route::GetRankings {
            mode,
            ranking_type: RankingType::Performance,
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;

        let fut = osu
            .request::<Rankings>(req)
            .map_ok(move |mut rankings: Rankings| {
                rankings.mode = Some(mode);
                rankings.ranking_type = Some(RankingType::Performance);

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
/// [`User`](crate::model::user::User)s are sorted
/// by their ranked score, i.e. the current ranked score leaderboard.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetScoreRankings<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, Rankings>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    #[serde(rename(serialize = "cursor[page]"))]
    page: Option<u32>,
}

impl<'a> GetScoreRankings<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            fut: None,
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

    fn start(&mut self) -> Pending<'a, Rankings> {
        let mode = self.mode;
        let query = Query::encode(self);

        let route = Route::GetRankings {
            mode,
            ranking_type: RankingType::Score,
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;

        let fut = osu
            .request::<Rankings>(req)
            .map_ok(move |mut rankings: Rankings| {
                rankings.mode = Some(mode);
                rankings.ranking_type = Some(RankingType::Score);

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

/// Get a [`TeamRankings`](crate::model::ranking::TeamRankings) struct whose
/// entriess are sorted by their pp.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Serialize)]
pub struct GetTeamRankings<'a> {
    #[serde(skip)]
    fut: Option<Pending<'a, TeamRankings>>,
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    mode: GameMode,
    #[serde(rename(serialize = "cursor[page]"))]
    page: Option<u32>,
}

impl<'a> GetTeamRankings<'a> {
    #[inline]
    pub(crate) const fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            fut: None,
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

    fn start(&mut self) -> Pending<'a, TeamRankings> {
        let mode = self.mode;
        let query = Query::encode(self);

        let route = Route::GetRankings {
            mode,
            ranking_type: RankingType::Team,
        };

        let req = Request::with_query(route, query);
        let osu = self.osu;

        let fut = osu
            .request::<TeamRankings>(req)
            .map_ok(move |mut rankings: TeamRankings| {
                rankings.mode = Some(mode);

                rankings
            });

        Box::pin(fut)
    }
}

poll_req!(GetTeamRankings => TeamRankings);
