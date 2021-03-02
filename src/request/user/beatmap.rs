use crate::{
    model::{Beatmapset, RankStatus},
    request::{Pending, Request, UserId},
    routing::Route,
    Osu, OsuError, OsuResult,
};

use reqwest::multipart::Form;

/// Get the beatmapsets of a user by their id.
///
/// The map type **must** be specified before awaiting, either manually through
/// [`map_type`](crate::request::GetUserBeatmapsets::map_type),
/// or through any of the methods [`loved`](crate::request::GetUserBeatmapsets::loved),
/// [`favourite`](crate::request::GetUserBeatmapsets::favourite),
/// [`graveyard`](crate::request::GetUserBeatmapsets::graveyard),
/// [`most_played`](crate::request::GetUserBeatmapsets::most_played),
/// [`ranked_and_approved`](crate::request::GetUserBeatmapsets::ranked_and_approved),
/// [`unranked`](crate::request::GetUserBeatmapsets::unranked).
pub struct GetUserBeatmapsets<'a> {
    fut: Option<Pending<'a, Vec<Beatmapset>>>,
    osu: &'a Osu,
    user_id: Option<UserId>,
    map_type: Option<&'static str>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl<'a> GetUserBeatmapsets<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, user_id: impl Into<UserId>) -> Self {
        Self {
            fut: None,
            osu,
            user_id: Some(user_id.into()),
            map_type: None,
            limit: None,
            offset: None,
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

    pub fn map_type(mut self, map_type: RankStatus) -> Self {
        let map_type = match map_type {
            RankStatus::Approved | RankStatus::Ranked => "ranked_and_approved",
            RankStatus::Graveyard => "graveyard",
            RankStatus::Pending | RankStatus::WIP | RankStatus::Qualified => "unranked",
            RankStatus::Loved => "loved",
        };

        self.map_type.replace(map_type);

        self
    }

    #[inline]
    pub fn loved(mut self) -> Self {
        self.map_type.replace("loved");

        self
    }

    #[inline]
    pub fn favourite(mut self) -> Self {
        self.map_type.replace("favourite");

        self
    }

    #[inline]
    pub fn graveyard(mut self) -> Self {
        self.map_type.replace("graveyard");

        self
    }

    #[inline]
    pub fn most_played(mut self) -> Self {
        self.map_type.replace("most_played");

        self
    }

    #[inline]
    pub fn ranked_and_approved(mut self) -> Self {
        self.map_type.replace("ranked_and_approved");

        self
    }

    #[inline]
    pub fn unranked(mut self) -> Self {
        self.map_type.replace("unranked");

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let map_type = self
            .map_type
            .ok_or(OsuError::MissingParameter { param: "map type" })?;

        let mut form = Form::new();

        if let Some(limit) = self.limit {
            form = form.text("limit", limit.to_string());
        }

        if let Some(offset) = self.offset {
            form = form.text("offset", offset.to_string());
        }

        let req = Request::from((
            form,
            Route::GetUserBeatmapsets {
                user_id: self.user_id.take().unwrap(),
                map_type,
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetUserBeatmapsets<'_>, Vec<Beatmapset>);
