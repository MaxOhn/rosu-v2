use crate::{
    error::OsuError,
    model::{GameMode, Rankings},
    request::{Pending, Request},
    routing::Route,
    Osu, OsuResult,
};

use reqwest::multipart::Form;

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
    // TODO: Cursor
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

    fn start(&mut self) -> OsuResult<()> {
        let ranking_type = self.ranking_type.ok_or(OsuError::MissingParameter {
            param: "ranking type",
        })?;

        let mut form = Form::new();

        if let Some(country) = self.country.take() {
            form = form.text("country", country);
        }

        if let Some(variant) = self.variant {
            form = form.text("variant", variant);
        }

        if let Some(spotlight) = self.spotlight {
            form = form.text("spotlight", spotlight.to_string());
        }

        if let Some(filter) = self.filter {
            form = form.text("filter", filter);
        }

        let req = Request::from((
            form,
            Route::GetRankings {
                mode: self.mode,
                ranking_type,
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetRankings<'_>, Rankings);
