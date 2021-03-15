use crate::{
    error::OsuError,
    model::{
        ranking::{Rankings, RankingsCursor, Spotlight},
        GameMode,
    },
    request::{Pending, Query, Request},
    routing::Route,
    Osu, OsuResult,
};

use futures::future::TryFutureExt;
use serde::Deserialize;

/// Get a [`Rankings`](crate::model::ranking::Rankings) struct.
///
/// Any of the `type_` methods **must** be specified before awaiting.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct GetRankings<'a> {
    fut: Option<Pending<'a, Rankings>>,
    osu: &'a Osu,
    mode: GameMode,
    filter: Option<&'static str>,
    ranking_type: Option<&'static str>,
    country: Option<String>,
    variant: Option<&'static str>,
    spotlight: Option<u32>,
    cursor: Option<RankingsCursor>,
}

impl<'a> GetRankings<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, mode: GameMode) -> Self {
        Self {
            fut: None,
            osu,
            mode,
            filter: None,
            ranking_type: None,
            country: None,
            variant: None,
            spotlight: None,
            cursor: None,
        }
    }

    #[inline]
    pub fn country(mut self, country: impl Into<String>) -> Self {
        if let None | Some("performance") = self.ranking_type {
            self.country.replace(country.into());
        }

        self
    }

    #[inline]
    pub fn filter_all(mut self) -> Self {
        self.filter.replace("all");

        self
    }

    #[inline]
    pub fn filter_friends(mut self) -> Self {
        self.filter.replace("friends");

        self
    }

    #[inline]
    pub fn spotlight(mut self, spotlight_id: u32) -> Self {
        if let None | Some("charts") = self.ranking_type {
            self.spotlight.replace(spotlight_id);
        }

        self
    }

    #[inline]
    pub fn type_charts(mut self) -> Self {
        self.ranking_type.replace("charts");

        self
    }

    #[inline]
    pub fn type_country(mut self) -> Self {
        self.ranking_type.replace("country");

        self
    }

    #[inline]
    pub fn type_performance(mut self) -> Self {
        self.ranking_type.replace("performance");

        self
    }

    #[inline]
    pub fn type_score(mut self) -> Self {
        self.ranking_type.replace("score");

        self
    }

    #[inline]
    pub fn variant_4k(mut self) -> Self {
        if self.mode == GameMode::MNA && matches!(self.ranking_type, None | Some("performance")) {
            self.variant.replace("4k");
        }

        self
    }

    #[inline]
    pub fn variant_7k(mut self) -> Self {
        if self.mode == GameMode::MNA && matches!(self.ranking_type, None | Some("performance")) {
            self.variant.replace("7k");
        }

        self
    }

    #[inline]
    pub fn cursor(mut self, cursor: RankingsCursor) -> Self {
        self.cursor.replace(cursor);

        self
    }

    fn start(&mut self) -> OsuResult<Pending<'a, Rankings>> {
        let ranking_type = self.ranking_type.ok_or(OsuError::MissingParameter {
            param: "ranking type",
        })?;

        let mut query = Query::new();

        if let Some(country) = self.country.take() {
            query.push("country", country);
        }

        if let Some(variant) = self.variant {
            query.push("variant", variant);
        }

        if let Some(spotlight) = self.spotlight {
            query.push("spotlight", spotlight.to_string());
        }

        if let Some(filter) = self.filter {
            query.push("filter", filter);
        }

        if let Some(cursor) = self.cursor {
            query.push("cursor[page]", cursor.page.to_string());
        }

        let req = Request::from((
            query,
            Route::GetRankings {
                mode: self.mode,
                ranking_type,
            },
        ));

        Ok(Box::pin(self.osu.inner.request(req)))
    }
}

poll_req!(GetRankings<'_> => OsuResult<Rankings>);

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
        let req = Request::from(Route::GetSpotlights);

        let fut = self
            .osu
            .inner
            .request(req)
            .map_ok(|s: Spotlights| s.spotlights);

        Box::pin(fut)
    }
}

poll_req!(GetSpotlights<'_> => Vec<Spotlight>);

#[derive(Deserialize)]
struct Spotlights {
    spotlights: Vec<Spotlight>,
}
