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

use serde::Deserialize;

/// Get the recent events of a user by their id.
///
/// Any of the `type_` methods **must** be specified before awaiting.
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

    fn start(&mut self) -> OsuResult<()> {
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

        self.fut.replace(Box::pin(self.osu.inner.request(req)));

        Ok(())
    }
}

poll_req!(GetRankings<'_> => OsuResult<Rankings>);

/// Get the list of spotlights
pub struct GetSpotlights<'a> {
    fut: Option<Pending<'a, Spotlights>>,
    osu: &'a Osu,
}

impl<'a> GetSpotlights<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu) -> Self {
        Self { fut: None, osu }
    }

    fn start(&mut self) {
        let req = Request::from(Route::GetSpotlights);
        self.fut.replace(Box::pin(self.osu.inner.request(req)));
    }
}

#[derive(Deserialize)]
struct Spotlights {
    spotlights: Vec<Spotlight>,
}

impl std::future::Future for GetSpotlights<'_> {
    type Output = crate::OsuResult<Vec<Spotlight>>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        loop {
            if let Some(fut) = self.as_mut().fut.as_mut() {
                return fut
                    .as_mut()
                    .poll(cx)
                    .map_ok(|spotlights| spotlights.spotlights);
            }

            self.as_mut().start();
        }
    }
}
