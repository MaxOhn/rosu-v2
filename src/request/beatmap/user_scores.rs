use crate::{
    model::{BeatmapUserScore, GameMode, GameMods},
    request::{Pending, Request, UserId},
    routing::Route,
    Osu, OsuResult,
};

use reqwest::multipart::Form;

/// Get scores of a user on a beatmap by the user's and the map's id.
pub struct GetBeatmapUserScore<'a> {
    fut: Option<Pending<'a, BeatmapUserScore>>,
    osu: &'a Osu,
    map_id: u32,
    user_id: Option<UserId>,
    mode: Option<GameMode>,
    mods: Option<GameMods>,
    // TODO: limit & offset?
}

impl<'a> GetBeatmapUserScore<'a> {
    #[inline]
    pub(crate) fn new(osu: &'a Osu, map_id: u32, user_id: impl Into<UserId>) -> Self {
        Self {
            fut: None,
            osu,
            map_id,
            user_id: Some(user_id.into()),
            mode: None,
            mods: None,
        }
    }

    #[inline]
    pub fn mode(mut self, mode: GameMode) -> Self {
        self.mode.replace(mode);

        self
    }

    #[inline]
    pub fn mods(mut self, mods: GameMods) -> Self {
        self.mods.replace(mods);

        self
    }

    fn start(&mut self) -> OsuResult<()> {
        let mut form = Form::new();

        if let Some(mode) = self.mode {
            form = form.text("mode", mode.to_string());
        }

        if let Some(_mods) = self.mods {
            // TODO
        }

        let req = Request::from((
            form,
            Route::GetBeatmapUserScore {
                map_id: self.map_id,
                user_id: self.user_id.take().unwrap(),
            },
        ));

        self.fut.replace(Box::pin(self.osu.0.request(req)));

        Ok(())
    }
}

poll_req!(GetBeatmapUserScore<'_>, BeatmapUserScore);
