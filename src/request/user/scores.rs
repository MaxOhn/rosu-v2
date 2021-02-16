use crate::{
    error::ValueEnum,
    model::{GameMode, Score},
    request::{Pending, Request, UserId},
    routing::Route,
    Osu, OsuError, OsuResult,
};

use reqwest::multipart::Form;

/// Get scores of a user by the user's id.
///
/// [`crate::request::user::score::ScoreType`] **must** be specified before awaiting.
pub struct GetUserScores<'a> {
    fut: Option<Pending<'a, Vec<Score>>>,
    osu: &'a Osu,
    user_id: Option<UserId>,
    score_type: Option<&'static str>,
    limit: Option<u32>,
    offset: Option<u32>,
    include_fails: Option<bool>,
    mode: Option<GameMode>,
}

impl<'a> GetUserScores<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: impl Into<UserId>) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id.into()),
            score_type: None,
            limit: None,
            offset: None,
            include_fails: None,
            mode: None,
        }
    }

    #[inline]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit.replace(limit);

        self
    }

    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset.replace(offset);

        self
    }

    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    #[inline]
    pub fn include_fails(mut self, include_fails: bool) -> Self {
        self.include_fails.replace(include_fails);

        self
    }

    #[inline]
    pub fn best(mut self) -> Self {
        self.score_type.replace("best");

        self
    }

    #[inline]
    pub fn firsts(mut self) -> Self {
        self.score_type.replace("firsts");

        self
    }

    #[inline]
    pub fn recent(mut self) -> Self {
        self.score_type.replace("recent");

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let score_type = self.score_type.ok_or(OsuError::MissingParameter {
            param: ValueEnum::ScoreType,
        })?;

        let mut form = Form::new();

        if let Some(limit) = self.limit {
            form = form.text("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            form = form.text("offset", offset.to_string());
        }

        if let Some(mode) = self.mode {
            form = form.text("mode", mode.to_string());
        }

        let req = Request::from((
            form,
            Route::GetUserScores {
                user_id: self.user_id.take().unwrap(),
                score_type,
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetUserScores<'_>, Vec<Score>);
