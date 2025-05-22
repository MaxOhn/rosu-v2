use rosu_mods::GameMode;
use serde::Serialize;

use crate::{
    prelude::{ProcessedScores, Score},
    routing::Route,
    Osu,
};

use super::{serialize::maybe_mode_as_str, Query, Request};

/// Get a [`Score`] struct.
#[must_use = "requests must be configured and executed"]
#[derive(Clone)]
pub struct GetScore<'a> {
    osu: &'a Osu,
    mode: Option<GameMode>,
    score_id: u64,
    legacy_scores: bool,
}

impl<'a> GetScore<'a> {
    pub(crate) const fn new(osu: &'a Osu, score_id: u64) -> Self {
        Self {
            osu,
            mode: None,
            score_id,
            legacy_scores: false,
        }
    }

    /// Specify the mode
    #[inline]
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Specify whether the score should contain legacy data or not.
    ///
    /// Legacy data consists of a different grade calculation, less
    /// populated statistics, legacy mods, and a different score kind.
    #[inline]
    pub const fn legacy_scores(mut self, legacy_scores: bool) -> Self {
        self.legacy_scores = legacy_scores;

        self
    }
}

into_future! {
    |self: GetScore<'_>| -> Score {
        let route = Route::GetScore {
            mode: self.mode,
            score_id: self.score_id,
        };

        let mut req = Request::new(route);

        if self.legacy_scores {
            req.api_version(0);
        }

        req
    }
}

/// Get a list of recently processed [`Score`] structs.
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetScores<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(rename(serialize = "ruleset"), serialize_with = "maybe_mode_as_str")]
    mode: Option<GameMode>,
    #[serde(rename(serialize = "cursor[id]"))]
    score_id: Option<u64>,
    #[serde(rename(serialize = "cursor_string"))]
    cursor: Option<Box<str>>,
}

impl<'a> GetScores<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            osu,
            mode: None,
            score_id: None,
            cursor: None,
        }
    }

    /// Specify the mode
    pub const fn mode(mut self, mode: GameMode) -> Self {
        self.mode = Some(mode);

        self
    }

    /// Fetch from the given score id onward
    pub const fn score_id(mut self, score_id: u64) -> Self {
        self.score_id = Some(score_id);

        self
    }

    /// Specify a cursor
    pub fn cursor(mut self, cursor: Box<str>) -> Self {
        self.cursor = Some(cursor);

        self
    }
}

into_future! {
    |self: GetScores<'_>| -> ProcessedScores {
        (Request::with_query(Route::GetScores, Query::encode(&self)), self.mode)
    } => |scores, mode: Option<GameMode>| -> ProcessedScores {
        scores.mode = mode;

        Ok(scores)
    }
}
